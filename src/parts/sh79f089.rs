// Auto-generated from GPT file for SH79F089

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f0890000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 8192,
    eeprom_size: 1024,
    default_code_options: &hex!("2000008d"),
    code_option_mask: &hex!("c0c00f00"),
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
            "OP_WDTPD",
            OptionInfo {
                byte_index: 0,
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in Power-Down mode"),
                    (1, "Enable WDT function in Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_OSC",
            OptionInfo {
                byte_index: 2,
                bits_start: 0,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([
                    (
                        0,
                        "Oscillator1 is internal 12MHz RC, oscillator2 is disabled",
                    ),
                    (
                        3,
                        "Oscillator1 is internal 128kHz RC, oscillator2 is internal 12MHz RC",
                    ),
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
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([(0, "4.3V LVR level 1"), (1, "2.1V LVR level 2")]),
            },
        ),
    ])
}
