// Auto-generated from GPT file for SH79F1611

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f1611000"),
    chip_type: 0x02,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 2048,
    default_code_options: &hex!("80000018"),
    code_option_mask: &hex!("fffffffd"),
    jtag_id: 0x1611,
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
        ("OP_LVREN", OptionInfo {
            byte_index: 1,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function"),
                (1, "Enable LVR function"),
            ]),
        }),
        ("OP_LVRLE", OptionInfo {
            byte_index: 1,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "4.1V LVR level 1"),
                (1, "2.8V LVR level 2"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "P1.5 used as RST pin"),
                (1, "P1.5 used as I/O pin"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable WDT"),
                (1, "Disable WDT"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "disableWDT in STOP MODE"),
                (1, "enable WDT  in STOP MODE"),
            ]),
        }),
        ("OP_WMT", OptionInfo {
            byte_index: 0,
            bits_start: 3,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "longest warm up time"),
                (1, "longer warm up time"),
                (2, "shorter warm up time"),
                (3, "shortest warm up time"),
            ]),
        }),
        ("OP_SCM", OptionInfo {
            byte_index: 1,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "SCM is invalid in warm up period"),
                (1, "SCM is valid in warm up period"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator1 is internal 27M RC, oscillator2 is disabled"),
                (3, "Oscillator1 is internal 128k RC, oscillator2 is internal 27M RC"),
                (6, "Oscillator1 is internal 128k RC, oscillator2 is 400k-16M crystal/ceramic oscillator"),
                (14, "Oscillator1 is 400k-16M crystal/ceramic oscillator, oscillator2 is disabled"),
                (0, "0: disable"),
                (1, "1: enable"),
                (0, "disable"),
                (1, "enable"),
                (0, "disable"),
                (1, "enable"),
            ]),
        }),
        ("OP_CRMC", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "crystal is used"),
                (1, "ceramic is used"),
            ]),
        }),
        ("OP_OSCDRV", OptionInfo {
            byte_index: 3,
            bits_start: 3,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (3, "400k-16M crystal/ceramic oscillator driving control bit :highest"),
                (2, "400k-16M crystal/ceramic oscillator driving control bit : high"),
                (1, "400k-16M crystal/ceramic oscillator driving control bit : low"),
                (0, "400k-16M crystal/ceramic oscillator driving control bit : lowest"),
            ]),
        }),
    ])
}
