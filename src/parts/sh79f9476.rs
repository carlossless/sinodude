// Auto-generated from GPT file for SH79F9476

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f9476000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 4096,
    default_code_options: &hex!("0000008000000080"),
    code_option_mask: &hex!("24f80f1f0000000f"),
    jtag_id: 0x9476,
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
        ("OP_SCMEN", OptionInfo {
            byte_index: 0,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Enable SCM"),
                (1, "Disable SCM"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "P1.5 used as RST pin"),
                (1, "P1.5 used as I/O pin"),
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
        ("OP_COM/IO", OptionInfo {
            byte_index: 1,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "P1.7,P2.0-P2.6 sink ability normal"),
                (1, "P1.7,P2.0-P2.6 sink ability large"),
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
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator1 is internal 24M RC, oscillator2 is disabled"),
                (3, "Oscillator1 is internal 128k RC, oscillator2 is internal 24M RC"),
                (10, "Oscillator1 is 32.768k crystal oscillator, oscillator2 is internal 24M RC"),
                (15, "Oscillator1 external clock, oscillator2 is disabled"),
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
                (0, "Enter ISP mode directly regardless the condition of P2.2 and P2.3"),
                (1, "Enter ISP mode only when P2.2 and P2.3 are connected to GND, simultaneously"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (5, "Disable WDT function"),
                (0, "Enable WDT function"),
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
        ("OP_32KCAP", OptionInfo {
            byte_index: 4,
            bits_start: 2,
            bits_end: 2,
            editable: false,
            states: IndexMap::from([
                (0, "Disable the inbuild 7pF capacitance of 32.768K Crystal"),
                (1, "Enable the inbuild 7pF capacitance of 32.768K Crystal"),
            ]),
        }),
        ("OP_32KDRIVE", OptionInfo {
            byte_index: 4,
            bits_start: 1,
            bits_end: 1,
            editable: false,
            states: IndexMap::from([
                (0, "Drive ability of 32.768K Crystal is normal"),
                (1, "Enhance the drive ability of 32.768K Crystal"),
            ]),
        }),
        ("OP_ISPSIZE", OptionInfo {
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
        }),
        ("OP_EEPROMSIZE", OptionInfo {
            byte_index: 7,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "8 x 512Bytes"),
                (1, "7 x 512Bytes"),
                (2, "6 x 512Bytes"),
                (3, "5 x 512Bytes"),
                (4, "4 x 512Bytes"),
                (5, "3 x 512Bytes"),
                (6, "2 x 512Bytes"),
                (7, "1 x 512Bytes"),
                (8, "0 Bytes"),
            ]),
        }),
    ])
}
