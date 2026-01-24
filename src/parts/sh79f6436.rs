// Auto-generated from GPT file for SH79F6436

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f6436000"),
    chip_type: 0x02,
    custom_block: 0x02,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 2048,
    default_code_options: &hex!("00000080"),
    code_option_mask: &hex!("c1803000"),
    jtag_id: 0xf322,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 1,
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
        ("OP_MNM", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "Normal Mode"),
                (1, "Missing Neutral Mode"),
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
        ("OP_RTC", OptionInfo {
            byte_index: 1,
            bits_start: 5,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "Method A"),
                (1, "Method B"),
            ]),
        }),
        ("OP_WDTSIDL", OptionInfo {
            byte_index: 2,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "Disable WDT function in the Super-IDLE mode"),
                (1, "Enable WDT function in the Super-IDLE mode"),
            ]),
        }),
        ("OP_VINEN", OptionInfo {
            byte_index: 2,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "P2.3 used as VIN pin, Enable detect VIN pin voltage"),
                (1, "P2.3 used as I/O pin, Disable detect VIN pin voltage"),
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
                (0, "Enter ISP mode only when P2.1 and P2.2 are connected to GND, simultaneously"),
                (1, "Enter ISP mode directly regardless the condition of P2.1 and P2.2"),
            ]),
        }),
        ("OP_OVL", OptionInfo {
            byte_index: 3,
            bits_start: 5,
            bits_end: 5,
            editable: false,
            states: IndexMap::from([
                (0, "OVL generates WDT Reset"),
                (1, "OVL generates OVL interrupt"),
            ]),
        }),
    ])
}
