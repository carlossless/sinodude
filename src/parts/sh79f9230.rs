// Auto-generated from GPT file for SH79F9230

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f9230000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 32768,
    eeprom_size: 4096,
    default_code_options: &hex!("200000c000000080"),
    code_option_mask: &hex!("00e10f010000ff0f"),
    jtag_id: 0x9228,
    sector_size: 512,
    option_byte_count: 8,
    security_level: 4,
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
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "P1.0 used as RST pin"),
                (1, "P1.0 used as I/O pin"),
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
                (0, "4.1V LVR Level 1"),
                (1, "3.7V LVR Level 2"),
                (2, "3.1V LVR Level 3"),
            ]),
        }),
        ("OP_P06-P07", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Port0[7:6] sink ability normal mode"),
                (1, "Port0[7:6] sink ability large mode"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator1 is internal 24M RC, oscillator2 is disabled"),
                (3, "Oscillator1 is internal 128k RC, oscillator2 is internal 24M RC"),
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
                (0, "Enter ISP mode directly regardless the condition of P1.0 and P1.1"),
                (1, "Enter ISP mode only when P1.0 and P1.1 are connected to GND, simultaneously"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in Power-Down mode"),
                (1, "Enable WDT function in Power-Down mode"),
            ]),
        }),
        ("OP_PPGO", OptionInfo {
            byte_index: 5,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "output Active level is low"),
                (1, "output Active level is high"),
            ]),
        }),
        ("OP_PPGGO Limit", OptionInfo {
            byte_index: 5,
            bits_start: 0,
            bits_end: 1,
            editable: false,
            states: IndexMap::from([
                (0, "2048*tosc"),
                (1, "1280*tosc"),
                (2, "768*tosc"),
                (3, "512*tosc"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 6,
            bits_start: 0,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (165, "Disable WDT function"),
                (0, "Enable WDT function"),
            ]),
        }),
        ("OP_ISPSIZE", OptionInfo {
            byte_index: 7,
            bits_start: 4,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "8x1024 Bytes"),
                (1, "7x1024 Bytes"),
                (2, "6x1024 Bytes"),
                (3, "5x1024 Bytes"),
                (4, "4x1024 Bytes"),
                (5, "3x1024 Bytes"),
                (6, "2x1024 Bytes"),
                (7, "1x1024 Bytes"),
                (8, "0 Bytes"),
            ]),
        }),
        ("OP_EEPROMSIZE", OptionInfo {
            byte_index: 7,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "8x512 Bytes"),
                (1, "7x512 Bytes"),
                (2, "6x512 Bytes"),
                (3, "5x512 Bytes"),
                (4, "4x512 Bytes"),
                (5, "3x512 Bytes"),
                (6, "2x512 Bytes"),
                (7, "1x512 Bytes"),
                (8, "0 Bytes"),
            ]),
        }),
    ])
}
