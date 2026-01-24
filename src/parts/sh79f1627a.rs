// Auto-generated from GPT file for SH79F1627A

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
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable WDT function"),
                (1, "Disable WDT function"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the Power-Down mode"),
                (1, "Enable WDT function in the Power-Down mode"),
            ]),
        }),
        ("OP_WMT", OptionInfo {
            byte_index: 0,
            bits_start: 3,
            bits_end: 4,
            editable: false,
            states: IndexMap::from([
                (0, "longest warm up time"),
                (1, "longer warm up time"),
                (2, "shorter warm up time"),
                (3, "shortest warm up time"),
            ]),
        }),
        ("OP_SEG/IO", OptionInfo {
            byte_index: 0,
            bits_start: 1,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "P0.0~P0.4, P1.0~P1.2 output 40mA current when VDD=3.0V"),
                (1, "P0.0~P0.4, P1.0~P1.2 output 30mA current when VDD=3.0V"),
                (2, "P0.0~P0.4, P1.0~P1.2 output 20mA current when VDD=3.0V"),
                (3, "P0.0~P0.4, P1.0~P1.2 output 10mA current when VDD=3.0V"),
            ]),
        }),
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
            bits_start: 5,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "4.1V LVR Level 1"),
                (1, "3.7V LVR Level 2"),
                (2, "2.8V LVR Level 3"),
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
        ("OP_COM/IO", OptionInfo {
            byte_index: 1,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Port1[7:3] sink ability large mode"),
                (1, "Port1[7:3] sink ability normal mode"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Internal 16.6MHz RC oscillator"),
                (1, "2M-12MHz Crystal oscillator"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 3,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "Enable ISP function"),
                (1, "Disable ISP function"),
            ]),
        }),
        ("OP_ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: false,
            states: IndexMap::from([
                (0, "Enter ISP mode directly regardless the condition of P2.6 and P2.7"),
                (1, "Enter ISP mode only when P2.6 and P2.7 are connected to GND simultaneously"),
            ]),
        }),
        ("OP_OVL", OptionInfo {
            byte_index: 3,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "OVL generates WDT Reset"),
                (1, "OVL generates OVL interrupt"),
            ]),
        }),
        ("OP_OSC_DRIVE", OptionInfo {
            byte_index: 3,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "4M crystal"),
                (1, "8M-12M crystal"),
            ]),
        }),
        ("OP_FEED_RES", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 2,
            editable: false,
            states: IndexMap::from([
                (0, "2M"),
                (1, "1M"),
                (2, "500K"),
                (3, "300K"),
            ]),
        }),
    ])
}
