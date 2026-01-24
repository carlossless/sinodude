// Auto-generated from GPT file for SH79F169B

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f169b000"),
    chip_type: 0x02,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 2048,
    default_code_options: &hex!("00000002"),
    code_option_mask: &hex!("000000ff"),
    jtag_id: 0x169b,
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
                byte_index: 3,
                bits_start: 7,
                bits_end: 7,
                editable: true,
                states: IndexMap::from([(0, "Enable WDT function"), (1, "Disable WDT function")]),
            },
        ),
        (
            "OP_WDTPD",
            OptionInfo {
                byte_index: 3,
                bits_start: 6,
                bits_end: 6,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in the PD mode"),
                    (1, "Enable WDT function in the PD mode"),
                ]),
            },
        ),
        (
            "OP_WDTSIDL",
            OptionInfo {
                byte_index: 3,
                bits_start: 5,
                bits_end: 5,
                editable: true,
                states: IndexMap::from([
                    (0, "Disable WDT function in the IDLE mode"),
                    (1, "Enable WDT function in the  IDLE mode"),
                ]),
            },
        ),
        (
            "OP_LVREN",
            OptionInfo {
                byte_index: 3,
                bits_start: 4,
                bits_end: 4,
                editable: true,
                states: IndexMap::from([(0, "Disable LVR function"), (1, "Enable LVR function")]),
            },
        ),
        (
            "OP_RST",
            OptionInfo {
                byte_index: 3,
                bits_start: 3,
                bits_end: 3,
                editable: true,
                states: IndexMap::from([(0, "P2.0 used as RST pin"), (1, "P2.0 used as I/O pin")]),
            },
        ),
        (
            "OP_VINEN",
            OptionInfo {
                byte_index: 3,
                bits_start: 2,
                bits_end: 2,
                editable: true,
                states: IndexMap::from([
                    (
                        0,
                        "Pin 2.3 used as I/O  pin. Disable detect VIN pin voltage",
                    ),
                    (1, "Pin 2.3 used as VIN pin, Enable detect VIN pin voltage"),
                ]),
            },
        ),
        (
            "OP_ISP",
            OptionInfo {
                byte_index: 3,
                bits_start: 1,
                bits_end: 1,
                editable: true,
                states: IndexMap::from([(0, "Enable ISP function."), (1, "Disable ISP function")]),
            },
        ),
        (
            "OP_ ISPPIN",
            OptionInfo {
                byte_index: 3,
                bits_start: 0,
                bits_end: 0,
                editable: true,
                states: IndexMap::from([
                    (
                        0,
                        "Enter ISP mode only when P2.1 and P2.2 are connected to GND",
                    ),
                    (
                        1,
                        "Enter ISP mode directly regardless the condition of P2.1 and P2.2",
                    ),
                ]),
            },
        ),
    ])
}
