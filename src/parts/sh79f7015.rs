// Auto-generated from GPT file for SH79F7015

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f7015000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 131072,
    eeprom_size: 4096,
    default_code_options: &hex!("0000008489000080"),
    code_option_mask: &hex!("588030040000000f"),
    jtag_id: 0x7015,
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
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: false,
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
                (0, "Disable WDT function in the Power-Down mode"),
                (1, "Enable WDT function in the Power-Down mode"),
            ]),
        }),
        ("OP_RST", OptionInfo {
            byte_index: 0,
            bits_start: 5,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "P2.0 used as RST pin"),
                (1, "P2.0 used as I/O pin"),
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
        ("OP_SCMEN", OptionInfo {
            byte_index: 0,
            bits_start: 1,
            bits_end: 1,
            editable: false,
            states: IndexMap::from([
                (0, "Enable SCM"),
                (1, "Disable SCM"),
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
        ("OP_WDTIDL", OptionInfo {
            byte_index: 2,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the IDLE mode"),
                (1, "Enable WDT function in the IDLE mode"),
            ]),
        }),
        ("OP_VINEN", OptionInfo {
            byte_index: 2,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "Pin 1.3 used as VIN pin, Enable detects VIN pin voltage"),
                (1, "Pin 1.3 used as I/O pin, Disable detect VIN pin voltage"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: false,
            states: IndexMap::from([
                (0, "Define by CLKCON register"),
                (15, "Oscillator1 external clock from P1.1, oscillator2 is disabled."),
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
                (0, "Enter ISP mode only when P0.6 and P0.7 are connected to GND, simultaneously."),
                (1, "Enter ISP mode directly regardless the condition of P0.6 and P0.7"),
            ]),
        }),
        ("OP_CRY16M", OptionInfo {
            byte_index: 3,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator is 2M-12MHz"),
                (1, "Oscillator is 16M-20MHz"),
            ]),
        }),
        ("OP_TPS_CLK_SEL", OptionInfo {
            byte_index: 4,
            bits_start: 7,
            bits_end: 7,
            editable: false,
            states: IndexMap::from([
                (0, "TPS clock is 512k"),
                (1, "TPS clock is 1024k"),
            ]),
        }),
        ("OP_CICCTL", OptionInfo {
            byte_index: 4,
            bits_start: 2,
            bits_end: 3,
            editable: false,
            states: IndexMap::from([
                (0, "CIC 16384"),
                (1, "CIC 8192"),
                (2, "CIC 4096"),
                (3, "CIC 2048"),
            ]),
        }),
        ("OP_IB2_SEL", OptionInfo {
            byte_index: 4,
            bits_start: 0,
            bits_end: 1,
            editable: false,
            states: IndexMap::from([
                (0, "Normal"),
                (1, "Back ADC current X 2"),
                (2, "Front BIAS current X 2"),
                (3, "ADC current X 2"),
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
                (8, "0Bytes"),
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
                (8, "0Bytes"),
            ]),
        }),
    ])
}
