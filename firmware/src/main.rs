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
const VERSION_MAJOR: u8 = 6;
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
    pub const CMD_ERASE_EEPROM_PAGE: u8 = 0x0E;
    pub const CMD_SEND_KEY: u8 = 0x0F;

    // Read read-protected flash over the 3-wire OCD MOVC bypass (the CPU code-fetch is not gated); same wire args as CMD_READ_FLASH.
    pub const CMD_READ_FLASH_OCD: u8 = 0x10; // addr(u32 LE), len(u16 LE) -> len bytes

    // Response codes
    pub const RSP_OK: u8 = 0x00;
    pub const RSP_ERR: u8 = 0xFF;
    pub const RSP_DATA: u8 = 0x01;
}

// ICP command opcodes.
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

// 8051 opcodes injected into the target CPU over the debug link.
mod op8051 {
    pub const NOP: u8 = 0x00;
    pub const MOV_DPTR: u8 = 0x90; // MOV DPTR,#data16
    pub const MOVC_A_DPTR: u8 = 0x93; // MOVC A,@A+DPTR
    pub const CLR_A: u8 = 0xe4;
}

#[derive(PartialEq)]
enum Mode {
    Unset,
    Ready,
    Icp,
    Jtag,
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
    jtag_gadget: Option<u16>,
    clk_delay: u32,
    ocd_slow: u16,
}

impl IcpController {
    fn new(pins: IcpPins) -> Self {
        Self {
            pins,
            delay: Delay::<MHz16>::new(),
            connected: false,
            mode: Mode::Unset,
            chip_type: None,
            jtag_gadget: None,
            clk_delay: 2,
            ocd_slow: 8,
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
        self.jtag_gadget = None;
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
        self.mode = Mode::Icp;
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
            Mode::Unset => {
                return;
            }
            Mode::Icp | Mode::Ready => {
                self.tck_high();

                self.delay_us(8);

                self.tms_high();
                self.delay_us(2);
                self.tms_low();
                self.delay_us(2);
            }
            Mode::Jtag => {
                for _ in 0..35 {
                    self.jtag_next_state(true);
                }
                self.pins.tck.set_high();
                self.pins.tms.set_low();
            }
        }

        self.mode = Mode::Ready;
    }

    fn start_mode(&mut self) {
        self.tck_low();
        self.delay_us(2);

        let mut mode: u8 = match self.mode {
            Mode::Unset => 0,
            Mode::Ready => 1,
            Mode::Icp => 150,
            Mode::Jtag => 165,
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
        self.switch_mode(Mode::Icp);

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

        if self.mode != Mode::Ready {
            self.reset();
        }

        self.mode = mode;
        self.start_mode();

        if self.mode == Mode::Icp {
            self.delay_us(800);

            self.ping_icp();
        } else if self.mode == Mode::Jtag {
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

            // most likely breakpoint init; SH68F881W works without it, maybe mandatory for other chips
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
        if self.mode != Mode::Icp {
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
        self.switch_mode(Mode::Jtag);

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
        self.delay_us(self.clk_delay);

        let b = self.pins.tdo.is_high();
        self.pins.tck.set_low();
        self.delay_us(self.clk_delay);

        b
    }

    fn jtag_next_state_out(&mut self, tms: bool, out: bool) -> bool {
        if out {
            self.tdi_high();
        } else {
            self.tdi_low();
        }

        self.jtag_next_state(tms)
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

        value
    }

    fn icp_select_program_space(&mut self, chip_type: u8) {
        let space: u8 = match chip_type {
            0 => 0xfe,
            2 | 4 | 6 | 7 => 0xf0,
            _ => return,
        };
        self.send_icp_byte(0x46);
        self.send_icp_byte(space);
        self.send_icp_byte(0xff);
    }

    fn icp_erase_opcode(mode: u8, sub_flag: u8, chip_type: u8) -> u8 {
        match mode {
            0 => {
                if sub_flag == 0 {
                    0xe6
                } else {
                    0x55
                }
            }
            1 => {
                if chip_type == 7 {
                    0x4b
                } else {
                    0xaa
                }
            }
            2 => {
                if chip_type == 7 {
                    0x3c
                } else {
                    0xda
                }
            }
            3 => 0x5e,
            4 => 0x2d,
            _ => 0xc3,
        }
    }

    fn icp_send_key(&mut self, key: &[u8; 8], verify_addr: u32) -> u8 {
        self.switch_mode(Mode::Icp);

        self.send_icp_byte(0x4b);
        for &b in key.iter() {
            self.send_icp_byte(b);
        }

        let mut marker = [0u8; 1];
        if self.icp_read_flash(verify_addr, &mut marker, true) {
            marker[0]
        } else {
            0
        }
    }

    fn icp_read_flash(&mut self, addr: u32, buffer: &mut [u8], custom_block: bool) -> bool {
        self.switch_mode(Mode::Icp);

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

        for byte in buffer.iter_mut() {
            *byte = self.receive_icp_byte();
        }

        self.reset();

        true
    }

    // 3-wire OCD debug link: clk = TCK (PD5), data-out = TDI (PD4), data-in = TDO (PD2), FRAME = TMS (PD3); frame-marked transfers, not JTAG TAP.
    #[inline(never)]
    fn ocd_pulse(&mut self, d: u32) {
        self.tck_high();
        self.delay_us(d);
        self.tck_low();
        self.delay_us(d);
    }

    /// Frame-marked 4-bit send, LSB-first; `typ` 0 = type A (IR select), 1 = type B (4-bit DR shift); `d` = pulse delay.
    #[inline(never)]
    fn ocd_send4(&mut self, n: u8, d: u32, typ: u8) {
        let s = self.clk_delay;
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(d);
        if typ == 0 {
            self.delay_us(s);
            self.ocd_pulse(d);
        }
        self.tms_low();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.delay_us(s);
        self.ocd_pulse(d);
        for i in 0..3u8 {
            if (n >> i) & 1 != 0 {
                self.tdi_high();
            } else {
                self.tdi_low();
            }
            self.delay_us(s);
            self.ocd_pulse(d);
        }
        if (n >> 3) & 1 != 0 {
            self.tdi_high();
        } else {
            self.tdi_low();
        }
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(d);
        if typ == 0 {
            self.delay_us(s);
            self.ocd_pulse(d);
            self.tms_low();
            self.tdi_low();
            self.delay_us(s);
            self.ocd_pulse(d);
        } else {
            self.tdi_low();
            self.delay_us(s);
            self.ocd_pulse(d);
            self.tms_low();
            self.delay_us(s);
            self.ocd_pulse(d);
            self.ocd_pulse(d);
        }
    }

    /// Frame-marked 16-bit read, MSB-first.
    #[inline(never)]
    fn ocd_read16(&mut self) -> u16 {
        let s = self.clk_delay;
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(s);
        self.tms_low();
        self.delay_us(s);
        self.ocd_pulse(s);
        self.delay_us(s);
        self.ocd_pulse(s);
        let mut val: u16 = 0;
        for _ in 0..15 {
            self.tck_high();
            self.delay_us(s);
            let bit = self.tdo_read() as u16;
            val = (val | bit) << 1;
            self.tck_low();
            self.delay_us(s);
        }
        self.tms_high();
        self.delay_us(s);
        self.tck_high();
        self.delay_us(s);
        val |= self.tdo_read() as u16;
        self.tck_low();
        self.delay_us(s);
        self.tms_low();
        val
    }

    /// Inject one 8051 opcode byte, MSB-first, with frame-marked preamble and postamble.
    #[inline(never)]
    fn ocd_inject_opcode(&mut self, b: u8) {
        let s = self.clk_delay;
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(s);
        self.tms_low();
        self.delay_us(s);
        self.ocd_pulse(s);
        self.delay_us(s);
        self.ocd_pulse(s);
        let mut i = 7i8;
        while i >= 1 {
            if (b >> (i as u8)) & 1 != 0 {
                self.tdi_high();
            } else {
                self.tdi_low();
            }
            self.delay_us(s);
            self.ocd_pulse(s);
            i -= 1;
        }
        if b & 1 != 0 {
            self.tdi_high();
        } else {
            self.tdi_low();
        }
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(s);
        self.tdi_low();
        self.delay_us(s);
        self.ocd_pulse(s);
        self.tms_low();
        self.delay_us(s);
        self.ocd_pulse(s);
        self.delay_us(s);
        self.ocd_pulse(s);
    }

    /// Send a 23-bit debug value: 16 bits `lo` LSB-first, then 7 bits `hi`, the last with frame raised.
    #[inline(never)]
    fn ocd_send23(&mut self, lo: u16, hi: u8, d: u32) {
        let s = self.clk_delay;
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.tms_low();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.delay_us(s);
        self.ocd_pulse(d);
        for i in 0..16u8 {
            if (lo >> i) & 1 != 0 {
                self.tdi_high();
            } else {
                self.tdi_low();
            }
            self.delay_us(s);
            self.ocd_pulse(d);
        }
        for i in 0..6u8 {
            if (hi >> i) & 1 != 0 {
                self.tdi_high();
            } else {
                self.tdi_low();
            }
            self.delay_us(s);
            self.ocd_pulse(d);
        }
        if (hi >> 6) & 1 != 0 {
            self.tdi_high();
        } else {
            self.tdi_low();
        }
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.tdi_low();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.tms_low();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.delay_us(s);
        self.ocd_pulse(d);
    }

    /// Read the halted CPU context into `out` (out[0]=status, out[1]=byte0, out[2..8]=6 register bytes).
    #[inline(never)]
    fn ocd_read_context(&mut self, out: &mut [u8; 8]) {
        let s = self.clk_delay;
        let d = self.ocd_slow as u32;
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.tms_low();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.delay_us(s);
        self.ocd_pulse(d);
        // status bit0, bit1
        self.tck_high();
        self.delay_us(s);
        let b0 = self.tdo_read() as u8;
        self.tck_low();
        self.delay_us(s);
        self.tck_high();
        self.delay_us(s);
        let b1 = self.tdo_read() as u8;
        self.tck_low();
        self.delay_us(s);
        // 8-bit byte0, LSB-first
        let mut byte0: u8 = 0;
        for i in 0..8u8 {
            self.tck_high();
            self.delay_us(s);
            byte0 |= (self.tdo_read() as u8) << i;
            self.tck_low();
            self.delay_us(s);
        }
        // status bit2, bit3
        self.tck_high();
        self.delay_us(s);
        let b2 = self.tdo_read() as u8;
        self.tck_low();
        self.delay_us(s);
        self.tck_high();
        self.delay_us(s);
        let b3 = self.tdo_read() as u8;
        self.tck_low();
        self.delay_us(s);
        out[0] = b0 | (b1 << 1) | (b2 << 2) | (b3 << 3);
        out[1] = byte0;
        // 4 dummy clocks
        for _ in 0..4 {
            self.ocd_pulse(s);
        }
        // 6 context bytes, LSB-first
        for n in 0..6usize {
            let mut v: u8 = 0;
            for i in 0..8u8 {
                self.tck_high();
                self.delay_us(s);
                self.tck_low();
                v |= (self.tdo_read() as u8) << i;
                self.delay_us(s);
            }
            out[2 + n] = v;
        }
        // postamble
        self.tms_high();
        self.delay_us(s);
        self.ocd_pulse(d);
        self.tms_low();
        self.delay_us(s);
        self.ocd_pulse(d);
    }

    /// Enter OCD debug mode: framed control-register init (0x403000/0x402000/0x400000 + breakpoint regs 0x63..0x7F), then inject MOV 0xFF,#0x80 (SFR 0xFF) to arm the debug/ISP engine; assumes the target is already connected.
    fn ocd_enter(&mut self) {
        let f = self.clk_delay;
        self.ocd_send4(2, f, 0); // IR2
        self.ocd_send4(4, f, 1); // data4(4)
        self.ocd_send4(3, f, 0); // IR3
        self.ocd_send23(0x3000, 0x40, f); // 0x403000
        self.ocd_send23(0x2000, 0x40, f); // 0x402000
        self.ocd_send23(0x0000, 0x40, f); // 0x400000
        self.ocd_send23(0x0000, 0x63, f); // breakpoint regs 0x63..0x7F, disabled
        self.ocd_send23(0x0000, 0x67, f);
        self.ocd_send23(0x0000, 0x6b, f);
        self.ocd_send23(0x0000, 0x6f, f);
        self.ocd_send23(0x0000, 0x73, f);
        self.ocd_send23(0x0000, 0x77, f);
        self.ocd_send23(0x0000, 0x7b, f);
        self.ocd_send23(0x0000, 0x7f, f);
        self.ocd_send4(2, f, 0); // IR2
        self.ocd_send4(1, f, 1); // data4(1)
        self.ocd_send4(0x0c, f, 0); // IR12
        // Enable debug/ISP: inject MOV 0xFF,#0x80 (SFR 0xFF <- 0x80).
        self.ocd_inject_opcode(0x75);
        self.ocd_inject_opcode(0xff);
        self.ocd_inject_opcode(0x80);
        self.ocd_inject_opcode(0x00);
        self.ocd_inject_opcode(0x00);
    }

    /// Read one flash byte over the OCD MOVC bypass (reads read-protected flash; the CPU code-fetch is not gated); requires ocd_enter() first.
    /// Run the injected instructions (run 0x402000, halt 0x400000) and read back the halted context.
    #[inline(never)]
    fn ocd_step(&mut self, ctx: &mut [u8; 8]) {
        let f = self.clk_delay;
        self.ocd_send4(2, f, 0);
        self.ocd_send4(4, f, 1);
        self.ocd_send4(3, f, 0);
        self.ocd_send23(0x2000, 0x40, f);
        self.ocd_send4(4, f, 0);
        self.delay_us(self.ocd_slow as u32);
        self.ocd_send4(2, f, 0);
        self.ocd_send4(4, f, 1);
        self.ocd_send4(3, f, 0);
        self.ocd_send23(0x0000, 0x40, f);
        self.ocd_send4(2, f, 0);
        self.ocd_send4(1, f, 1);
        self.ocd_send4(0x0c, f, 0);
        self.delay_us(self.ocd_slow as u32);
        self.ocd_read_context(ctx);
    }

    #[inline(never)]
    fn ocd_movc_byte(&mut self, addr: u16) -> u8 {
        self.ocd_inject_opcode(op8051::NOP);
        self.ocd_inject_opcode(op8051::NOP);
        self.ocd_inject_opcode(op8051::NOP);
        self.ocd_inject_opcode(op8051::CLR_A);
        self.ocd_inject_opcode(op8051::MOV_DPTR);
        self.ocd_inject_opcode((addr >> 8) as u8);
        self.ocd_inject_opcode(addr as u8);
        self.ocd_inject_opcode(op8051::MOVC_A_DPTR);
        let mut ctx = [0u8; 8];
        self.ocd_step(&mut ctx);
        // decode: flash = rev6(ctx[3] low 6 bits) | ctx[2] bit7 -> bit6 | bit6 -> bit7
        let low6 = (ctx[3].reverse_bits() >> 2) & 0x3f;
        low6 | (((ctx[2] >> 7) & 1) << 6) | (((ctx[2] >> 6) & 1) << 7)
    }

    /// Read `buf.len()` read-protected flash bytes from `addr` over the OCD MOVC bypass; addresses 0..2 misdecode here and are patched host-side from an ICP read.
    fn ocd_read_flash(&mut self, addr: u32, buf: &mut [u8]) {
        self.clk_delay = 1;
        self.ocd_slow = 50;
        self.jtag_get_id();
        self.ocd_enter();
        for (i, b) in buf.iter_mut().enumerate() {
            *b = self.ocd_movc_byte((addr as u16).wrapping_add(i as u16));
        }
    }

    fn icp_write_region(&mut self, addr: u32, data: &[u8], custom_block: bool) -> bool {
        self.switch_mode(Mode::Icp);

        let Some(chip_type) = self.chip_type else {
            return false;
        };

        self.icp_select_program_space(chip_type);

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

        for byte in data.iter().skip(2) {
            self.send_icp_byte(*byte);
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

    fn icp_select_erase_space(&mut self) {
        self.send_icp_byte(0x46);
        self.send_icp_byte(0xf0);
        self.send_icp_byte(0xff);
    }

    fn icp_mass_erase(&mut self, mode: u8) -> bool {
        self.switch_mode(Mode::Icp);

        let Some(chip_type) = self.chip_type else {
            return false;
        };

        self.icp_select_erase_space();

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

        let erase_cmd = Self::icp_erase_opcode(mode, 0, chip_type);
        self.send_icp_byte(erase_cmd);
        self.send_icp_byte(0x15);
        self.send_icp_byte(0x0a);
        self.send_icp_byte(0x09);
        self.send_icp_byte(0x06);
        self.send_icp_byte(0xff);

        self.pins.tdi.set_high(); // keep tdi line high

        self.delay.delay_ms(30u8);
        let mut waited_ms: u16 = 0;
        while !self.tdo_read() {
            self.delay.delay_ms(5u8);
            self.send_icp_byte(0x00);
            waited_ms += 5;
            if waited_ms >= 5000 {
                return false;
            }
        }

        true
    }

    fn icp_erase_flash(&mut self, addr: u32) -> bool {
        self.icp_erase_at(addr, 0)
    }

    fn icp_erase_eeprom_page(&mut self, addr: u32) -> bool {
        self.icp_erase_at(addr, 1)
    }

    fn icp_erase_at(&mut self, addr: u32, sub_flag: u8) -> bool {
        self.switch_mode(Mode::Icp);

        let Some(chip_type) = self.chip_type else {
            return false;
        };

        self.icp_select_erase_space();

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

        self.send_icp_byte(Self::icp_erase_opcode(0, sub_flag, chip_type));
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

            cmd::CMD_READ_FLASH | cmd::CMD_READ_FLASH_OCD => {
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

                // CMD_READ_FLASH_OCD (0x0F) = 3-wire OCD MOVC bypass; CMD_READ_FLASH (0x08) = fast ICP read.
                let ok = if cmd_byte == cmd::CMD_READ_FLASH_OCD {
                    icp.ocd_read_flash(addr, &mut buffer[..read_len]);
                    true
                } else {
                    icp.icp_read_flash(addr, &mut buffer[..read_len], false)
                };
                if ok {
                    let _ = nb::block!(tx.write(cmd::RSP_DATA));
                    let _ = nb::block!(tx.write(read_len as u8));
                    let _ = nb::block!(tx.write((read_len >> 8) as u8));
                    for byte in buffer[..read_len].iter() {
                        let _ = nb::block!(tx.write(*byte));
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
                    for byte in buffer[..read_len].iter() {
                        let _ = nb::block!(tx.write(*byte));
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
                for byte in buffer[..write_len].iter_mut() {
                    *byte = nb::block!(rx.read()).unwrap_or(0);
                }

                if icp.icp_write_flash(addr, &buffer[..write_len]) {
                    let _ = nb::block!(tx.write(cmd::RSP_OK));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }
            cmd::CMD_SEND_KEY => {
                let mut key = [0u8; 8];
                for b in key.iter_mut() {
                    *b = nb::block!(rx.read()).unwrap_or(0);
                }
                let verify_addr = {
                    let b0 = nb::block!(rx.read()).unwrap_or(0);
                    let b1 = nb::block!(rx.read()).unwrap_or(0);
                    let b2 = nb::block!(rx.read()).unwrap_or(0);
                    let b3 = nb::block!(rx.read()).unwrap_or(0);
                    u32::from_le_bytes([b0, b1, b2, b3])
                };

                let marker = icp.icp_send_key(&key, verify_addr);
                let _ = nb::block!(tx.write(cmd::RSP_DATA));
                let _ = nb::block!(tx.write(marker));
            }

            cmd::CMD_ERASE_EEPROM_PAGE => {
                let addr = {
                    let b0 = nb::block!(rx.read()).unwrap_or(0);
                    let b1 = nb::block!(rx.read()).unwrap_or(0);
                    let b2 = nb::block!(rx.read()).unwrap_or(0);
                    let b3 = nb::block!(rx.read()).unwrap_or(0);
                    u32::from_le_bytes([b0, b1, b2, b3])
                };

                if icp.icp_erase_eeprom_page(addr) {
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
                let mode = nb::block!(rx.read()).unwrap_or(1);
                if icp.icp_mass_erase(mode) {
                    let _ = nb::block!(tx.write(cmd::RSP_OK));
                } else {
                    let _ = nb::block!(tx.write(cmd::RSP_ERR));
                }
            }

            cmd::CMD_WRITE_CUSTOM_REGION => {
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
                let write_len = len.min(buffer.len());
                for byte in buffer[..write_len].iter_mut() {
                    *byte = nb::block!(rx.read()).unwrap_or(0);
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
