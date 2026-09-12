// Auto-generated from GPT file for SH4225

use super::{AddressField, OptionInfo, Options, Part, SecurityRecordFormat, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("32f4225000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 245760,
    eeprom_size: 3072,
    default_code_options: &hex!("0100000000000000"),
    code_option_mask: &hex!("0103000000000000"),
    jtag_id: 0x4f25,
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
            "OP_MCMPIN",
            OptionInfo {
                byte_index: 1,
                bits_start: 0,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([
                    (0, "PC4-PC9:PWM21/PWM11/PWM01/PWM2/PWM1/PWM0"),
                    (1, "PC4-PC9:PWM0/PWM1/PWM2/PWM01/PWM11/PWM21"),
                    (2, "PC4-PC9:PWM21/PWM2/PWM11/PWM1/PWM01/PWM0"),
                    (3, "PC4-PC9:PWM0/PWM01/PWM1/PWM11/PWM2/PWM21"),
                ]),
            },
        ),
        (
            "OP_CRYCAP",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (0, "Crystal frequency is lower than 10Mhz"),
                    (1, "Crystal frequency is greater than 10Mhz"),
                ]),
            },
        ),
    ])
}
