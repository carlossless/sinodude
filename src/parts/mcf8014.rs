// Auto-generated from GPT file for MCF8014

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f1627000"),
    chip_type: 0x02,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 1024,
    default_code_options: &hex!("0002008c"),
    code_option_mask: &hex!("c6ea0128"),
    jtag_id: 0x1626,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 3,
    bank_type: 1,
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
                states: IndexMap::from([(0, "Enable WDT function"), (1, "Disable WDT function")]),
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
                    (0, "Disable WDT function in the Power-Down mode"),
                    (1, "Enable WDT function in the Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_WMT",
            OptionInfo {
                byte_index: 0,
                bits_start: 3,
                bits_end: 4,
                editable: false,
                states: IndexMap::from([
                    (0, "Longest warm-up time"),
                    (1, "Longer warm-up time"),
                    (2, "Short warm-up time"),
                    (3, "Shortest warm-up time"),
                ]),
            },
        ),
        (
            "OP_SEG/IO",
            OptionInfo {
                byte_index: 0,
                bits_start: 1,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (0, "P0.0~P0.4,P1.0~P1.2 output 40mA current when VDD-3.0V"),
                    (1, "P0.0~P0.4,P1.0~P1.2 output30mA current when VDD-3.0V"),
                    (2, "P0.0~P0.4,P1.0~P1.2 output 20mA current when VDD-3.0V"),
                    (3, "P0.0~P0.4,P1.0~P1.2 output 10mA current when VDD-3.0V"),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 1,
                bits_start: 5,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable LVR function"),
                    (1, "Enable LVR function"),
                    (0, "4.1V LVR level 1"),
                    (1, "3.7V LVR level 2"),
                    (2, "2.8V LVR level 3"),
                ]),
            },
        ),
        (
            "OP_SCM",
            OptionInfo {
                byte_index: 1,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(0, "Disable SCM function"), (1, "Enable SCM function")]),
            },
        ),
        (
            "OP_COM/IO",
            OptionInfo {
                byte_index: 1,
                bits_start: 1,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([
                    (0, "Enable P1.3~P1.7 sink current buffer"),
                    (1, "Disable sink current buffer"),
                ]),
            },
        ),
        (
            "OP_OSC",
            OptionInfo {
                byte_index: 2,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (
                        0,
                        "Internal RC OSC (16.6MHz), XTAL1 and XTAL2 shared with IO",
                    ),
                    (1, "External Crystal OSC (2M~12MHz)"),
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
                states: IndexMap::from([(0, "Enable ISP function"), (1, "Disable ISP function")]),
            },
        ),
        (
            "OP_ISPPIN",
            OptionInfo {
                byte_index: 3,
                bits_start: 6,
                bits_end: 6,
                editable: false,
                states: IndexMap::from([
                    (0, "Don't detect P2.6 and P2.7 enter ISP mode"),
                    (1, "When P2.6 and P2.7 are low enter ISP mode"),
                ]),
            },
        ),
        (
            "OP_OVL",
            OptionInfo {
                byte_index: 3,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "WDT reset when OVL"), (1, "OVL interrupt when OVL")]),
            },
        ),
        (
            "OP_OSC_DRIVE",
            OptionInfo {
                byte_index: 3,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(0, "4MHz crystal"), (1, "8M~12MHz crystal")]),
            },
        ),
        (
            "OP_FEED_RES",
            OptionInfo {
                byte_index: 3,
                bits_start: 1,
                bits_end: 2,
                editable: false,
                states: IndexMap::from([
                    (0, "2M Resistance"),
                    (1, "1M Resistance"),
                    (2, "500K Resistance"),
                    (3, "300K Resistance"),
                ]),
            },
        ),
    ])
}
