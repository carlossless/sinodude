// Auto-generated from GPT file for SH86F7066

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("86f7066000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 4096,
    default_code_options: &hex!("004300c960000080"),
    code_option_mask: &hex!("bfff0f0fe00f000f"),
    jtag_id: 0xf708,
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
                (2, "2.8V LVR Level 3"),
                (3, "2.1V LVR Level 4"),
            ]),
        }),
        ("OP_SCM", OptionInfo {
            byte_index: 1,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "SCM is invalid in warm up period"),
                (1, "SCM is valid in warm up period"),
            ]),
        }),
        ("OP_SCMEN", OptionInfo {
            byte_index: 1,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Enable SCM"),
                (1, "Disable SCM"),
            ]),
        }),
        ("OP_32KLCAP", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Turn off the built-in load capacitor of 32.768kHz Crystal Oscillator."),
                (1, "32.768kHz Crystal Oscillator built-in load capacitor is 8pF."),
                (2, "32.768kHz Crystal Oscillator built-in load capacitor is 10pF."),
                (3, "32.768kHz Crystal Oscillator built-in load capacitor is 12pF."),
                (4, "32.768kHz Crystal Oscillator built-in load capacitor is 15pF."),
                (5, "32.768kHz Crystal Oscillator built-in load capacitor is 20pF."),
                (6, "32.768kHz Crystal Oscillator built-in load capacitor is 22pF."),
                (7, "32.768kHz Crystal Oscillator built-in load capacitor is 25pF."),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Build in 20MHz RC as OSC1CLK, OSC2CLK is closed."),
                (3, "Build in 128KHz RC as OSC1CLK, Build in 20MHz RC as OSC2CLK."),
                (6, "Build in 128KHz RC as OSC1CLK, 2M-16M Crystal/Ceramic as OSC2CLK."),
                (10, "32.768KHz Crystal as OSC1CLK, Build in 20MHz RC as OSC2CLK."),
                (14, "2M-16MHz Crystal/Ceramic as OSC1CLK, OSC2CLK is closed."),
                (15, "External clock as OSC1CLK, OSC2CLK is closed."),
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
                (0, "Enter ISP mode directly regardless the condition of P7.6 and P7.7"),
                (1, "Enter ISP mode only when P7.6 and P7.7 are connected to GND, simultaneously"),
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
            bits_start: 2,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "2~12M ceramic regardless of cap loads"),
                (1, "16M ceramic regardless of cap loads"),
                (2, "4M crystal and 8M~12M crystal with cap load(Cg=Cd)<20pF"),
                (3, "16M crystal and 8M~12M crystal with cap load(Cg=Cd)>=20pF"),
            ]),
        }),
        ("OP_32KDRIVE", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "32k Crystal drive ability normal."),
                (1, "32k Crystal drive ability enhance. (Driving CL>=15pF)"),
            ]),
        }),
        ("OP_LCDSEL", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Select resistor LCD driver."),
                (1, "Select capacitor LCD driver."),
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
        ("OP_P46SINK", OptionInfo {
            byte_index: 5,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Port4[6] sink ability normal."),
                (1, "Port4[6] sink ability larger."),
            ]),
        }),
        ("OP_P45SINK", OptionInfo {
            byte_index: 5,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Port4[5] sink ability normal."),
                (1, "Port4[5] sink ability larger."),
            ]),
        }),
        ("OP_P05SINK", OptionInfo {
            byte_index: 5,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "Port0[5] sink ability normal."),
                (1, "Port0[5] sink ability larger."),
            ]),
        }),
        ("OP_P04SINK", OptionInfo {
            byte_index: 5,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Port0[4] sink ability normal."),
                (1, "Port0[4] sink ability larger."),
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
