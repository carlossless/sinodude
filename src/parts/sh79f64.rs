// Auto-generated from GPT file for SH79F64

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f6400000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 2048,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("ffff0000"),
    jtag_id: 0xf640,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 1,
    customer_id: AddressField { address: 0x0800 },
    operation_number: AddressField { address: 0x0804 },
    customer_option: AddressField { address: 0x0806 },
    security: AddressField { address: 0x080a },
    serial_number: AddressField { address: 0x083c },
    compatible_voltages: &[Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        ("OP_LVREN", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function"),
                (1, "Enable LVR function"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function"),
                (1, "Enable WDT function"),
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
                (0, "Internal RC Oscillator(8MHz)"),
                (3, "32.768kHz Crystal Oscillator at OSC, Internal RC can be enabled, OSCX shared with I/O(400kHz-8MHz)"),
                (4, "32.768kHz Crystal Oscillator at OSC, 400k ~ 12M Crystal Oscillator at OSCX"),
                (5, "Crystal Oscillator(400kHz-12MHz)"),
                (6, "Ceramic Oscillator(400kHz-12MHz)"),
                (7, "32.768kHz Crystal Oscillator at OSC, 400k ~ 12M Ceramic resonator at OSCX"),
            ]),
        }),
    ])
}
