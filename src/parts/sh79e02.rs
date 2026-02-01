// Auto-generated from GPT file for SH79E02

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79e0200000"),
    chip_type: 0x01,
    custom_block: 0x01,
    product_block: 0x01,
    flash_size: 2048,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("ffff0000"),
    jtag_id: 0xe020,
    sector_size: 512,
    option_byte_count: 4,
    security_level: 0,
    bank_type: 0,
    customer_id: AddressField { address: 0x07c0 },
    operation_number: AddressField { address: 0x07c4 },
    customer_option: AddressField { address: 0x07c6 },
    security: AddressField { address: 0x07ca },
    serial_number: AddressField { address: 0x07fc },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_OSC",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (0, "Internal RC oscillator(8M)"),
                    (2, "External clock(400kHz - 8MHz)"),
                    (3, "32.768kHz Crystal Oscillator"),
                    (5, "Crystal/Ceramic Oscillator (500kHz - 8MHz)"),
                    (6, "455kHz Ceramic Oscillator"),
                ]),
            },
        ),
        (
            "OP_OP1",
            OptionInfo {
                byte_index: 0,
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([(0, "Disable OP1"), (1, "Enable OP1")]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 0,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_LVRLE",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "2.6V LVR LEVEL1"), (1, "4.0V LVR LEVEL2")]),
            },
        ),
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(0, "Disable WDT function"), (1, "Enable WDT function")]),
            },
        ),
        (
            "OP_OP2",
            OptionInfo {
                byte_index: 0,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Disable OP2"), (1, "Enable OP2")]),
            },
        ),
        (
            "OP_WDTPD",
            OptionInfo {
                byte_index: 1,
                bits_start: 2,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in Power-Down mode"),
                    (1, "Enable WDT function in Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_WMT",
            OptionInfo {
                byte_index: 1,
                bits_start: 0,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([
                    (0, "Longest warm up time"),
                    (1, "Longer warm up time"),
                    (2, "Shorter warm up time"),
                    (3, "Shortest warm up time"),
                ]),
            },
        ),
        (
            "OP_RST",
            OptionInfo {
                byte_index: 1,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(0, "P3.3 used as RST pin"), (1, "P3.3 used as I/O pin")]),
            },
        ),
    ])
}
