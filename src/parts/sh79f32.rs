// Auto-generated from GPT file for SH79F32

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f3200000"),
    chip_type: 0x00,
    custom_block: 0x01,
    product_block: 0x01,
    flash_size: 32768,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("ff000000"),
    jtag_id: 0x9797,
    sector_size: 2048,
    option_byte_count: 4,
    security_level: 0,
    bank_type: 0,
    customer_id: AddressField { address: 0x7fc0 },
    operation_number: AddressField { address: 0x7fc4 },
    customer_option: AddressField { address: 0x7fc6 },
    security: AddressField { address: 0x7fca },
    serial_number: AddressField { address: 0x7ffc },
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
                    (0, "Internal RC Oscillator(8MHz)"),
                    (2, "External Clock(400kHz-8MHz)"),
                    (3, "32.768kHz Crystal Oscillator"),
                    (5, "Crystal/Ceramic Oscillator(400kHz-8MHz)"),
                ]),
            },
        ),
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Disable WDT function"), (1, "Enable WDT function")]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 0,
                bits_start: 6,
                bits_end: 6,
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
                states: IndexMap::from([(0, "Low LVR Level(2.3V)"), (1, "High LVR Level(3.1V)")]),
            },
        ),
        (
            "OP_RTC",
            OptionInfo {
                byte_index: 0,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([(0, "Method A"), (1, "Method B")]),
            },
        ),
        (
            "OP_LCD",
            OptionInfo {
                byte_index: 0,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(0, "CAP LCD"), (1, "PWM LCD")]),
            },
        ),
    ])
}
