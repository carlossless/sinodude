// Auto-generated from GPT file for SH33F2053

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("33f2053000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 258048,
    eeprom_size: 3072,
    default_code_options: &hex!("1500000000000000"),
    code_option_mask: &hex!("1f00000000000080"),
    jtag_id: 0xf025,
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
    IndexMap::from([
        (
            "OP_WDTPD",
            OptionInfo {
                byte_index: 0,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable IWDT function in the Power-Down mode"),
                    (1, "Enable IWDT function in the Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_IWDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(5, "Disable IWDT Function"), (0, "Enable IWDT Function")]),
            },
        ),
        (
            "OP_ISP",
            OptionInfo {
                byte_index: 7,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Enable ISP Function"), (1, "Disable ISP Function")]),
            },
        ),
    ])
}
