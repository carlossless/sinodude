// Auto-generated from GPT file for SH79F7019A

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f7019a00"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 131072,
    eeprom_size: 4096,
    default_code_options: &hex!("0000000000000088"),
    code_option_mask: &hex!("c38030000000000f"),
    jtag_id: 0x7011,
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
        (
            "OP_WDT",
            OptionInfo {
                byte_index: 0,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Enable WDT function"), (1, "Disable WDT function")]),
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
                    (0, "Disable WDT function in the Power-Down mode"),
                    (1, "Enable WDT function in the Power-Down mode"),
                ]),
            },
        ),
        (
            "OP_RST",
            OptionInfo {
                byte_index: 0,
                bits_start: 5,
                bits_end: 5,
                editable: false,
                states: IndexMap::from([
                    (0, "Pin 1.0 used as RST pin"),
                    (1, "Pin 1.0 used as I/O pin"),
                ]),
            },
        ),
        (
            "OP_SCMEN",
            OptionInfo {
                byte_index: 0,
                bits_start: 1,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([(0, "Enable SCM"), (1, "Disable SCM")]),
            },
        ),
        (
            "OP_CLKDIV",
            OptionInfo {
                byte_index: 0,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (0, "System clock 2-bit frequency divider"),
                    (1, "System clock 3-bit frequency divider"),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 1,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_WDTSIDL",
            OptionInfo {
                byte_index: 2,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in the IDLE mode"),
                    (1, "Enable WDT function in the IDLE mode"),
                ]),
            },
        ),
        (
            "OP_VINEN",
            OptionInfo {
                byte_index: 2,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([
                    (0, "Pin 0.3 used as VIN pin, Enable detects VIN pin voltage"),
                    (1, "Pin 0.3 used as I/O pin, Disable detect VIN pin voltage"),
                ]),
            },
        ),
        (
            "OP_OVL",
            OptionInfo {
                byte_index: 2,
                bits_start: 0,
                bits_end: 0,
                editable: false,
                states: IndexMap::from([
                    (0, "OVL generates WDT Reset"),
                    (1, "OVL generates OVL interrupt"),
                ]),
            },
        ),
        (
            "OP_ISPSIZE",
            OptionInfo {
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
            },
        ),
        (
            "OP_EEPROMSIZE",
            OptionInfo {
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
            },
        ),
    ])
}
