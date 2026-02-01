// Auto-generated from GPT file for SH79F329

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("36f3200000"),
    chip_type: 0x00,
    custom_block: 0x01,
    product_block: 0x01,
    flash_size: 32768,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("17000000"),
    jtag_id: 0x3601,
    sector_size: 2048,
    option_byte_count: 4,
    security_level: 0,
    bank_type: 0,
    customer_id: AddressField { address: 0x7fc0 },
    operation_number: AddressField { address: 0x7fc4 },
    customer_option: AddressField { address: 0x7fc6 },
    security: AddressField { address: 0x7fca },
    serial_number: AddressField { address: 0x7ffc },
    compatible_voltages: &[Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([(0, "Disable WDT function"), (1, "Enable WDT function")]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 0,
                bits_start: 1,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_EWDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 2,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT in PD mode"),
                    (1, "Enable WDT in PD mode"),
                ]),
            },
        ),
    ])
}
