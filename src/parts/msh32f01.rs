// Auto-generated from GPT file for MSH32F01

use super::{AddressField, OptionInfo, Options, Part, SecurityRecordFormat, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("33f2801000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 131072,
    eeprom_size: 4096,
    default_code_options: &hex!("1500000000000080"),
    code_option_mask: &hex!("1f07030000000080"),
    jtag_id: 0x3f26,
    sector_size: 1024,
    option_byte_count: 8,
    security_level: 10,
    bank_type: 0,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0xfffe000 },
    serial_number: AddressField { address: 0x0000 },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
    security_record_format: SecurityRecordFormat::Record19,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_IWDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(5, "Disable WDT function"), (0, "Enable WDT function")]),
            },
        ),
        (
            "OP_WDTPD",
            OptionInfo {
                byte_index: 0,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable IWDT function in the Power-Down mode"),
                    (1, "Enable IWDT function in the Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_MCMPIN",
            OptionInfo {
                byte_index: 2,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (0, "PB1-PB6: No output"),
                    (1, "PB1-PB6: PWM21/PWM11/PWM01/PWM2/PWM1/PWM0"),
                    (2, "PB1-PB6: PWM0/PWM1/PWM2/PWM01/PWM11/PWM21"),
                    (3, "PB1-PB6: PWM21/PWM2/PWM11/PWM1/PWM01/PWM0"),
                    (4, "PB1-PB6: PWM0/PWM01/PWM1/PWM11/PWM2/PWM21"),
                    (0, "Flash is not divided into blocks"),
                    (
                        1,
                        "Flash is divided into two blocks, Main Memory and Backup Memory",
                    ),
                    (0, "PC pointer addresses from Main Memory"),
                    (1, "PC pointer addresses from Backup Memory"),
                ]),
            },
        ),
        (
            "OP_ISP",
            OptionInfo {
                byte_index: 7,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Enable ISP function"), (1, "Disable ISP function")]),
            },
        ),
    ])
}
