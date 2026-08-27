// Auto-generated from GPT file for SH30F9071
// Source GPT SHA-256: 80e0cf5f93cd5fe605a1331c55c116dec478e02e52ae5206065121ce71ba9d24

use super::{AddressField, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("30f9071000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 262144,
    eeprom_size: 4096,
    default_code_options: &hex!("0000000000000000"),
    code_option_mask: &hex!("c000000000000000"),
    jtag_id: 0x0f81,
    sector_size: 1024,
    option_byte_count: 8,
    security_level: 10,
    bank_type: 0,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0xfff8000 },
    serial_number: AddressField { address: 0x0000 },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::new()
}
