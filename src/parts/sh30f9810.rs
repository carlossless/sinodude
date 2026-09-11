// Auto-generated from GPT file for SH30F9810

use super::{AddressField, OptionInfo, Options, Part, SecurityRecordFormat, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("30f9810000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 131072,
    eeprom_size: 4096,
    default_code_options: &hex!("0000000000000000"),
    code_option_mask: &hex!("c0e0000000000000"),
    jtag_id: 0x0901,
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
    security_record_format: SecurityRecordFormat::Record19,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_ISP",
            OptionInfo {
                byte_index: 7,
                bits_start: 7,
                bits_end: 7,
                editable: false,
                states: IndexMap::from([(0, "Enable ISP Function"), (1, "Disable ISP Function")]),
            },
        ),
        (
            "OP_LVRLE",
            OptionInfo {
                byte_index: 1,
                bits_start: 6,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([
                    (0, "4.1V LVR Level"),
                    (1, "3.7V LVR Level"),
                    (2, "3.1V LVR Level"),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 1,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
    ])
}
