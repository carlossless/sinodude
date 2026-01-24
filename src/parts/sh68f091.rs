// Auto-generated from GPT file for SH68F091

use super::{AddressField, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("6809100000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 16384,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("00000000"),
    jtag_id: 0x9c9c,
    sector_size: 2048,
    option_byte_count: 4,
    security_level: 0,
    bank_type: 0,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0x0000 },
    serial_number: AddressField { address: 0x0000 },
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::new()
}
