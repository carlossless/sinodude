// Auto-generated from GPT file for SH77P1652

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("0000000000"),
    chip_type: 0x06,
    custom_block: 0x06,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("fe070000"),
    jtag_id: 0xf751,
    sector_size: 4096,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 0,
    customer_id: AddressField { address: 0x3fe0 },
    operation_number: AddressField { address: 0x3fe4 },
    customer_option: AddressField { address: 0x3fe6 },
    security: AddressField { address: 0x3fea },
    serial_number: AddressField { address: 0x3ffc },
    compatible_voltages: &[Voltage::V3_3],
    options,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Enable WDT function"), (1, "Disable WDT function")]),
            },
        ),
        (
            "OP_ WDTPD",
            OptionInfo {
                byte_index: 0,
                bits_start: 6,
                bits_end: 6,
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
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_SCMEN",
            OptionInfo {
                byte_index: 0,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([(0, "Disable SCM function"), (1, "Enable SCM function")]),
            },
        ),
        (
            "OP_SCM",
            OptionInfo {
                byte_index: 0,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable SCM in warm up"),
                    (1, "Enable SCM in warm up"),
                ]),
            },
        ),
        (
            "OP_RST",
            OptionInfo {
                byte_index: 0,
                bits_start: 2,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (0, "Pin P5.5 used as RST pin"),
                    (1, "Pin P5.5 used as I/O pin"),
                ]),
            },
        ),
        (
            "OP_OSC",
            OptionInfo {
                byte_index: 0,
                bits_start: 1,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([
                    (
                        0,
                        "Internal RC OSC1 (32KHz)and OSC2 (4MHz), XTAL1 and XTAL2 shared with IO",
                    ),
                    (1, "External OSC1(32.768KHz),Internal RC OSC2 (4MHz)"),
                ]),
            },
        ),
        (
            "OP_REM_Current",
            OptionInfo {
                byte_index: 1,
                bits_start: 1,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([(0, "125mA"), (1, "250mA"), (2, "375mA"), (3, "500mA")]),
            },
        ),
        (
            "OP_LCDSEL",
            OptionInfo {
                byte_index: 1,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (0, "Select resistive LCD driver"),
                    (1, "Select capacitive LCD driver"),
                ]),
            },
        ),
    ])
}
