// Auto-generated from GPT file for SH68F093

use super::{AddressField, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("68f0930000"),
    chip_type: 0x02,
    custom_block: 0x01,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("00000000"),
    jtag_id: 0xf093,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 0,
    customer_id: AddressField { address: 0x3fc0 },
    operation_number: AddressField { address: 0x3fc4 },
    customer_option: AddressField { address: 0x3fc6 },
    security: AddressField { address: 0x3fca },
    serial_number: AddressField { address: 0x3ffc },
    compatible_voltages: &[Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::new()
}
