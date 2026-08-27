// Auto-generated from GPT file for SH87F8815
// Source GPT SHA-256: bc9c4ea4999223907d8f03c653ca87bc0dda5ca3618a0e2da1f2e140c0e85488

use super::{AddressField, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("87f8815000"),
    chip_type: 0x00,
    custom_block: 0x07,
    product_block: 0x01,
    flash_size: 262144,
    eeprom_size: 8192,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("ff000000"),
    jtag_id: 0x5f15,
    sector_size: 2048,
    option_byte_count: 4,
    security_level: 6,
    bank_type: 0,
    customer_id: AddressField { address: 0x210000 },
    operation_number: AddressField { address: 0x210004 },
    customer_option: AddressField { address: 0x210008 },
    security: AddressField { address: 0x210010 },
    serial_number: AddressField { address: 0x210050 },
    compatible_voltages: &[Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::new()
}
