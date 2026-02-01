// Auto-generated from GPT file for SH79F9273

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f9273000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 32768,
    eeprom_size: 4096,
    default_code_options: &hex!("200000c000000080"),
    code_option_mask: &hex!("00ee0f1f0100000f"),
    jtag_id: 0x328a,
    sector_size: 512,
    option_byte_count: 8,
    security_level: 4,
    bank_type: 1,
    customer_id: AddressField { address: 0x1000 },
    operation_number: AddressField { address: 0x1004 },
    customer_option: AddressField { address: 0x1006 },
    security: AddressField { address: 0x100a },
    serial_number: AddressField { address: 0x103c },
    compatible_voltages: &[Voltage::V5_0, Voltage::V3_3],
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
                (0, "P5.2 used as RST pin"),
                (1, "P5.2 used as I/O pin"),
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
        ("OP_P37-P34", OptionInfo {
            byte_index: 1,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Port3[7:4] sink ability normal mode"),
                (1, "Port3[7:4] sink ability large mode"),
            ]),
        }),
        ("OP_P33-P30", OptionInfo {
            byte_index: 1,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "Port3[3:0] sink ability normal mode"),
                (1, "Port3[3:0] sink ability large mode"),
            ]),
        }),
        ("OP_MODSW", OptionInfo {
            byte_index: 1,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "LED Counter RUN,when MODSW set"),
                (1, "LED Counter STOP and data is preserved,when MODSW set"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator1 is internal High RC, oscillator2 is disabled"),
                (3, "Oscillator1 is internal 128k RC, oscillator2 is internal High RC"),
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
                (0, "Enter ISP mode directly regardless the condition of P4.1 and P4.2"),
                (1, "Enter ISP mode only when P4.1 and P4.2 are connected to GND, simultaneously"),
            ]),
        }),
        ("OP_RC_SEL", OptionInfo {
            byte_index: 4,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "24M RC"),
                (1, "16M RC"),
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
