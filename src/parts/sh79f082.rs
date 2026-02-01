// Auto-generated from GPT file for SH79F082

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f0820000"),
    chip_type: 0x02,
    custom_block: 0x01,
    product_block: 0x01,
    flash_size: 8192,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("ff030000"),
    jtag_id: 0xf168,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 0,
    customer_id: AddressField { address: 0x1fc0 },
    operation_number: AddressField { address: 0x1fc4 },
    customer_option: AddressField { address: 0x1fc6 },
    security: AddressField { address: 0x1fca },
    serial_number: AddressField { address: 0x1ffc },
    compatible_voltages: &[Voltage::V5_0],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_RST",
            OptionInfo {
                byte_index: 0,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "P1.5 used as RST pin"), (1, "P1.5 used as I/O pin")]),
            },
        ),
        (
            "OP_ WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function"),
                    (1, "Always Enable WDT function, Even in Power-Down mode"),
                    (2, "Disable WDT function"),
                    (3, "Enable WDT function, But Disable in Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_WMT",
            OptionInfo {
                byte_index: 0,
                bits_start: 3,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([
                    (0, "Longest warm up time"),
                    (1, "longer warm up time"),
                    (2, "shorter warm up time"),
                    (3, "shortest warm up time"),
                ]),
            },
        ),
        (
            "OP_OSC",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (0, "Internal RC Oscillator(16.6MHz)"),
                    (5, "Crystal Oscillator(400kHz-16.6MHz)"),
                    (6, "Ceramic Oscillator(400kHz-16.6MHz)"),
                ]),
            },
        ),
        (
            "OP_LVR",
            OptionInfo {
                byte_index: 1,
                bits_start: 0,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable LVR function"),
                    (1, "Enable LVR,LVR level=4.3V"),
                    (2, "Enable LVR,LVR level=3.4V"),
                ]),
            },
        ),
    ])
}
