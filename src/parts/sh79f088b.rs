// Auto-generated from GPT file for SH79F088B

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f0880000"),
    chip_type: 0x02,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 8192,
    eeprom_size: 256,
    default_code_options: &hex!("90000080"),
    code_option_mask: &hex!("e0000000"),
    jtag_id: 0xf087,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 3,
    bank_type: 0,
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
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(1, "Disable WDT"), (0, "Enable WDT")]),
            },
        ),
        (
            "OP_WDTPD",
            OptionInfo {
                byte_index: 0,
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([
                    (0, " Disable WDT function in the Power-Down mode"),
                    (1, "Enable WDT function in the Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
    ])
}
