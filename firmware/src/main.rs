#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal::clock::MHz16;
use atmega_hal::delay::Delay;
use atmega_hal::pac;
use atmega_hal::port::{mode, Pin, Pins, PD2, PD3, PD4, PD5, PD6};
use atmega_hal::prelude::*;
use atmega_hal::usart::{Baudrate, Usart};
use core::panic::PanicInfo;

type Serial = Usart<pac::USART0, Pin<mode::Input, PD0>, Pin<mode::Output, PD1>, MHz16>;
type PD0 = atmega_hal::port::PD0;
type PD1 = atmega_hal::port::PD1;

// ICP Pin assignments (matching reference implementation)
// TDO - D2 (input)
// TMS - D3 (output)
// TDI - D4 (output)
// TCK - D5 (output)
// Power - D6 (output)

// Serial protocol commands
mod cmd {
    pub const CMD_PING: u8 = 0x01;
    pub const CMD_CONNECT: u8 = 0x02;
    pub const CMD_DISCONNECT: u8 = 0x03;
    pub const CMD_READ_FLASH: u8 = 0x04;
    pub const CMD_WRITE_FLASH: u8 = 0x05;
    pub const CMD_ERASE_FLASH: u8 = 0x06;
    pub const CMD_POWER_ON: u8 = 0x07;
    pub const CMD_POWER_OFF: u8 = 0x08;

    pub const RSP_OK: u8 = 0x00;
    pub const RSP_ERR: u8 = 0xFF;
    pub const RSP_DATA: u8 = 0x01;
}

// ICP Commands (from reference)
mod icp_cmd {
    pub const ICP_SET_IB_OFFSET_L: u8 = 0x40;
    pub const ICP_SET_IB_OFFSET_H: u8 = 0x41;
    pub const ICP_SET_IB_DATA: u8 = 0x42;
    pub const ICP_GET_IB_OFFSET: u8 = 0x43;
    pub const ICP_READ_FLASH: u8 = 0x44;
    pub const ICP_WRITE_FLASH: u8 = 0x45;
    pub const ICP_ERASE_FLASH: u8 = 0x46;
    pub const ICP_PING: u8 = 0x49;
    pub const ICP_SET_XPAGE: u8 = 0x4C;
}

#[derive(PartialEq)]
enum Mode {
    ERROR,
    READY,
    ICP,
    JTAG,
}

struct IcpPins {
    tdo: Pin<mode::Input<mode::PullUp>, PD2>,
    tms: Pin<mode::Output, PD3>,
    tdi: Pin<mode::Output, PD4>,
    tck: Pin<mode::Output, PD5>,
    power: Pin<mode::Output, PD6>,
}

struct IcpController {
    pins: IcpPins,
    delay: Delay<MHz16>,
    connected: bool,
    mode: Mode,
}

impl IcpController {
    fn new(pins: IcpPins) -> Self {
        Self {
            pins,
            delay: Delay::<MHz16>::new(),
            connected: false,
            mode: Mode::READY,
        }
    }

    fn init(&mut self) {
        self.pins.power.set_high();
        self.pins.tck.set_low();
        self.pins.tdi.set_low();
        self.pins.tms.set_low();
    }

    fn power_on(&mut self) {
        self.pins.power.set_low();
    }

    fn power_off(&mut self) {
        self.pins.power.set_high();
        self.connected = false;
    }

    fn delay_us(&mut self, us: u32) {
        self.delay.delay_us(us);
    }

    fn tck_high(&mut self) {
        self.pins.tck.set_high();
    }

    fn tck_low(&mut self) {
        self.pins.tck.set_low();
    }

    fn tdi_high(&mut self) {
        self.pins.tdi.set_high();
    }

    fn tdi_low(&mut self) {
        self.pins.tdi.set_low();
    }

    fn tms_high(&mut self) {
        self.pins.tms.set_high();
    }

    fn tms_low(&mut self) {
        self.pins.tms.set_low();
    }

    fn tdo_read(&self) -> bool {
        self.pins.tdo.is_high()
    }

    fn clock_pulse(&mut self) {
        self.delay_us(1);
        self.tck_high();
        self.delay_us(1);
        self.tck_low();
    }

    fn send_byte(&mut self, mut byte: u8) {
        // Send MSB first
        for _ in 0..8 {
            if byte & 0x80 != 0 {
                self.tdi_high();
            } else {
                self.tdi_low();
            }
            
            self.delay_us(1);
            self.tck_high();
            self.delay_us(1);
            self.tck_low();

            byte <<= 1;
        }
    }

    fn receive_byte(&mut self) -> u8 {
        let mut byte: u8 = 0;
        for _ in 0..8 {
            self.clock_pulse();

            if self.tdo_read() {
                byte |= 1;
            }

            byte <<= 1;
        }
        self.clock_pulse();
        byte
    }

    fn reverse_bits(mut byte: u8) -> u8 {
        let mut result: u8 = 0;
        for _ in 0..8 {
            result = (result << 1) | (byte & 1);
            byte >>= 1;
        }
        result
    }

    fn connect(&mut self) -> bool {
        // Initial setup: Set TCK, TDI, TMS high
        self.tck_high();
        self.tdi_high();
        self.tms_high();

        self.delay_us(500);

        self.tck_low();
        self.delay_us(1);
        self.tck_high();
        self.delay_us(50);

        // 165 TMS toggles
        for _ in 0..165 {
            self.tms_low();
            self.delay_us(2);
            self.tms_high();
            self.delay_us(2);
        }

        // 105 TDI toggles
        for _ in 0..105 {
            self.tdi_low();
            self.delay_us(2);
            self.tdi_high();
            self.delay_us(2);
        }

        // 90 TCK cycles
        for _ in 0..90 {
            self.tck_low();
            self.delay_us(2);
            self.tck_high();
            self.delay_us(2);
        }

        // 25600 TMS cycles
        for _ in 0..25600u16 {
            self.tms_low();
            self.delay_us(2);
            self.tms_high();
            self.delay_us(2);
        }

        self.delay_us(8);

        self.tms_low();

        // Enter ICP mode (send mode byte 150)
        self.mode = Mode::ICP;
        self.start_mode();

        // 25600 TCK cycles
        for _ in 0..25600u16 {
            self.tck_high();
            self.delay_us(2);
            self.tck_low();
            self.delay_us(2);
        }

        self.reset();

        // Verify connection with ping
        if self.check() {
            self.connected = true;
            true
        } else {
            self.connected = false;
            false
        }
        // self.check_2()
    }

    fn reset(&mut self) {
        // only implemented for ICP
        match self.mode {
            Mode::ERROR => { return; }
            Mode::ICP | Mode::READY => {
                self.tck_high();

                self.tms_high();
                self.delay_us(2);
                self.tms_low();
                self.delay_us(2);
            }
            Mode::JTAG => {
                // JTAG reset sequence (not implemented)
            }
        }

        self.mode = Mode::READY;
    }

    fn start_mode(&mut self) {
        self.tck_low();
        self.delay_us(2);

        let mut mode = match self.mode {
            Mode::ERROR => 0,
            Mode::READY => 1,
            Mode::ICP => 150,
            Mode::JTAG => 165,
        };

        // Send MSB first
        for _ in 0..8 {
            if mode & 0x80 != 0 {
                self.tdi_high();
            } else {
                self.tdi_low();
            }
            
            self.tck_high();
            self.delay_us(2);
            self.tck_low();
            self.delay_us(2);

            mode <<= 1;
        }

        self.tck_high();
        self.delay_us(2);
        self.tck_low();
        self.delay_us(2);

        self.tck_high();
        self.delay_us(2);
        self.tck_low();
        self.delay_us(2);
    }

    fn disconnect(&mut self) {
        // Reset to READY mode
        self.send_byte(1); // READY mode
        self.connected = false;
        self.tck_low();
        self.tdi_low();
        self.tms_low();
    }

    fn check(&mut self) -> bool {
        self.switch_mode(Mode::ICP);

        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_L);
        self.send_icp_byte(0x69);
        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_H);
        self.send_icp_byte(0xFF);

        self.send_icp_byte(icp_cmd::ICP_GET_IB_OFFSET);
        let response = self.receive_icp_byte();
        self.receive_icp_byte();

        response == 0x69
    }

    fn switch_mode(&mut self, mode: Mode) {
        if self.mode == mode {
            return;
        }

        if self.mode != Mode::READY {
            self.reset();
        }

        self.mode = mode;
        self.start_mode();

        if self.mode == Mode::ICP {
            self.delay_us(800);
            self.ping_icp();
        } else if self.mode == Mode::JTAG {
            // JTAG mode initialization (not implemented)
        } else {
            panic!("Invalid mode switch");
        }
    }

    fn ping_icp(&mut self) {
        if self.mode != Mode::ICP {
            return;
        }

        self.send_icp_byte(icp_cmd::ICP_PING);
        self.send_icp_byte(0xFF);
    }

    fn send_icp_byte(&mut self, mut byte: u8) {
        // Send MSB first
        for _ in 0..8 {
            if byte & 0x80 != 0 {
                self.tdi_high();
            } else {
                self.tdi_low();
            }
            
            self.delay_us(1);
            self.tck_high();
            self.delay_us(1);
            self.tck_low();

            byte <<= 1;
        }

        self.delay_us(1);
        self.tck_high();
        self.delay_us(1);
        self.tck_low();

        self.tdi_low();
    }

    fn receive_icp_byte(&mut self) -> u8 {
        let mut byte: u8 = 0;
        let mut mask: u8 = 1;
        for _ in 0..8 {
            self.delay_us(1);
            self.tck_high();
            self.delay_us(1);
            self.tck_low();

            if self.tdo_read() {
                byte |= mask;
            }

            mask <<= 1;
        }

        self.delay_us(1);
        self.tck_high();
        self.delay_us(1);
        self.tck_low();

        byte
    }

    fn set_address(&mut self, addr: u32) {
        // Set low byte
        self.send_byte(icp_cmd::ICP_SET_IB_OFFSET_L);
        self.send_byte(addr as u8);

        // Set high byte
        self.send_byte(icp_cmd::ICP_SET_IB_OFFSET_H);
        self.send_byte((addr >> 8) as u8);

        // Set extended page if needed
        if addr > 0xFFFF {
            self.send_byte(icp_cmd::ICP_SET_XPAGE);
            self.send_byte((addr >> 16) as u8);
        }
    }

    fn read_flash(&mut self, addr: u32, buffer: &mut [u8]) -> bool {
        if !self.connected {
            return false;
        }

        self.set_address(addr);
        self.send_byte(icp_cmd::ICP_READ_FLASH);

        for byte in buffer.iter_mut() {
            *byte = Self::reverse_bits(self.receive_byte());
        }

        // Reset address
        self.send_byte(icp_cmd::ICP_SET_IB_OFFSET_L);
        self.send_byte(0);
        self.send_byte(icp_cmd::ICP_SET_IB_OFFSET_H);
        self.send_byte(0);

        true
    }

    fn write_flash(&mut self, addr: u32, data: &[u8]) -> bool {
        if !self.connected || data.is_empty() {
            return false;
        }

        self.set_address(addr);

        // Send first byte
        self.send_byte(icp_cmd::ICP_SET_IB_DATA);
        self.send_byte(data[0]);

        // Programming sequence
        self.send_byte(icp_cmd::ICP_WRITE_FLASH);
        self.send_byte(0x6E);
        self.send_byte(0x15);
        self.send_byte(0x0A);
        self.send_byte(0x09);
        self.send_byte(0x06);

        // Send remaining bytes
        for &byte in &data[1..] {
            self.send_byte(icp_cmd::ICP_SET_IB_DATA);
            self.send_byte(byte);
            self.delay_us(5);
        }

        true
    }

    fn erase_flash(&mut self, addr: u32) -> bool {
        if !self.connected {
            return false;
        }

        self.set_address(addr);

        // Set data to 0
        self.send_byte(icp_cmd::ICP_SET_IB_DATA);
        self.send_byte(0x00);

        // Erase sequence
        self.send_byte(icp_cmd::ICP_ERASE_FLASH);
        self.send_byte(0xE6);
        self.send_byte(0x15);
        self.send_byte(0x0A);
        self.send_byte(0x09);
        self.send_byte(0x06);

        // Wait for erase to complete (300ms typical)
        self.delay.delay_ms(300u16);

        // Check status
        let status = self.receive_byte();
        (status & 0x01) == 0
    }
}

fn read_u16(serial: &mut Serial) -> u16 {
    let low = nb::block!(serial.read()).unwrap_or(0);
    let high = nb::block!(serial.read()).unwrap_or(0);
    u16::from_le_bytes([low, high])
}

fn read_u32(serial: &mut Serial) -> u32 {
    let b0 = nb::block!(serial.read()).unwrap_or(0);
    let b1 = nb::block!(serial.read()).unwrap_or(0);
    let b2 = nb::block!(serial.read()).unwrap_or(0);
    let b3 = nb::block!(serial.read()).unwrap_or(0);
    u32::from_le_bytes([b0, b1, b2, b3])
}

fn send_response(serial: &mut Serial, code: u8) {
    let _ = nb::block!(serial.write(code));
}

fn send_data(serial: &mut Serial, data: &[u8]) {
    let _ = nb::block!(serial.write(cmd::RSP_DATA));
    let len = data.len() as u16;
    let _ = nb::block!(serial.write(len as u8));
    let _ = nb::block!(serial.write((len >> 8) as u8));
    for &byte in data {
        let _ = nb::block!(serial.write(byte));
    }
}

#[atmega_hal::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Setup serial at 115200 baud
    let serial = Usart::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        Baudrate::<MHz16>::new(115200),
    );

    let (mut rx, mut tx) = serial.split();

    // Setup ICP pins
    let icp_pins = IcpPins {
        tdo: pins.pd2.into_pull_up_input(),
        tms: pins.pd3.into_output(),
        tdi: pins.pd4.into_output(),
        tck: pins.pd5.into_output(),
        power: pins.pd6.into_output(),
    };

    let mut icp = IcpController::new(icp_pins);

    icp.init();

    // Buffer for flash operations (limited by AVR RAM)
    let mut buffer: [u8; 64] = [0; 64];

    loop {
        // Wait for command
        let Ok(cmd_byte) = nb::block!(rx.read());
        match cmd_byte {
            cmd::CMD_PING => {
                // Simple ping response
                let _ = nb::block!(tx.write(cmd::RSP_OK));
                let _ = nb::block!(tx.write(b'S'));
                let _ = nb::block!(tx.write(b'W'));
            }

            cmd::CMD_POWER_ON => {
                icp.power_on();
                let _ = nb::block!(tx.write(cmd::RSP_OK));
            }

            cmd::CMD_POWER_OFF => {
                icp.power_off();
                let _ = nb::block!(tx.write(cmd::RSP_OK));
            }

            cmd::CMD_CONNECT => {
                if icp.connect() {
                    let _ = nb::block!(tx.write(cmd::RSP_OK));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }

            cmd::CMD_DISCONNECT => {
                icp.disconnect();
                let _ = nb::block!(tx.write(cmd::RSP_OK));
            }

            cmd::CMD_READ_FLASH => {
                // Read address (4 bytes) and length (2 bytes)
                let addr = {
                    let b0 = nb::block!(rx.read()).unwrap_or(0);
                    let b1 = nb::block!(rx.read()).unwrap_or(0);
                    let b2 = nb::block!(rx.read()).unwrap_or(0);
                    let b3 = nb::block!(rx.read()).unwrap_or(0);
                    u32::from_le_bytes([b0, b1, b2, b3])
                };
                let len = {
                    let low = nb::block!(rx.read()).unwrap_or(0);
                    let high = nb::block!(rx.read()).unwrap_or(0);
                    u16::from_le_bytes([low, high]) as usize
                };

                // Clamp length to buffer size
                let read_len = len.min(buffer.len());

                if icp.read_flash(addr, &mut buffer[..read_len]) {
                    let _ = nb::block!(tx.write(cmd::RSP_DATA));
                    let _ = nb::block!(tx.write(read_len as u8));
                    let _ = nb::block!(tx.write((read_len >> 8) as u8));
                    for i in 0..read_len {
                        let _ = nb::block!(tx.write(buffer[i]));
                    }
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }

            cmd::CMD_WRITE_FLASH => {
                // Read address (4 bytes) and length (2 bytes)
                let addr = {
                    let b0 = nb::block!(rx.read()).unwrap_or(0);
                    let b1 = nb::block!(rx.read()).unwrap_or(0);
                    let b2 = nb::block!(rx.read()).unwrap_or(0);
                    let b3 = nb::block!(rx.read()).unwrap_or(0);
                    u32::from_le_bytes([b0, b1, b2, b3])
                };
                let len = {
                    let low = nb::block!(rx.read()).unwrap_or(0);
                    let high = nb::block!(rx.read()).unwrap_or(0);
                    u16::from_le_bytes([low, high]) as usize
                };

                // Clamp length to buffer size
                let write_len = len.min(buffer.len());

                // Read data into buffer
                for i in 0..write_len {
                    buffer[i] = nb::block!(rx.read()).unwrap_or(0);
                }

                if icp.write_flash(addr, &buffer[..write_len]) {
                    let _ = nb::block!(tx.write(cmd::RSP_OK));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }

            cmd::CMD_ERASE_FLASH => {
                // Read address (4 bytes)
                let addr = {
                    let b0 = nb::block!(rx.read()).unwrap_or(0);
                    let b1 = nb::block!(rx.read()).unwrap_or(0);
                    let b2 = nb::block!(rx.read()).unwrap_or(0);
                    let b3 = nb::block!(rx.read()).unwrap_or(0);
                    u32::from_le_bytes([b0, b1, b2, b3])
                };

                if icp.erase_flash(addr) {
                    let _ = nb::block!(tx.write(cmd::RSP_OK));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }

            _ => {
                // Unknown command
                let _ = nb::block!(tx.write(cmd::RSP_ERR));
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    // Setup serial at 115200 baud
    let serial = Usart::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        Baudrate::<MHz16>::new(115200),
    );

    let (mut _rx, mut tx) = serial.split();

    loop {
        let _ = nb::block!(tx.write(b'S'));
    }
}
