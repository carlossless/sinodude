// Auto-generated from GPT file for SH88F6162

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("88f6162000"),
    chip_type: 0x02,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 2048,
    default_code_options: &hex!("0020030c"),
    code_option_mask: &hex!("f8e8cfdf"),
    jtag_id: 0x6161,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 3,
    bank_type: 1,
    customer_id: AddressField { address: 0x1000 },
    operation_number: AddressField { address: 0x1004 },
    customer_option: AddressField { address: 0x1006 },
    security: AddressField { address: 0x100a },
    serial_number: AddressField { address: 0x103c },
    options,
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
                (0, "Disable WDT function.(default)"),
                (1, "Enable WDT function."),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the Power-Down mode.(Default)"),
                (1, "Enable WDT function in the Power-Down mode."),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Pin P7.0 used as RST pin.(Default)"),
                (1, "Pin P7.0 used as I/O pin."),
            ]),
        }),
        ("OP_LVREN", OptionInfo {
            byte_index: 1,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function. (Default)"),
                (1, "Enable LVR function."),
            ]),
        }),
        ("OP_LVRLEVEL", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "4.1V LVR Level 1"),
                (1, "3.7V LVR Level 2 (Default)"),
                (2, "2.7V LVR Level 3"),
                (3, "2.1V LVR Level 4"),
            ]),
        }),
        ("OP_SCM", OptionInfo {
            byte_index: 1,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "SCM is invalid in warm up period(default)"),
                (1, "SCM is valid in warm up period"),
            ]),
        }),
        ("OP_SINK", OptionInfo {
            byte_index: 2,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "P1 sink ability normal mode(default)"),
                (1, "P1 sink ability large mode"),
            ]),
        }),
        ("OP_DRIVE", OptionInfo {
            byte_index: 2,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "P3 drive ability normal mode(default)"),
                (1, "P3 drive ability large mode"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (3, "Internal RC( 128KHz )and Internal RC (12MHz)(default)"),
                (10, "32.768kHz crystal oscillator and Internal RC (12MHz)"),
                (6, "Internal RC(128K)and 2M~12M crystal oscillator"),
                (13, "32.768kHz crystal oscillator and 2M~12M crystal oscillator"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 3,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable ISP function. (default)"),
                (1, "Disable ISP function."),
            ]),
        }),
        ("OP_ ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Enter ISP mode directly regardless the condition of P3.4 and P3.5. (default)"),
                (1, "Enter ISP mode only when P3.4 and P3.5 are connected to GND, simultaneously."),
            ]),
        }),
        ("OP_WMT", OptionInfo {
            byte_index: 0,
            bits_start: 3,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "longest warm up time(default)"),
                (1, "longer warm up time"),
                (2, "shorter warm up time"),
                (3, "shortest warm up time"),
            ]),
        }),
        ("OP_OSCRFB", OptionInfo {
            byte_index: 2,
            bits_start: 4,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "Feedback Resistor is 2M ohm.(default)"),
                (1, "Feedback Resistor is 1M ohm"),
                (2, "Feedback Resistor is 500K ohm"),
                (3, "Feedback Resistor is 300K ohm"),
            ]),
        }),
        ("OP_OVL", OptionInfo {
            byte_index: 3,
            bits_start: 5,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "OVL generates WDT Reset (default)"),
                (1, "OVL generates OVL interrupt."),
            ]),
        }),
        ("OP_OSCDRV", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (3, "8M~12M crystal.(default)"),
                (1, "4M crystal"),
                (7, "12M ceramic"),
                (5, "8M ceramic"),
                (6, "4M ceramic"),
                (4, "2M ceramic"),
            ]),
        }),
        ("OP_DACTIME", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "100 us(default)"),
                (1, "70   us"),
                (2, "150 us"),
                (3, "200 us"),
            ]),
        }),
    ])
}
