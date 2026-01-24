// Auto-generated from GPT file for HP60908

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("39f3008300"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 32768,
    eeprom_size: 4096,
    default_code_options: &hex!("000000c060000086"),
    code_option_mask: &hex!("ffec0f0ce000000f"),
    jtag_id: 0x6441,
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
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the Power-Down mode"),
                (1, "Enable WDT function in the Power-Down mode"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "P1.7 used as RST pin"),
                (1, "P1.7 used as I/O pin"),
            ]),
        }),
        ("OP_WMT", OptionInfo {
            byte_index: 0,
            bits_start: 4,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "longest warm up time"),
                (1, "longer warm up time"),
                (2, "shorter warm up time"),
                (3, "shortest warm up time"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (5, "Disable WDT function"),
                (0, "Enable WDT function"),
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
                (2, "2.8V LVR Level 3"),
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
        ("OP_SCMEN", OptionInfo {
            byte_index: 1,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Enable SCM"),
                (1, "Disable SCM"),
            ]),
        }),
        ("OP_P0DRV", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 1,
            editable: false,
            states: IndexMap::from([
                (0, "Port0 driving current is not changed"),
                (1, "Port0 driving current is reduced to 5/10 of default value"),
                (2, "Port0 driving current is reduced to 3/10 of default value"),
                (3, "Port0 driving current is reduced to 1/10 of default value"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Build in 24M RC as OSC1CLK, OSC2CLK is closed"),
                (3, "Build in 128KHz RC as OSC1CLK, Build in 24MHz RC as OSC2CLK"),
                (6, "Build in 128KHz RC as OSC1CLK, 2M-16M Crystal/Ceramic as OSC2CLK"),
                (10, "32.768KHz Crystal as OSC1CLK, Build in 24MHz RC as OSC2CLK"),
                (14, "2M-16MHz Crystal/Ceramic as OSC1CLK, OSC2CLK is closed"),
                (15, "External clock as OSC1CLK, OSC2CLK is closed"),
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
                (0, "Enter ISP mode directly regardless the condition of P0.4 and P0.5"),
                (1, "Enter ISP mode only when P0.4 and P0.5 are connected to GND, simultaneously"),
            ]),
        }),
        ("OP_OSCRFB", OptionInfo {
            byte_index: 3,
            bits_start: 4,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "150k"),
                (1, "200k"),
                (2, "300k"),
                (3, "500k"),
            ]),
        }),
        ("OP_OSCDRIVE", OptionInfo {
            byte_index: 3,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "2~12M crystal /ceramic"),
                (1, "16M crystal /ceramic"),
            ]),
        }),
        ("OP_AHRV", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "C32K Anti-Humidity register reset value = 0"),
                (1, "C32K Anti-Humidity register reset value = 1"),
            ]),
        }),
        ("OP_SCMSEL", OptionInfo {
            byte_index: 4,
            bits_start: 5,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "2MHz"),
                (1, "4MHz"),
                (2, "6MHz"),
                (3, "8MHz"),
                (4, "12MHz"),
                (5, "16MHz"),
            ]),
        }),
        ("OP_P35 P32-P30", OptionInfo {
            byte_index: 5,
            bits_start: 4,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "Port3[5] Port3[2:0] sink ability select larger level"),
                (1, "Port3[5] Port3[2:0] sink ability select smaller level"),
                (2, "Port3[5] Port3[2:0] sink ability select minimum level"),
                (3, "Port3[5] Port3[2:0] sink ability select max level"),
            ]),
        }),
        ("OP_P27-P24", OptionInfo {
            byte_index: 5,
            bits_start: 2,
            bits_end: 3,
            editable: false,
            states: IndexMap::from([
                (0, "Port2[7:4] sink ability select larger level"),
                (1, "Port2[7:4] sink ability select smaller level"),
                (2, "Port2[7:4] sink ability select minimum level"),
                (3, "Port2[7:4] sink ability select max level"),
            ]),
        }),
        ("OP_P23-P20", OptionInfo {
            byte_index: 5,
            bits_start: 0,
            bits_end: 1,
            editable: false,
            states: IndexMap::from([
                (0, "Port2[3:0] sink ability select larger level"),
                (1, "Port2[3:0] sink ability select smaller level"),
                (2, "Port2[3:0] sink ability select minimum level"),
                (3, "Port2[3:0] sink ability select max level"),
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
                (8, "0 bytes"),
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
                (8, "0 bytes"),
            ]),
        }),
    ])
}
