// Auto-generated from GPT file for SH68F097

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("68f0970000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 1024,
    default_code_options: &hex!("a000038d"),
    code_option_mask: &hex!("e0c00000"),
    jtag_id: 0x166a,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 2,
    bank_type: 1,
    customer_id: AddressField { address: 0x0800 },
    operation_number: AddressField { address: 0x0804 },
    customer_option: AddressField { address: 0x0806 },
    security: AddressField { address: 0x080a },
    serial_number: AddressField { address: 0x083c },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function"),
                    (1, "Enable WDT function (Default)"),
                ]),
            },
        ),
        (
            "OP_WDTPD",
            OptionInfo {
                byte_index: 0,
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in Power-Down mode (Default)"),
                    (1, "Enable WDT function in Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_RST",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([
                    (0, "Pin 5.2 used as RST pin"),
                    (1, "Pin 5.2 used as I/O pin (Default)"),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 1,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable LVR function (Default)"),
                    (1, "Enable LVR function"),
                ]),
            },
        ),
        (
            "OP_LVRLE",
            OptionInfo {
                byte_index: 1,
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([(0, "4.3V LVR Level (Default)"), (1, "2.1V LVR Level")]),
            },
        ),
    ])
}
