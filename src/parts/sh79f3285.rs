// Auto-generated from GPT file for SH79F3285

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f3285000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 32768,
    eeprom_size: 2048,
    default_code_options: &hex!("0200008c"),
    code_option_mask: &hex!("ffffff7f"),
    jtag_id: 0x3283,
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
                (0, "Enable WDT function"),
                (1, "Disable WDT function"),
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
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator1 is internal 12M RC, oscillator2 is disabled"),
                (1, "Oscillator1 is internal 8M RC, oscillator2 is disabled"),
                (2, "Oscillator1 is internal 16M RC, oscillator2 is disabled"),
                (3, "Oscillator1 is internal 128k RC, oscillator2 is internal 12M RC"),
                (4, "Oscillator1 is internal 128k RC, oscillator2 is internal 8M RC"),
                (6, "Oscillator1 is internal 128k RC, oscillator2 is 2M-16M cyrstal/ceramic oscillator"),
                (10, "Oscillator1 is 32.768k crystal oscillator, oscillator2 is internal 12M RC"),
                (11, "Oscillator1 is 32.768k crystal oscillator, oscillator2 is internal 8M RC"),
                (13, "Oscillator1 is 32.768k crystal oscillator, oscillator2 is 2M-16M cyrstal/ceramic oscillator"),
                (14, "Oscillator1 is 2M-16M cyrstal/ceramic oscillator, oscillator2 is disabled"),
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
                (0, "4.1V LVR level 1"),
                (1, "3.7V LVR level 2"),
                (2, "2.8V LVR level 3"),
                (3, "2.1V LVR level 4"),
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
        ("OP_MODSW", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "LCD/LED Counter Run, if MODSW is 1"),
                (1, "LCD/LED Counter stop and data is preserved,if MODSW is 1"),
            ]),
        }),
        ("OP_OSCDRIVE", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (3, "8M-16M Crystal"),
                (1, "4M crystal"),
                (7, "12M ceramic"),
                (5, "8M ceramic"),
                (6, "4M ceramic"),
                (4, "2M ceramic"),
            ]),
        }),
        ("OP_PORTDRIVE", OptionInfo {
            byte_index: 2,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "All Port drive ability normal mode(not including P3)"),
                (1, "All Port drive ability large mode(not including P3)"),
            ]),
        }),
        ("OP_P1DRIVE", OptionInfo {
            byte_index: 2,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "P1 drive ability normal mode"),
                (1, "P1 drive ability large mode"),
            ]),
        }),
        ("OP_P33-P30", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Port3[3:0]sink ability normal mode"),
                (1, "Port3[3:0]sink ability large mode"),
            ]),
        }),
        ("OP_P37-P34", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "port3[7:4]sink ability normal mode"),
                (1, "port3[7:4]sink ability large mode"),
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
        ("OP_TF3", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Timer3 interrupt flag cleared by hardware(can't used with UART1 together)"),
                (1, "Timer3 interrupt flag cleared by software (can used with UART1 together)"),
            ]),
        }),
        ("OP_OSCRFB", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "SET OSC Feadback resistor 2M"),
                (1, "SET OSC Feadback resistor 1M"),
                (2, "SET OSC Feadback resistor 500K"),
                (3, "SET OSC Feadback resistor 300K"),
            ]),
        }),
    ])
}
