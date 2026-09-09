// Auto-generated from GPT file for 67P90

use super::{SecurityRecordFormat, AddressField, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("0000000000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 0,
    eeprom_size: 0,
    default_code_options: &hex!("0004"),
    code_option_mask: &hex!("00fc"),
    jtag_id: 0x0000,
    sector_size: 0,
    option_byte_count: 2,
    security_level: 0,
    bank_type: 0,
    single_wire: false,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0x0000 },
    serial_number: AddressField { address: 0x0000 },
    compatible_voltages: &[],
    options,
    erase_full_mode: 5,
    option_high_addr: None,
    security_record_format: SecurityRecordFormat::Record19,
    isp_password_addr: None,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::new()
}
