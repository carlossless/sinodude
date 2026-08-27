// Auto-generated from GPT file for SH87F8801
// Source GPT SHA-256: 71da95f6fffa05276ee28b26640b6c0b8c6b4932e3f94df926ac03719a718b7f

use super::{AddressField, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("87f8801000"),
    chip_type: 0x00,
    custom_block: 0x07,
    product_block: 0x01,
    flash_size: 131072,
    eeprom_size: 0,
    default_code_options: &hex!("01030000"),
    code_option_mask: &hex!("0c030100"),
    jtag_id: 0x0000,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 6,
    bank_type: 0,
    customer_id: AddressField { address: 0x20000 },
    operation_number: AddressField { address: 0x20004 },
    customer_option: AddressField { address: 0x20008 },
    security: AddressField { address: 0x20010 },
    serial_number: AddressField { address: 0x20050 },
    compatible_voltages: &[Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::new()
}
