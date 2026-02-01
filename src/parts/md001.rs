// Auto-generated from GPT file for MD001

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f1610000"),
    chip_type: 0x00,
    custom_block: 0x01,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("ff000000"),
    jtag_id: 0x9c9c,
    sector_size: 2048,
    option_byte_count: 4,
    security_level: 0,
    bank_type: 0,
    customer_id: AddressField { address: 0x3fc0 },
    operation_number: AddressField { address: 0x3fc4 },
    customer_option: AddressField { address: 0x3fc6 },
    security: AddressField { address: 0x3fca },
    serial_number: AddressField { address: 0x3ffc },
    compatible_voltages: &[Voltage::V5_0],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_OSC",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([(0, "Internal RC oscillator"), (6, "Ceramic oscillator")]),
            },
        ),
        (
            "OP_RST",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "Pin7 used as RST pin"), (1, "Pin7 used as I/O pin")]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 0,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(0, "Disable WDT function"), (1, "Enable WDT function")]),
            },
        ),
    ])
}
