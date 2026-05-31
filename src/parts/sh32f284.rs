// Auto-generated from GPT file for SH32F284

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("32f2840000"),
    chip_type: 0x00,
    custom_block: 0x00,
    product_block: 0x00,
    flash_size: 131072,
    eeprom_size: 6144,
    default_code_options: &hex!("1500000000000000"),
    code_option_mask: &hex!("ff53000000000080"),
    jtag_id: 0x3f25,
    sector_size: 2048,
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
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        ("OP_IWDT", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (5, "Disable WDT function"),
                (0, "Enable WDT function"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "Disable IWDT function in the Power-Down mode"),
                (1, "Enable IWDT function in the Power-Down mode"),
            ]),
        }),
        ("OP_LVREN", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function"),
                (1, "Enable LVR function"),
            ]),
        }),
        ("OP_LVRLE", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "4.1V LVR Level"),
                (1, "3.7V LVR Level"),
                (2, "2.5V LVR Level"),
            ]),
        }),
        ("OP_MCM1PIN", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "PB8-PB13: PWM21/PWM11/PWM01/PWM2/PWM1/PWM0"),
                (1, "PB8-PB13: PWM0/PWM1/PWM2/PWM01/PWM11/PWM21"),
                (2, "PB8-PB13: PWM21/PWM2/PWM11/PWM1/PWM01/PWM0"),
                (3, "PB8-PB13: PWM0/PWM01/PWM1/PWM11/PWM2/PWM21"),
            ]),
        }),
        ("OP_MCM2PIN", OptionInfo {
            byte_index: 1,
            bits_start: 2,
            bits_end: 3,
            editable: false,
            states: IndexMap::from([
                (0, "PB14-PC3 or PA15-PA10: PWM21/PWM11/PWM01/PWM2/PWM1/PWM0"),
                (1, "PB14-PC3 or PA15-PA10: PWM0/PWM1/PWM2/PWM01/PWM11/PWM21"),
                (2, "PB14-PC3 or PA15-PA10: PWM21/PWM2/PWM11/PWM1/PWM01/PWM0"),
                (3, "PB14-PC3 or PA15-PA10: PWM0/PWM01/PWM1/PWM11/PWM2/PWM21"),
            ]),
        }),
        ("OP_CSM", OptionInfo {
            byte_index: 1,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "CSM Mode is on after power on reset"),
                (1, "CSM Mode is off after power on reset"),
            ]),
        }),
        ("OP_HSE16M", OptionInfo {
            byte_index: 1,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable high-frequency 16M HSE"),
                (1, "Enable high-frequency 16M HSE"),
            ]),
        }),
        ("OP_FLSALG", OptionInfo {
            byte_index: 1,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "Use BRCHSTAT as an accelerated basis"),
                (1, "Disuse BRCHSTAT as an accelerated basis"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 7,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable ISP function"),
                (1, "Disable ISP function"),
            ]),
        }),
    ])
}
