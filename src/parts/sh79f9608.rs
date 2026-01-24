// Auto-generated from GPT file for SH79F9608

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f9608000"),
    chip_type: 0x02,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 512,
    default_code_options: &hex!("08000080"),
    code_option_mask: &hex!("fa000000"),
    jtag_id: 0x1628,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 3,
    bank_type: 0,
    customer_id: AddressField { address: 0x1000 },
    operation_number: AddressField { address: 0x1004 },
    customer_option: AddressField { address: 0x1006 },
    security: AddressField { address: 0x100a },
    serial_number: AddressField { address: 0x103c },
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
                states: IndexMap::from([(0, "Enable WDT function."), (1, "Disable WDT function.")]),
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
                    (0, "Disable WDT function in the Power-Down mode."),
                    (1, "Enable WDT function in the Power-Down mode."),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function."), (1, "Enable LVR function.")]),
            },
        ),
        (
            "OP_LVRLE",
            OptionInfo {
                byte_index: 0,
                bits_start: 3,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([(1, "3.7V LVR Level 1"), (2, "2.8V LVR Level 2")]),
            },
        ),
        (
            "OP_PPGO",
            OptionInfo {
                byte_index: 0,
                bits_start: 2,
                bits_end: 2,
                editable: false,
                states: IndexMap::from([
                    (0, "PPG output pulse is low effective."),
                    (1, "PPG output pulse is high effective."),
                ]),
            },
        ),
        (
            "OP_I/O",
            OptionInfo {
                byte_index: 0,
                bits_start: 1,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([
                    (0, "P0.4/P1.3/P1.4/P1.6 drive ability normal mode."),
                    (1, "P0.4/P1.3/P1.4/P1.6 drive ability large mode."),
                ]),
            },
        ),
        (
            "OP_PPGO Limit",
            OptionInfo {
                byte_index: 1,
                bits_start: 6,
                bits_end: 7,
                editable: false,
                states: IndexMap::from([
                    (0, "2048*tosc"),
                    (1, "1280*tosc"),
                    (2, "768*tosc"),
                    (3, "512*tosc"),
                ]),
            },
        ),
        (
            "OP_ISP",
            OptionInfo {
                byte_index: 3,
                bits_start: 7,
                bits_end: 7,
                editable: false,
                states: IndexMap::from([(0, "Enable ISP function."), (1, "Disable ISP function.")]),
            },
        ),
    ])
}
