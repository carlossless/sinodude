// Auto-generated from GPT file for SH79F1621A

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f1621a00"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 16384,
    eeprom_size: 1024,
    default_code_options: &hex!("0000b086"),
    code_option_mask: &hex!("f8faff2f"),
    jtag_id: 0x166b,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 5,
    bank_type: 1,
    customer_id: AddressField { address: 0x0800 },
    operation_number: AddressField { address: 0x0804 },
    customer_option: AddressField { address: 0x0806 },
    security: AddressField { address: 0x080a },
    serial_number: AddressField { address: 0x083c },
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
                (0, "Disable WDT function"),
                (1, "Enable WDT function"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in Power-Down mode"),
                (1, "Enable WDT function in Power-Down mode"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "P5.2 used as RST pin"),
                (1, "P5.2 used as I/O pin"),
            ]),
        }),
        ("OP_WMT", OptionInfo {
            byte_index: 0,
            bits_start: 3,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "longest warm up time"),
                (1, "longer warm up time"),
                (2, "shorter warm up time"),
                (3, "shortest warm up time"),
            ]),
        }),
        ("OP_LVREN", OptionInfo {
            byte_index: 1,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable LVR function"),
                (1, "Enable LVR function"),
            ]),
        }),
        ("OP_LVRLE", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "4.3V LVR Level 1"),
                (1, "3.7V LVR Level 2"),
                (2, "2.1V LVR Level 3"),
                (3, "2.8V LVR Level 4"),
            ]),
        }),
        ("OP_SCMEN", OptionInfo {
            byte_index: 1,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "Enable SCM"),
                (1, "Disable SCM"),
            ]),
        }),
        ("OP_SCM", OptionInfo {
            byte_index: 1,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "SCM is invalid in warm up period"),
                (1, "SCM is valid in warm up period"),
            ]),
        }),
        ("OP_LEDCOM", OptionInfo {
            byte_index: 1,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "LED common signal is normal"),
                (1, "LED common signal is inverted"),
            ]),
        }),
        ("OP_PORTDRIVE", OptionInfo {
            byte_index: 2,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Port Drive ability normal mode(without P3 port)"),
                (1, "Port Drive ability large mode(without P3 port)"),
            ]),
        }),
        ("OP_SCMSEL", OptionInfo {
            byte_index: 2,
            bits_start: 4,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "2MHz"),
                (1, "4MHz"),
                (2, "6MHz"),
                (3, "8MHz"),
                (4, "12MHz"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator1 is internal 12M RC, oscillator2 is disabled"),
                (3, "Oscillator1 is internal 128K RC, oscillator2 is internal 12M RC"),
                (6, "Oscillator1 is internal 128K RC, oscillator2 is 2M-12M crystal/ceramic oscillator"),
                (10, "Oscillator1 is 32.768k crystal oscillator, oscillator2 is internal 12M RC"),
                (14, "Oscillator1 is 2M-12M crystal/ceramic oscillator, oscillator2 is disabled"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 3,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "Enable ISP function"),
                (1, "Disable ISP function"),
            ]),
        }),
        ("OP_ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: false,
            states: IndexMap::from([
                (0, "Enter ISP mode only when P0.4 and P0.5 are connected to GND, simultaneously"),
                (1, "Enter ISP mode directly regardless the condition of P0.4 and P0.5"),
            ]),
        }),
        ("OP_32KCAP", OptionInfo {
            byte_index: 3,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Disable 32kHz Crystal built-in 7pF capacity"),
                (1, "Enable 32kHz Crystal built-in 7pF capacity"),
            ]),
        }),
        ("OP_32KBIAS", OptionInfo {
            byte_index: 3,
            bits_start: 4,
            bits_end: 4,
            editable: false,
            states: IndexMap::from([
                (0, "Disable 32kHz Crystal bias resistor"),
                (1, "Enable 32kHz Crystal bias resistor"),
            ]),
        }),
        ("OP_P54EN", OptionInfo {
            byte_index: 3,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Disable P5.4 function"),
                (1, "Enable P5.4 function"),
            ]),
        }),
        ("OP_CRY_DR", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Crystal drive capability is Normal"),
                (1, "Crystal drive capability is Larger"),
            ]),
        }),
        ("OP_P3.3-P3.0", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Port3[3:0] sink ability normal mode"),
                (1, "Port3[3:0] sink ability large mode"),
            ]),
        }),
        ("OP_P3.7-P3.4", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Port3[7:4] sink ability large mode"),
                (1, "Port3[7:4] sink ability normal mode"),
            ]),
        }),
    ])
}
