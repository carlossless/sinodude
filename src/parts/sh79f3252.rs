// Auto-generated from GPT file for SH79F3252

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f3252000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 32768,
    eeprom_size: 4096,
    default_code_options: &hex!("007b438000000080"),
    code_option_mask: &hex!("ff9eef1f0000000f"),
    jtag_id: 0xf352,
    sector_size: 512,
    option_byte_count: 8,
    security_level: 4,
    bank_type: 1,
    customer_id: AddressField { address: 0x1000 },
    operation_number: AddressField { address: 0x1004 },
    customer_option: AddressField { address: 0x1006 },
    security: AddressField { address: 0x100a },
    serial_number: AddressField { address: 0x103c },
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        ("OP_P0", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Port0(0~7)sink ability normal mode.(Default)"),
                (1, "Port0(0~7)sink ability large mode."),
            ]),
        }),
        ("OP_P1", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Port1(0~7)sink ability normal mode.(Default)"),
                (1, "Port1(0~7)sink ability large mode."),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "P4.7used as RST pin.(Default)"),
                (1, "P4.7 used as I/O pin."),
            ]),
        }),
        ("OP_P2", OptionInfo {
            byte_index: 0,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "Port2(0~7)sink ability normal mode.(Default)"),
                (1, "Port2(0~7)sink ability large mode."),
            ]),
        }),
        ("OP_P3", OptionInfo {
            byte_index: 0,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Port3(0~7)sink ability normal mode.(Default)"),
                (1, "Port3(0~7)sink ability large mode."),
            ]),
        }),
        ("OP_SCMEN", OptionInfo {
            byte_index: 0,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Enable SCM.(Default)"),
                (1, "Disable SCM."),
            ]),
        }),
        ("OP_P4", OptionInfo {
            byte_index: 0,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Port4(0~7)sink ability normal mode.(Default)"),
                (1, "Port4(0~7)sink ability large mode."),
            ]),
        }),
        ("OP_P5", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Port5(0~4)sink ability normal mode.(Default)"),
                (1, "Port5(0~4)sink ability large mode."),
            ]),
        }),
        ("OP_LVREN", OptionInfo {
            byte_index: 1,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function.(Default)"),
                (1, "Enable LVR function."),
            ]),
        }),
        ("OP_LVRLE", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 6,
            editable: false,
            states: IndexMap::from([
                (0, "4.1V LVR Level1."),
                (1, "3.7V LVR Level2."),
                (2, "2.8V LVR Level3."),
                (3, "1.9V LVR Level4.(Default)"),
            ]),
        }),
        ("OP_SCM", OptionInfo {
            byte_index: 1,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "SCM is invalid in warm up period."),
                (1, "SCM is valid in warm up period.(Default)"),
            ]),
        }),
        ("OP_REM_CUTTENT(P0.0 REM sink ability)", OptionInfo {
            byte_index: 1,
            bits_start: 2,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "125 mA."),
                (1, "250 mA."),
                (2, "350 mA.(Default)"),
                (3, "450 mA."),
            ]),
        }),
        ("OP_LCDSEL", OptionInfo {
            byte_index: 1,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Select resistor LCD driver."),
                (1, "Select capacitor LCD driver.(Default)"),
            ]),
        }),
        ("OP_LVRPD", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 0,
            editable: false,
            states: IndexMap::from([
                (0, "Disable LVR function in Power-Down mode.(Default)"),
                (1, "Enable LVR function in Power-Down mode."),
            ]),
        }),
        ("OP_PWM_REM_IOSEL", OptionInfo {
            byte_index: 2,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "P1.2 as PWM1/REM Pin.(Defaut)"),
                (1, "P0.0 as PWM1/REM Pin."),
            ]),
        }),
        ("OP_AHRV", OptionInfo {
            byte_index: 2,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "32.768K crystal oscillator humidity resistance control bit(AHUM)reset value is 0."),
                (1, "32.768K crystal oscillator humidity resistance control bit(AHUM)reset value is 1.(Defualt)"),
            ]),
        }),
        ("OP_LRCSEL", OptionInfo {
            byte_index: 2,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Internal Low RC output 32K Hz.(Default)"),
                (1, "Internal Low RC output 128K Hz."),
            ]),
        }),
        ("OP_HRCSEL", OptionInfo {
            byte_index: 2,
            bits_start: 4,
            bits_end: 4,
            editable: false,
            states: IndexMap::from([
                (0, "Internal High RC output 8M Hz.(Default)"),
                (1, "Internal High RC output 24M Hz."),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Internal High 8M RC as oscillator 1,oscillator 2 close."),
                (3, "Internal Low RC as oscillator 1, Internal High 8M RC as oscillator 2.(Default)"),
                (10, "External 32.768kHz Crystal as oscillator 1, Internal High 8M RC as oscillator 2."),
                (15, "External clock source as oscillator 1,oscillator 2 close."),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 3,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "Enable ISP function."),
                (1, "Disable ISP function.(Default)"),
            ]),
        }),
        ("OP_ ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: false,
            states: IndexMap::from([
                (0, "Enter ISP mode directly regardless the condition of P3.5 and P3.6.(Default)"),
                (1, "Enter ISP mode only when P3.5 and P3.6 are connected to GND, simultaneously."),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (5, "Disable WDT function."),
                (0, "Enable WDT function.(Default)"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in Power-Down mode.(Default)"),
                (1, "Enable WDT function in Power-Down mode."),
            ]),
        }),
        ("OP_ISPSIZE", OptionInfo {
            byte_index: 7,
            bits_start: 4,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "8*1024Bytes"),
                (1, "7*1024Bytes"),
                (2, "6*1024Bytes"),
                (3, "5*1024Bytes"),
                (4, "4*1024Bytes"),
                (5, "3*1024Bytes"),
                (6, "2*1024Bytes"),
                (7, "1*1024Bytes"),
                (8, "0Bytes(Default)"),
            ]),
        }),
        ("OP_EEPROMSIZE", OptionInfo {
            byte_index: 7,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "8*512Bytes (Default)"),
                (1, "7*512Bytes"),
                (2, "6*512Bytes"),
                (3, "5*512Bytes"),
                (4, "4*512Bytes"),
                (5, "3*512Bytes"),
                (6, "2*512Bytes"),
                (7, "1*512Bytes"),
                (8, "0Bytes"),
            ]),
        }),
    ])
}
