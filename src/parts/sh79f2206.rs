// Auto-generated from GPT file for SH79F2206

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f2206000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 31744,
    eeprom_size: 1024,
    default_code_options: &hex!("80000300"),
    code_option_mask: &hex!("f8e8ef26"),
    jtag_id: 0x3213,
    sector_size: 512,
    option_byte_count: 4,
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
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable WDT function"),
                (1, "Disable WDT function"),
            ]),
        }),
        ("OP_ WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the Power-Down mode"),
                (1, "Enable WDT function in the Power-Down mode"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "P1.3 used as RST pin"),
                (1, "P1.3 used as I/O pin"),
            ]),
        }),
        ("OP_WMT", OptionInfo {
            byte_index: 0,
            bits_start: 3,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "longest warm up time"),
                (1, "longer warm up time"),
                (2, "shorter warm up time"),
                (3, "shortest warm up time"),
            ]),
        }),
        ("OP_LVREN", OptionInfo {
            byte_index: 1,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function"),
                (1, "Enable LVR function"),
            ]),
        }),
        ("OP_LVRLE", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "4.1V LVR Level 1"),
                (1, "3.7V LVR Level 2"),
                (2, "2.5V LVR Level 3"),
            ]),
        }),
        ("OP_SCM", OptionInfo {
            byte_index: 1,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "SCM is invalid in warm up period"),
                (1, "SCM is valid in warm up period"),
            ]),
        }),
        ("OP_PWMPIN", OptionInfo {
            byte_index: 2,
            bits_start: 5,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "The pin 20~25 as PWM0/PWM1/PWM2/PWM01/PWM11/PWM21"),
                (1, "The pin 20~25 as PWM0/PWM01/PWM1/PWM11/PWM2/PWM21"),
                (2, "The pin 20~25 as PWM21/PWM11/PWM01/PWM2/PWM1/PWM0"),
                (3, "The pin 20~25 as PWM21/PWM2/PWM11/PWM1/PWM01/PWM0"),
                (4, "The pin 20~25 as PWM2/PWM21/PWM11/PWM1/PWM01/PWM0"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (3, "Oscillator 1 is internal 128KHz RC, oscillator 2 is internal 12MHz RC"),
                (6, "Oscillator 1 is internal 128KHz RC, oscillator 2 is 400k-16MHz crystal /ceramic resonator"),
                (0, "Disable"),
                (1, "Enable"),
                (0, "Disable"),
                (1, "Enable"),
            ]),
        }),
        ("OP_PWMDRV", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Disable"),
                (1, "Enable"),
            ]),
        }),
    ])
}
