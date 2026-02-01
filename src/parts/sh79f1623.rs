// Auto-generated from GPT file for SH79F1623

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f1623000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 512,
    default_code_options: &hex!("00010080"),
    code_option_mask: &hex!("e0e100c4"),
    jtag_id: 0x1623,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 0,
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
                states: IndexMap::from([(0, "Disable WDT function"), (1, "Enable WDT function")]),
            },
        ),
        (
            "OP_ WDTPD",
            OptionInfo {
                byte_index: 0,
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in the Power-Down mode"),
                    (1, "Enable WDT function in the Power-Down mode"),
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
                    (0, "Pin P4.0 used as RST pin"),
                    (1, "Pin P4.0 used as I/O pin"),
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
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_LVRLE",
            OptionInfo {
                byte_index: 1,
                bits_start: 5,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([(0, "4.1V LVR Level 1"), (1, "2.8V LVR Level 2")]),
            },
        ),
        (
            "OP_IO",
            OptionInfo {
                byte_index: 1,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (0, "IO is Input Only mode when reset occurs"),
                    (1, "IO is Quasi-Bi mode when reset occurs"),
                ]),
            },
        ),
        (
            "OP_ISP",
            OptionInfo {
                byte_index: 3,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Enable ISP function"), (1, "Disable ISP function")]),
            },
        ),
        (
            "OP_ISPSEL",
            OptionInfo {
                byte_index: 3,
                bits_start: 2,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (0, "Enter Main Block"),
                    (1, "Enter BootRom Block"),
                    (0, "Disable Protect"),
                    (1, "Enable Protect"),
                ]),
            },
        ),
    ])
}
