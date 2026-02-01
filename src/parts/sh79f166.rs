// Auto-generated from GPT file for SH79F166

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f1660000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 1024,
    default_code_options: &hex!("80000080"),
    code_option_mask: &hex!("7fce0000"),
    jtag_id: 0xf166,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 1,
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
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "Disable WDT function"),
                (1, "Enable WDT function"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in Power-Down mode"),
                (1, "Enable WDT function in Power-Down mode"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Pin 10 used as RST pin"),
                (1, "Pin 10 used as I/O pin"),
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
        ("OP_OSC", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Internal RC oscillator (12MHz),  OSC and OSCX shared with I/O"),
                (3, "32.768kHz Crystal Oscillator at OSC, Internal RC can be enabled. OSCX shared with I/O"),
                (5, "Crystal oscillator (400kHz - 12MHz) at OSC, OSCX shared with I/O"),
                (6, "Ceramic resonator (400kHz - 12MHz) at OSC, OSCX shared with I/O"),
                (4, "32.768kHz Crystal at OSC, 400kHz - 12MHz Crystal at OSCX"),
                (7, "32.768kHz at OSC, 400kHz - 12MHz Ceramic resonator at OSCX"),
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
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "4.3V LVR level 1"),
                (1, "3.7V LVR level 2"),
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
        ("OP_RLCD", OptionInfo {
            byte_index: 1,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "LCD bias resistor is 20K for 1/3 bias, and 15K for 1/4 bias"),
                (1, "LCD bias resistor is 100K for 1/3 bias, and 75K for 1/4 bias"),
            ]),
        }),
        ("OP_LEDCOM", OptionInfo {
            byte_index: 1,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "LED common signal is normal (default)"),
                (1, "LED common signal is inverted"),
            ]),
        }),
    ])
}
