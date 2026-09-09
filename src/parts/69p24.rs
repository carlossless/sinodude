// Auto-generated from GPT file for 69P24

use super::{SecurityRecordFormat, AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("0000000000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 0,
    eeprom_size: 0,
    default_code_options: &hex!("0000"),
    code_option_mask: &hex!("ffff"),
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
    IndexMap::from([
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 15,
            bits_end: 15,
            editable: false,
            states: IndexMap::from([
                (0, "Disable_WDT"),
                (1, "Enable_WDT"),
            ]),
        }),
        ("OP_LVR", OptionInfo {
            byte_index: 0,
            bits_start: 14,
            bits_end: 14,
            editable: false,
            states: IndexMap::from([
                (0, "Disable"),
                (1, "Enable"),
            ]),
        }),
        ("OP_LVR0", OptionInfo {
            byte_index: 0,
            bits_start: 13,
            bits_end: 13,
            editable: false,
            states: IndexMap::from([
                (0, "4V"),
                (1, "2.5V"),
            ]),
        }),
    ])
}
