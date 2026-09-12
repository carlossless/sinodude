// Auto-generated from GPT file for SH68F099A

use super::{AddressField, Options, Part, SecurityRecordFormat, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("68f099a000"),
    chip_type: 0x00,
    custom_block: 0x07,
    product_block: 0x01,
    flash_size: 131072,
    eeprom_size: 0,
    default_code_options: &hex!("01020000"),
    code_option_mask: &hex!("0c0e0100"),
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
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
    security_record_format: SecurityRecordFormat::Record19,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::new()
}
