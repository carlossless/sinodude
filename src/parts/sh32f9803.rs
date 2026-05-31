// Auto-generated from GPT file for SH32F9803

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("32f9803000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 131072,
    eeprom_size: 6144,
    default_code_options: &hex!("1510000000000000"),
    code_option_mask: &hex!("0000000000000000"),
    jtag_id: 0x0000,
    sector_size: 2048,
    option_byte_count: 8,
    security_level: 10,
    bank_type: 0,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0xfffe000 },
    serial_number: AddressField { address: 0x0000 },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([(
        "OP_IWDT",
        OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 3,
            editable: false,
            states: IndexMap::from([(0, "Enable WDT function"), (5, "Disable WDT function")]),
        },
    )])
}
