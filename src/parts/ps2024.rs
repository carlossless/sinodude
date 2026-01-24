// Auto-generated from GPT file for PS2024

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f1633000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 512,
    default_code_options: &hex!("00010088"),
    code_option_mask: &hex!("fff9001b"),
    jtag_id: 0x083a,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
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
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function(Default)"),
                (1, "Enable WDT function"),
            ]),
        }),
        ("OP_ WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the Power-Down mode(Default)"),
                (1, "Enable WDT function in the Power-Down mode"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Pin P4.0 used as RST pin(Default)"),
                (1, "Pin P4.0 used as I/O pin"),
            ]),
        }),
        ("OP_WMT", OptionInfo {
            byte_index: 0,
            bits_start: 3,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "longest warm up time(Default)"),
                (1, "longer warm up time"),
                (2, "shorter warm up time"),
                (3, "shortest warm up time"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Internal RC oscillator (16.6MHz)(Default)"),
                (2, "External clock source (30kHz-16.6MHz)"),
                (3, "32.768kHz crystal oscillator, Internal RC oscillator (16.6MHz) can be enabled"),
                (5, "Crystal oscillator(4MHz-16.6MHz)"),
                (6, "Ceramic oscillator(2MHz-16.6MHz)"),
            ]),
        }),
        ("OP_LVREN", OptionInfo {
            byte_index: 1,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function(Default)"),
                (1, "Enable LVR function"),
            ]),
        }),
        ("OP_LVRLE", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "4.1V LVR Level 1(Default)"),
                (2, "3.7V LVR Level 2"),
                (1, "2.8V LVR Level 3"),
            ]),
        }),
        ("OP_SCM", OptionInfo {
            byte_index: 1,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "SCM is invalid in warm up period(Default)"),
                (1, "SCM is valid in warm up period"),
            ]),
        }),
        ("OP_IO", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "IO is Input Only mode when reset occurs(Default)"),
                (1, "IO is Quasi-Bi mode when reset occurs"),
            ]),
        }),
        ("OP_OSCDRV", OptionInfo {
            byte_index: 3,
            bits_start: 3,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (1, "Middle(Default)"),
                (2, "Maximum"),
            ]),
        }),
        ("OP_P3", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "P3 sink ability normal mode(Default)"),
                (1, "P3 sink ability large mode"),
            ]),
        }),
        ("OP_P1P4", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "P1/P4 dirver ability normal mode(Default)"),
                (1, "P1/P4 dirver ability large mode"),
            ]),
        }),
        ("OP_LPDFLAG", OptionInfo {
            byte_index: 1,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "LPD FLAG cannot be set when P1.3 used as analog input pin and LPD detect voltage is select as VDD(Default)"),
                (1, "LPD FLAG can be set when P1.3 used as analog input pin and LPD detect voltage is select as VDD"),
            ]),
        }),
        ("OP_OSCRFB", OptionInfo {
            byte_index: 1,
            bits_start: 1,
            bits_end: 2,
            editable: false,
            states: IndexMap::from([
                (0, "230K(Default)"),
                (1, "150K"),
                (2, "500K"),
                (3, "1M"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 3,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "Enable ISP function(Default)"),
                (1, "Disable ISP function"),
            ]),
        }),
        ("OP_ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: false,
            states: IndexMap::from([
                (0, "Enter ISP mode only when P3.4 and P3.5 are connected to GND,simultaneously(Default)"),
                (1, "Enter ISP mode directly regardless the condition of P3.4 and P3.5"),
            ]),
        }),
        ("OP_OVL", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 2,
            editable: false,
            states: IndexMap::from([
                (0, "OVL generates WDT Reset(Default)"),
                (1, "OVL generates OVL interrupt"),
                (0, "Disable Protect(Default)"),
                (1, "Enable Protect"),
            ]),
        }),
    ])
}
