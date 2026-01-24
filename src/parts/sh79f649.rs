// Auto-generated from GPT file for SH79F649

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("36f8060000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 2048,
    default_code_options: &hex!("00000000"),
    code_option_mask: &hex!("c00000f8"),
    jtag_id: 0x3606,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 2,
    bank_type: 0,
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
        ("OP_CCEN", OptionInfo {
            byte_index: 3,
            bits_start: 3,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Disable CoulombCounter"),
                (1, "Enable CoulombCounter"),
            ]),
        }),
        ("OP_WDTPD", OptionInfo {
            byte_index: 0,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT in PD mode"),
                (1, "Enable WDT in PD mode"),
            ]),
        }),
        ("OP_WDT", OptionInfo {
            byte_index: 0,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT Function"),
                (1, "Enable WDT Function"),
            ]),
        }),
        ("OP_RCPD", OptionInfo {
            byte_index: 3,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "Enable RC in PD mode"),
                (1, "Disable RC in PD mode"),
            ]),
        }),
        ("OP_OVL", OptionInfo {
            byte_index: 3,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "OVL Generates WDT Reset"),
                (1, "OVL Generates OVL Interrupt"),
            ]),
        }),
        ("OP_ISPPIN", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: true,
            states: IndexMap::from([
                (0, "Enter ISP Mode Only when P0.0 and P1.3 are connected to GND simultaneously"),
                (1, "Enter ISP Mode directly regardless the condition of P0.0 and P1.3"),
            ]),
        }),
        ("OP_ISP", OptionInfo {
            byte_index: 3,
            bits_start: 7,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "Enable ISP Function"),
                (1, "Disable ISP Function"),
            ]),
        }),
    ])
}
