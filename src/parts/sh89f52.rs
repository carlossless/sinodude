// Auto-generated from GPT file for SH89F52

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("89f5200000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 8192,
    eeprom_size: 512,
    default_code_options: &hex!("00010040"),
    code_option_mask: &hex!("ffffffcf"),
    jtag_id: 0xf516,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 0,
    customer_id: AddressField { address: 0x0800 },
    operation_number: AddressField { address: 0x0804 },
    customer_option: AddressField { address: 0x0806 },
    security: AddressField { address: 0x080a },
    serial_number: AddressField { address: 0x083c },
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
                (0, "Disable WDT function"),
                (1, "Enable WDT function"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in Power-Down mode"),
                (1, "Enable WDT function in Power-Down mode"),
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
        ("OP_OSC", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Internal RC Oscillator(16.6MHz)"),
                (2, "External clock(30KHz-16.6MHz)"),
                (3, "32.768kHz Crystal Oscillator,internal RC oscillator(16.6MHz) .can be enabled"),
                (5, "Crystal Oscillator(400kHz-16.6MHz) or Ceramic Resonator(2MHz-16.6MHz)"),
                (6, "Ceramic Resonator(400kHz-2MHz)"),
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
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "4.3V LVR level"),
                (1, "3.7V LVR level"),
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
        ("OP_IO", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "IO is Input-Only mode when reset occurs"),
                (1, "IO is Quasi-Bi mode when reset occurs"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 3,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable ISP function"),
                (1, "Disable ISP function"),
            ]),
        }),
        ("OP_ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Enter ISP mode only when P1.0 and P1.1 are connected to GND simultaneously"),
                (1, "Enter ISP mode directly regardless the condition of P1.0 and P1.1"),
            ]),
        }),
        ("OP_OVL", OptionInfo {
            byte_index: 3,
            bits_start: 5,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "OVL generates WDT Reset"),
                (1, "OVL generates OVL interrupt"),
            ]),
        }),
    ])
}
