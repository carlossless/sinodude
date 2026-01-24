// Auto-generated from GPT file for SH99F01

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("99f0100000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 2048,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("1c000000"),
    jtag_id: 0xd010,
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
        (
            "OP_REG33",
            OptionInfo {
                byte_index: 0,
                bits_start: 2,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable Internal 3.3V Regulator"),
                    (1, "Enable Internal 3.3V Regulator"),
                ]),
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
    ])
}
