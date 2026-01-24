// Auto-generated from GPT file for SH79F1631

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f1631000"),
    chip_type: 0x02,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 2048,
    default_code_options: &hex!("20000080"),
    code_option_mask: &hex!("c0e00720"),
    jtag_id: 0x1631,
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
            editable: false,
            states: IndexMap::from([
                (0, "P1.6 used as RST pin"),
                (1, "P1.6 used as I/O pin"),
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
                (0, "WDT can't work in STOP MODE"),
                (1, "WDT can work  in STOP MODE"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator1 is internal 27M RC, oscillator2 is disabled"),
                (3, "Oscillator1 is 32.768k crystal oscillator, oscillator2 is internal 27M RC"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 3,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "Enable ISP function."),
                (1, "Disable ISP function."),
            ]),
        }),
        ("OP_ ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: false,
            states: IndexMap::from([
                (0, "Enter ISP mode only when P0.2 and P0.3 are connected to GND, simultaneously"),
                (3, "Enter ISP mode directly regardless the condition of P0.2 and P0.3."),
            ]),
        }),
        ("OP_SEG", OptionInfo {
            byte_index: 3,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "P0.0~P0.7,P2.0~P2.7 normal of output current capability."),
                (1, "P0.0~P0.7,P2.0~P2.7, 1/3 of normal of output current capability."),
            ]),
        }),
        ("OP_LEDCOM", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "P1.0-P1.6 sink ability normal mode"),
                (1, "P1.0-P1.6 sink ability large mode"),
            ]),
        }),
    ])
}
