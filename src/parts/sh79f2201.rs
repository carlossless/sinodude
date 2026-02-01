// Auto-generated from GPT file for SH79F2201

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f2201000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 32768,
    eeprom_size: 1024,
    default_code_options: &hex!("0a00030000000086"),
    code_option_mask: &hex!("3fc0f0020000000f"),
    jtag_id: 0x2201,
    sector_size: 512,
    option_byte_count: 8,
    security_level: 4,
    bank_type: 0,
    customer_id: AddressField { address: 0x1000 },
    operation_number: AddressField { address: 0x1004 },
    customer_option: AddressField { address: 0x1006 },
    security: AddressField { address: 0x100a },
    serial_number: AddressField { address: 0x103c },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_RST",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "P3.1 used as RST pin"), (1, "P3.1 used as I/O pin")]),
            },
        ),
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 1,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([(5, "Disable WDT function"), (4, "Enable WDT function")]),
            },
        ),
        (
            "OP_ WDTPD",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in the Power-Down mode"),
                    (1, "Enable WDT function in the Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 1,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_LVRLE",
            OptionInfo {
                byte_index: 1,
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([(0, "4.1V LVR Level 1"), (1, "2.5V LVR Level 2")]),
            },
        ),
        (
            "OP_WAIT_SEL",
            OptionInfo {
                byte_index: 1,
                bits_start: 0,
                bits_end: 1,
                editable: false,
                states: IndexMap::from([(0, "wait 4"), (1, "wait 3"), (2, "wait 5")]),
            },
        ),
        (
            "OP_PWMPIN",
            OptionInfo {
                byte_index: 2,
                bits_start: 6,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([
                    (0, "P2.0~P2.5 as PWM0/PWM1/PWM2/PWM01/PWM11/PWM21"),
                    (1, "P2.0~P2.5 as PWM0/PWM01/PWM1/PWM11/PWM2/PWM21"),
                    (2, "P2.0~P2.5 as PWM21/PWM11/PWM01/PWM2/PWM1/PWM0"),
                    (3, "P2.0~P2.5 as PWM21/PWM2/PWM11/PWM1/PWM01/PWM0"),
                ]),
            },
        ),
        (
            "OP_CAPTPIN",
            OptionInfo {
                byte_index: 2,
                bits_start: 4,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([
                    (0, "P1.4/P1.3/P1.2 as CAP0/CAP1/CAP2"),
                    (1, "P3.6/P3.7/P0.0 as CAP0/CAP1/CAP2"),
                    (2, "P4.0/P4.1/P4.2 as CAP0/CAP1/CAP2"),
                ]),
            },
        ),
        (
            "OP_OSC",
            OptionInfo {
                byte_index: 2,
                bits_start: 0,
                bits_end: 3,
                editable: false,
                states: IndexMap::from([
                    (
                        3,
                        "Oscillator1 is internal 128k RC, oscillator2 is internal 8M RC",
                    ),
                    (15, "Oscillator1 external clock, oscillator2 is disabled"),
                ]),
            },
        ),
        (
            "OP_OVL",
            OptionInfo {
                byte_index: 3,
                bits_start: 5,
                bits_end: 5,
                editable: false,
                states: IndexMap::from([
                    (0, "OVL generates WDT Reset"),
                    (1, "OVL generates OVL interrupt"),
                ]),
            },
        ),
        (
            "OP_PWMDRV",
            OptionInfo {
                byte_index: 3,
                bits_start: 1,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([
                    (0, "Reduce MCM pin output current function disable"),
                    (1, "Reduce MCM pin output current function enable"),
                ]),
            },
        ),
        (
            "OP_ISPSIZE",
            OptionInfo {
                byte_index: 7,
                bits_start: 4,
                bits_end: 7,
                editable: false,
                states: IndexMap::from([
                    (0, "8 x 1024Bytes"),
                    (1, "7 x 1024Bytes"),
                    (2, "6 x 1024Bytes"),
                    (3, "5 x 1024Bytes"),
                    (4, "4 x 1024Bytes"),
                    (5, "3 x 1024Bytes"),
                    (6, "2 x 1024Bytes"),
                    (7, "1 x 1024Bytes"),
                    (8, "0 Bytes"),
                ]),
            },
        ),
        (
            "OP_EEPROMSIZE",
            OptionInfo {
                byte_index: 7,
                bits_start: 0,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([
                    (6, "2 x 512Bytes"),
                    (7, "1 x 512Bytes"),
                    (8, "0 x 512Bytes"),
                ]),
            },
        ),
    ])
}
