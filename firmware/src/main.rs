#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::{ops, panic::PanicInfo};

use atmega_hal::{
    clock::MHz16,
    delay::Delay,
    pac,
    port::{mode, Pin, Pins, PD2, PD3, PD4, PD5, PD6},
    prelude::*,
    usart::{Baudrate, Usart},
};

// ICP Pin assignments (matching reference implementation)
// TDO - D2 (input)
// TMS - D3 (output)
// TDI - D4 (output)
// TCK - D5 (output)
// Power - D6 (output)

// Firmware version
const VERSION_MAJOR: u8 = 2;
const VERSION_MINOR: u8 = 0;

// Serial protocol commands
mod cmd {
    // System commands
    pub const CMD_PING: u8 = 0x01;
    pub const CMD_GET_VERSION: u8 = 0x02;

    // Connection
    pub const CMD_CONNECT: u8 = 0x03;
    pub const CMD_DISCONNECT: u8 = 0x04;

    // Identification
    pub const CMD_GET_ID: u8 = 0x05;
    // Configuration
    pub const CMD_SET_CONFIG: u8 = 0x06;
    pub const CMD_GET_CONFIG: u8 = 0x07;

    // Memory operations
    pub const CMD_READ_FLASH: u8 = 0x08;
    pub const CMD_WRITE_FLASH: u8 = 0x09;
    pub const CMD_ERASE_FLASH_SECTOR: u8 = 0x0A;
    pub const CMD_MASS_ERASE: u8 = 0x0B;
    pub const CMD_READ_CUSTOM_REGION: u8 = 0x0C;
    pub const CMD_WRITE_CUSTOM_REGION: u8 = 0x0D;

    // Response codes
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
    pub const ICP_PING: u8 = 0x49;
    pub const ICP_READ_CUSTOM_BLOCK: u8 = 0x4A;
    pub const ICP_SET_XPAGE: u8 = 0x4C;
}

mod jtag_instructions {
    pub const JTAG_IDCODE: u8 = 14;
}

#[derive(PartialEq)]
enum Mode {
    UNSET,
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
    chip_type: Option<u8>,
}

impl IcpController {
    fn new(pins: IcpPins) -> Self {
        Self {
            pins,
            delay: Delay::<MHz16>::new(),
            connected: false,
            mode: Mode::UNSET,
            chip_type: None,
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

    fn set_chip_type(&mut self, chip_type: u8) {
        self.chip_type = Some(chip_type);
    }

    fn connect(&mut self) -> bool {
        self.power_on();

        // Wait for power stabilization
        self.delay.delay_ms(5u8);

        // Initial setup: Set TCK, TDI, TMS high
        self.tms_high();
        self.tdi_high();
        self.tck_high();

        self.delay.delay_ms(3u8);

        self.tck_low();
        self.delay_us(2);
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

        // 20480 TMS cycles
        for _ in 0..20480u16 {
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

        // 1644 TCK cycles
        for _ in 0..1644u16 {
            self.tck_high();
            self.delay_us(2);
            self.tck_low();
            self.delay_us(2);
        }

        self.reset();

        self.delay_us(10);

        // Verify connection with ping
        if self.check() {
            self.connected = true;
            true
        } else {
            self.connected = false;
            false
        }
    }

    fn reset(&mut self) {
        // only implemented for ICP
        match self.mode {
            Mode::UNSET => {
                return;
            }
            Mode::ICP | Mode::READY => {
                self.tck_high();

                self.delay_us(8);

                self.tms_high();
                self.delay_us(2);
                self.tms_low();
                self.delay_us(2);
            }
            Mode::JTAG => {
                for _ in 0..35 {
                    self.jtag_next_state(true);
                }
                self.pins.tck.set_high();
                self.pins.tms.set_low();
            }
        }

        self.mode = Mode::READY;
    }

    fn start_mode(&mut self) {
        self.tck_low();
        self.delay_us(2);

        let mut mode: u8 = match self.mode {
            Mode::UNSET => 0,
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
        self.power_off();
    }

    fn check(&mut self) -> bool {
        self.switch_mode(Mode::ICP);

        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_L);
        self.send_icp_byte(0x69);
        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_H);
        self.send_icp_byte(0xFF);
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
            // reset JTAG state
            for _ in 0..8 {
                self.jtag_next_state(true);
            }

            self.jtag_send_instruction(2);
            self.jtag_send_data(4, 4u8);

            self.jtag_send_instruction(3);
            self.jtag_send_data(23, 0x403000u32);
            self.delay_us(50);
            self.jtag_send_data(23, 0x402000u32);
            self.jtag_send_data(23, 0x400000u32);

            // most likely breakpoints initialization
            // SH68F881W works without it, but maybe for other chips it's mandatory
            {
                self.jtag_send_data(23, 0x630000u32);
                self.jtag_send_data(23, 0x670000u32);
                self.jtag_send_data(23, 0x6B0000u32);
                self.jtag_send_data(23, 0x6F0000u32);
                self.jtag_send_data(23, 0x730000u32);
                self.jtag_send_data(23, 0x770000u32);
                self.jtag_send_data(23, 0x7B0000u32);
                self.jtag_send_data(23, 0x7F0000u32);
            }

            self.jtag_send_instruction(2);
            self.jtag_send_data(4, 1u8);

            self.jtag_send_instruction(12);
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

            self.delay_us(2);
            self.tck_high();
            self.delay_us(2);
            self.tck_low();

            byte <<= 1;
        }

        self.delay_us(2);
        self.tck_high();
        self.delay_us(2);
        self.tck_low();

        self.tdi_low();
    }

    fn receive_icp_byte(&mut self) -> u8 {
        let mut byte: u8 = 0;
        let mut mask: u8 = 1;
        for _ in 0..8 {
            self.delay_us(2);
            self.tck_high();
            self.delay_us(2);
            self.tck_low();

            if self.tdo_read() {
                byte |= mask;
            }

            mask <<= 1;
        }

        self.delay_us(2);
        self.tck_high();
        self.delay_us(2);
        self.tck_low();

        byte
    }

    fn jtag_get_id(&mut self) -> u16 {
        self.switch_mode(Mode::JTAG);

        self.jtag_send_instruction(jtag_instructions::JTAG_IDCODE);
        self.jtag_receive_data(16)
    }

    fn jtag_send_instruction(&mut self, instruction: u8) {
        self.jtag_next_state(false); // Idle
        self.jtag_next_state(true); // Select-DR
        self.jtag_next_state(true); // Select-IR
        self.jtag_next_state(false); // Capture-IR
        self.jtag_next_state(false); // Shift-IR
        self.jtag_send_bits(4, instruction);
        self.jtag_next_state(true); // Update-IR
        self.jtag_next_state(false); // Idle
    }

    fn jtag_receive_data<T>(&mut self, bit_length: u8) -> T
    where
        T: Copy + From<u8> + ops::Shl<u8, Output = T> + ops::BitOrAssign<T>,
    {
        self.jtag_next_state(true); // Select-DR
        self.jtag_next_state(false); // Capture-DR
        self.jtag_next_state(false); // Shift-DR
        let data: T = self.jtag_receive_bits(bit_length);
        self.jtag_next_state(true); // Update-DR
        self.jtag_next_state(false); // Idle
        data
    }

    fn jtag_send_data<T>(&mut self, bit_length: u8, data: T)
    where
        T: Copy + From<u8> + ops::BitAnd<Output = T> + ops::Shr<u8, Output = T> + PartialEq,
    {
        self.jtag_next_state(true); // Select-DR
        self.jtag_next_state(false); // Capture-DR
        self.jtag_next_state(false); // Shift-DR
        self.jtag_send_bits(bit_length, data);
        self.jtag_next_state(true); // Update-DR
        self.jtag_next_state(false); // Idle
        self.jtag_next_state(false); // Idle? Needed, don't know why
    }

    fn jtag_next_state(&mut self, tms: bool) -> bool {
        if tms {
            self.tms_high();
        } else {
            self.tms_low();
        }

        self.pins.tck.set_high();
        self.delay_us(2);

        let b = self.pins.tdo.is_high();
        self.pins.tck.set_low();
        self.delay_us(2);

        return b;
    }

    fn jtag_next_state_out(&mut self, tms: bool, out: bool) -> bool {
        if out {
            self.tdi_high();
        } else {
            self.tdi_low();
        }

        return self.jtag_next_state(tms);
    }

    fn jtag_send_bits<T>(&mut self, bit_length: u8, value: T)
    where
        T: Copy + From<u8> + ops::BitAnd<Output = T> + ops::Shr<u8, Output = T> + PartialEq,
    {
        for i in 0..bit_length {
            let bit = (value >> i) & T::from(1);
            let last_bit = i == (bit_length - 1);
            self.jtag_next_state_out(last_bit, bit != T::from(0));
        }

        self.pins.tdi.set_low();
    }

    fn jtag_receive_bits<T>(&mut self, bit_length: u8) -> T
    where
        T: Copy + From<u8> + ops::Shl<u8, Output = T> + ops::BitOrAssign<T>,
    {
        let mut value: T = 0.into();
        for i in 0..bit_length {
            let last_bit = i == (bit_length - 1);
            let bit = self.jtag_next_state(last_bit);
            if bit {
                value |= T::from(1) << (bit_length - 1 - i);
            }
        }

        return value;
    }

    fn icp_read_flash(&mut self, addr: u32, buffer: &mut [u8], custom_block: bool) -> bool {
        self.switch_mode(Mode::ICP);

        let Some(chip_type) = self.chip_type else {
            return false;
        };

        if chip_type != 1 {
            self.send_icp_byte(0x46);
            self.send_icp_byte(0xFE);
            self.send_icp_byte(0xFF);
        }

        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_L);
        self.send_icp_byte((addr & 0xFF) as u8);
        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_H);
        self.send_icp_byte(((addr & 0xFF00) >> 8) as u8);

        if chip_type == 4 || chip_type == 7 {
            self.send_icp_byte(icp_cmd::ICP_SET_XPAGE);
            self.send_icp_byte(((addr & 0xFF0000) >> 16) as u8);
        }

        let region = if custom_block {
            icp_cmd::ICP_READ_CUSTOM_BLOCK
        } else {
            icp_cmd::ICP_READ_FLASH
        };
        self.send_icp_byte(region);

        for i in 0..buffer.len() {
            buffer[i] = self.receive_icp_byte();
        }

        self.reset();

        true
    }

    fn icp_write_region(&mut self, addr: u32, data: &[u8], custom_block: bool) -> bool {
        self.switch_mode(Mode::ICP);

        let Some(chip_type) = self.chip_type else {
            return false;
        };

        if chip_type != 1 {
            self.send_icp_byte(0x46);
            self.send_icp_byte(0xF0);
            self.send_icp_byte(0xFF);
        }

        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_L);
        self.send_icp_byte((addr & 0xFF) as u8);
        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_H);
        self.send_icp_byte(((addr & 0xFF00) >> 8) as u8);
        if chip_type == 4 || chip_type == 7 {
            self.send_icp_byte(icp_cmd::ICP_SET_XPAGE);
            self.send_icp_byte(((addr & 0xFF0000) >> 16) as u8);
        }

        self.send_icp_byte(icp_cmd::ICP_SET_IB_DATA);
        self.send_icp_byte(data[0]);

        // Command byte: 0xa5 for custom region, 0x6e for flash
        let cmd = if custom_block { 0xa5 } else { 0x6e };
        self.send_icp_byte(cmd);
        self.send_icp_byte(0x15);
        self.send_icp_byte(0x0a);
        self.send_icp_byte(0x09);
        self.send_icp_byte(0x06);
        self.send_icp_byte(data[1]);

        self.delay_us(10);

        self.send_icp_byte(0x00);
        if !self.tdo_read() {
            return false;
        }

        for i in 2..data.len() {
            self.send_icp_byte(data[i]);
            self.delay_us(5);
            self.send_icp_byte(0x00);
            if !self.tdo_read() {
                return false;
            }
        }

        self.send_icp_byte(0x00);
        self.send_icp_byte(0xaa);
        // TDO must go high here to indicate success
        if !self.tdo_read() {
            return false;
        }
        self.send_icp_byte(0x00);
        self.send_icp_byte(0x00);

        self.delay_us(5);

        true
    }

    fn icp_write_flash(&mut self, addr: u32, data: &[u8]) -> bool {
        self.icp_write_region(addr, data, false)
    }

    fn icp_mass_erase(&mut self, alternate: bool) -> bool {
        self.switch_mode(Mode::ICP);

        let Some(chip_type) = self.chip_type else {
            return false;
        };

        if chip_type != 1 {
            self.send_icp_byte(0x46);
            self.send_icp_byte(0xF0);
            self.send_icp_byte(0xFF);
        }

        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_L);
        self.send_icp_byte(0x00);
        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_H);
        self.send_icp_byte(0x00);
        if chip_type == 4 || chip_type == 7 {
            self.send_icp_byte(icp_cmd::ICP_SET_XPAGE);
            self.send_icp_byte(0x00);
        }

        self.send_icp_byte(icp_cmd::ICP_SET_IB_DATA);
        self.send_icp_byte(0x00);

        // 0x4b - normal mass erase
        // 0xc3 - alternate mass erase (used when code options have non-default bits)
        let erase_cmd = if alternate { 0xc3 } else { 0x4b };
        self.send_icp_byte(erase_cmd);
        self.send_icp_byte(0x15);
        self.send_icp_byte(0x0a);
        self.send_icp_byte(0x09);
        self.send_icp_byte(0x06);
        self.send_icp_byte(0xff);

        self.pins.tdi.set_high(); // keep tdi line high

        self.delay.delay_ms(30u8);
        while !self.tdo_read() {
            self.delay.delay_ms(5u8);
            self.send_icp_byte(0x00);
        }

        return true; // TODO: at some point, time out and return false
    }

    fn icp_erase_flash(&mut self, addr: u32) -> bool {
        self.switch_mode(Mode::ICP);

        let Some(chip_type) = self.chip_type else {
            return false;
        };

        if chip_type != 1 {
            self.send_icp_byte(0x46);
            self.send_icp_byte(0xF0);
            self.send_icp_byte(0xFF);
        }

        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_L);
        self.send_icp_byte((addr & 0xFF) as u8);
        self.send_icp_byte(icp_cmd::ICP_SET_IB_OFFSET_H);
        self.send_icp_byte(((addr & 0xFF00) >> 8) as u8);
        if chip_type == 4 || chip_type == 7 {
            self.send_icp_byte(icp_cmd::ICP_SET_XPAGE);
            self.send_icp_byte(((addr & 0xFF0000) >> 16) as u8);
        }

        self.send_icp_byte(icp_cmd::ICP_SET_IB_DATA);
        self.send_icp_byte(0x00);

        self.send_icp_byte(0xe6);
        self.send_icp_byte(0x15);
        self.send_icp_byte(0x0a);
        self.send_icp_byte(0x09);
        self.send_icp_byte(0x06);
        self.send_icp_byte(0x00);

        self.delay.delay_ms(300u16);
        self.send_icp_byte(0x00);
        let status = self.pins.tdo.is_high();
        self.send_icp_byte(0x00);

        status
    }

    fn icp_write_custom_region(&mut self, addr: u32, data: &[u8]) -> bool {
        self.icp_write_region(addr, data, true)
    }
}

#[atmega_hal::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    #[cfg(feature = "atmega328p")]
    let pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    #[cfg(feature = "atmega328pb")]
    let pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

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

    // Buffer for flash operations
    let mut buffer: [u8; 1024] = [0; 1024];

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

            cmd::CMD_GET_VERSION => {
                // Return firmware version (major, minor)
                let _ = nb::block!(tx.write(cmd::RSP_DATA));
                let _ = nb::block!(tx.write(VERSION_MAJOR));
                let _ = nb::block!(tx.write(VERSION_MINOR));
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

            cmd::CMD_GET_ID => {
                let id = icp.jtag_get_id();
                let _ = nb::block!(tx.write(cmd::RSP_DATA));
                let _ = nb::block!(tx.write((id & 0xFF) as u8));
                let _ = nb::block!(tx.write((id >> 8) as u8));
            }

            cmd::CMD_SET_CONFIG => {
                let chip_type = nb::block!(rx.read()).unwrap_or(0);
                icp.set_chip_type(chip_type);
                let _ = nb::block!(tx.write(cmd::RSP_OK));
            }

            cmd::CMD_GET_CONFIG => {
                if let Some(chip_type) = icp.chip_type {
                    let _ = nb::block!(tx.write(cmd::RSP_DATA));
                    let _ = nb::block!(tx.write(chip_type));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
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

                if icp.icp_read_flash(addr, &mut buffer[..read_len], false) {
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

            cmd::CMD_READ_CUSTOM_REGION => {
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

                if icp.icp_read_flash(addr, &mut buffer[..read_len], true) {
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

                // Read data to write
                for i in 0..write_len {
                    buffer[i] = nb::block!(rx.read()).unwrap_or(0);
                }

                if icp.icp_write_flash(addr, &buffer[..write_len]) {
                    let _ = nb::block!(tx.write(cmd::RSP_OK));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }

            cmd::CMD_ERASE_FLASH_SECTOR => {
                // Read address (4 bytes)
                let addr = {
                    let b0 = nb::block!(rx.read()).unwrap_or(0);
                    let b1 = nb::block!(rx.read()).unwrap_or(0);
                    let b2 = nb::block!(rx.read()).unwrap_or(0);
                    let b3 = nb::block!(rx.read()).unwrap_or(0);
                    u32::from_le_bytes([b0, b1, b2, b3])
                };

                if icp.icp_erase_flash(addr) {
                    let _ = nb::block!(tx.write(cmd::RSP_OK));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }

            cmd::CMD_MASS_ERASE => {
                let alternate = nb::block!(rx.read()).unwrap_or(0) != 0;
                if icp.icp_mass_erase(alternate) {
                    let _ = nb::block!(tx.write(cmd::RSP_OK));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }

            cmd::CMD_WRITE_CUSTOM_REGION => {
                // Read address (4 bytes)
                let addr = {
                    let b0 = nb::block!(rx.read()).unwrap_or(0);
                    let b1 = nb::block!(rx.read()).unwrap_or(0);
                    let b2 = nb::block!(rx.read()).unwrap_or(0);
                    let b3 = nb::block!(rx.read()).unwrap_or(0);
                    u32::from_le_bytes([b0, b1, b2, b3])
                };
                // Read length (2 bytes)
                let len = {
                    let low = nb::block!(rx.read()).unwrap_or(0);
                    let high = nb::block!(rx.read()).unwrap_or(0);
                    u16::from_le_bytes([low, high]) as usize
                };

                // Clamp length to buffer size
                let write_len = len.min(buffer.len());

                // Read data to write
                for i in 0..write_len {
                    buffer[i] = nb::block!(rx.read()).unwrap_or(0);
                }

                if icp.icp_write_custom_region(addr, &buffer[..write_len]) {
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
    #[cfg(feature = "atmega328p")]
    let pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    #[cfg(feature = "atmega328pb")]
    let pins = Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    // Setup serial at 115200 baud
    let serial = Usart::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        Baudrate::<MHz16>::new(115200),
    );

    let (mut _rx, mut tx) = serial.split();

    loop {
        let _ = nb::block!(tx.write(b'P'));
    }
}
