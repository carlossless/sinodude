// Auto-generated from GPT file for SH32F9B00

use super::{SecurityRecordFormat, AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("32f9b00000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 523264,
    eeprom_size: 8192,
    default_code_options: &hex!("0000000000000000"),
    code_option_mask: &hex!("c000000000000000"),
    jtag_id: 0x39b0,
    sector_size: 1024,
    option_byte_count: 8,
    security_level: 10,
    bank_type: 0,
    single_wire: false,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0xfffe000 },
    serial_number: AddressField { address: 0x0000 },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
    erase_full_mode: 5,
    option_high_addr: None,
    security_record_format: SecurityRecordFormat::Record19,
    isp_password_addr: None,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([(
        "OP_MCM1PIN",
        OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 1,
            editable: false,
            states: IndexMap::from([
                (0, "PB13-PB8: PWM21/PWM11/PWM01/PWM2/PWM1/PWM0"),
                (1, "PB13-PB8: PWM0/PWM1/PWM2/PWM01/PWM11/PWM21"),
                (2, "PB13-PB8: PWM21/PWM2/PWM11/PWM1/PWM01/PWM0"),
                (3, "PB13-PB8: PWM0/PWM01/PWM1/PWM11/PWM2/PWM21"),
            ]),
        },
    )])
}
