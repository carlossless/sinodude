// Auto-generated from GPT file for 77P1654A-16K*1

use super::{SecurityRecordFormat, AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("77f165a000"),
    chip_type: 0x06,
    custom_block: 0x04,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 0,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("fe1f0000"),
    jtag_id: 0xf751,
    sector_size: 2048,
    option_byte_count: 4,
    security_level: 1,
    bank_type: 0,
    single_wire: false,
    customer_id: AddressField { address: 0x0000 },
    operation_number: AddressField { address: 0x0000 },
    customer_option: AddressField { address: 0x0000 },
    security: AddressField { address: 0x0000 },
    serial_number: AddressField { address: 0x0000 },
    compatible_voltages: &[Voltage::V3_3],
    options,
    erase_full_mode: 5,
    option_high_addr: None,
    security_record_format: SecurityRecordFormat::Record19,
    isp_password_addr: None,
};

/// Get all code options metadata
pub fn options() -> Options {
    IndexMap::from([
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable WDT function"),
                (1, "Disable WDT function"),
            ]),
        }),
        ("OP_ WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the Power-Down mode"),
                (1, "Enable WDT function in the Power-Down mode"),
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
        ("OP_SCMEN", OptionInfo {
            byte_index: 0,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "Disable SCM function"),
                (1, "Enable SCM function"),
            ]),
        }),
        ("OP_SCM", OptionInfo {
            byte_index: 0,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Disable SCM in warm up"),
                (1, "Enable SCM in warm up"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Pin P5.5 used as RST pin"),
                (1, "Pin P5.5 used as I/O pin"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 0,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Internal RC OSC1 (32KHz)and OSC2 (4MHz), XTAL1 and XTAL2 shared with IO"),
                (1, "External OSC1(32.768KHz),Internal RC OSC2 (4MHz)"),
            ]),
        }),
        ("OP_OVL", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 0,
            editable: false,
            states: IndexMap::from([
                (0, "OVL generates WDT reset"),
                (1, "OVL generates OVL interrupt"),
            ]),
        }),
        ("OP_C32K_ESD", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "Disable C32K ESD Buffer Bias circuit"),
                (1, "Enable C32K ESD Buffer Bias circuit"),
            ]),
        }),
        ("OP_AHUMC", OptionInfo {
            byte_index: 1,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "Enable built_in 7PF matching capacitors"),
                (1, "Disable built_in 7PF matching capacitors"),
            ]),
        }),
        ("OP_AHUM", OptionInfo {
            byte_index: 1,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Enable C32 humidity resistance function"),
                (1, "Disable C32 humidity resistance function"),
            ]),
        }),
        ("OP_REM_CURRENT", OptionInfo {
            byte_index: 1,
            bits_start: 1,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "125mA"),
                (1, "250mA"),
                (2, "375mA"),
                (3, "500mA"),
            ]),
        }),
        ("OP_LCDSEL", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Select resistive LCD driver"),
                (1, "Select capacitive LCD driver"),
            ]),
        }),
    ])
}
