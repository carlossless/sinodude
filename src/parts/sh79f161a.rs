// Auto-generated from GPT file for SH79F161A

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f161a000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 2048,
    default_code_options: &hex!("000000a0"),
    code_option_mask: &hex!("7f00003f"),
    jtag_id: 0x161a,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 0,
    customer_id: AddressField { address: 0x0800 },
    operation_number: AddressField { address: 0x0804 },
    customer_option: AddressField { address: 0x0806 },
    security: AddressField { address: 0x080a },
    serial_number: AddressField { address: 0x083c },
    compatible_voltages: &[Voltage::V5_0],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        ("OP_OSC", OptionInfo {
            byte_index: 0,
            bits_start: 1,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "Internal RC oscillator, XTAL1 and XTAL2 shared with IO"),
                (14, "400k ~ 16M crystal oscillator  or ceramic oscillator"),
            ]),
        }),
        ("OP_CRMC", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "2M-16M oscillator is used"),
                (1, "400K-2M oscillator is used"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 3,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "P1.7 used as RST pin"),
                (1, "P1.7 used as I/O pin"),
            ]),
        }),
        ("OP_LVREN", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function"),
                (1, "Enable LVR function"),
            ]),
        }),
        ("OP_LVRLE", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "4.3V LVR Level 1"),
                (1, "3.7V LVR Level 2"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function"),
                (1, "Enable WDT function"),
            ]),
        }),
        ("OP_ WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the Power-Down mode even if OP_WDT is enabled"),
                (1, "Enable WDT function in the Power-Down mode if OP_WDT is enabled"),
            ]),
        }),
        ("OP_WMT", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "longest warm up time"),
                (1, "longer warm up time"),
                (2, "shorter warm up time"),
                (3, "shortest warm up time"),
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
        ("OP_ ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: false,
            states: IndexMap::from([
                (0, "Enter ISP mode only when P0.4 and P0.5 are connected to GND, simultaneously"),
                (1, "Enter ISP mode directly regardless the condition of P0.4 and P0.5"),
            ]),
        }),
    ])
}
