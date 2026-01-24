// Auto-generated from GPT file for SH86331

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("86f3310f00"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 2048,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("ffffffff"),
    jtag_id: 0x0000,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 2,
    bank_type: 0,
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
            "OP_SYS12MCLK",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (0, "Internal 12MHz RC"),
                    (1, "External 12MHz crystal OSC."),
                ]),
            },
        ),
        (
            "OP_RTCCLK",
            OptionInfo {
                byte_index: 0,
                bits_start: 1,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([
                    (0, "External 32.768kHz clock"),
                    (1, "Internal 12MHz clock"),
                ]),
            },
        ),
        (
            "OP_WDTEN",
            OptionInfo {
                byte_index: 0,
                bits_start: 2,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([(0, "Enable WDT"), (1, "Disable WDT")]),
            },
        ),
        (
            "OP_USBEP3EN",
            OptionInfo {
                byte_index: 0,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([
                    (0, "Enable USB endpoint 3"),
                    (1, "Disable USB endpoint 3"),
                ]),
            },
        ),
        (
            "OP_RTCIEN",
            OptionInfo {
                byte_index: 0,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([(0, "Enable RTC interface"), (1, "Disable RTC interface")]),
            },
        ),
        (
            "OP_ISPEN",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "Enable ISP function"), (1, "Disable ISP function")]),
            },
        ),
    ])
}
