// Auto-generated from GPT file for SH79F7016

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f7016000"),
    chip_type: 0x04,
    custom_block: 0x04,
    product_block: 0x01,
    flash_size: 131072,
    eeprom_size: 4096,
    default_code_options: &hex!("00000080"),
    code_option_mask: &hex!("0000003d"),
    jtag_id: 0x7016,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 3,
    bank_type: 1,
    customer_id: AddressField { address: 0x2000 },
    operation_number: AddressField { address: 0x2004 },
    customer_option: AddressField { address: 0x2006 },
    security: AddressField { address: 0x200a },
    serial_number: AddressField { address: 0x203c },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 3,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "Enable WDT function"), (1, "Disable WDT function")]),
            },
        ),
        (
            "OP_ WDTPD",
            OptionInfo {
                byte_index: 3,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in the Power-Down mode"),
                    (1, "Enable WDT function in the Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 3,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_LVRLE",
            OptionInfo {
                byte_index: 3,
                bits_start: 2,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([(0, "4.1V LVR Level"), (1, "2.1V LVR Level")]),
            },
        ),
        (
            "OP_32KCHK",
            OptionInfo {
                byte_index: 3,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([(0, "32K SCM enable in PD"), (1, "32K SCM disable in PD")]),
            },
        ),
    ])
}
