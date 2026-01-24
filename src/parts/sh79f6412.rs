// Auto-generated from GPT file for SH79F6412

use super::{AddressField, OptionInfo, Options, Part};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f6412000"),
    chip_type: 0x02,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 2048,
    default_code_options: &hex!("0102008c"),
    code_option_mask: &hex!("dfee0f18"),
    jtag_id: 0x3212,
    sector_size: 1024,
    option_byte_count: 4,
    security_level: 3,
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
            editable: true,
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
                (0, "Disable WDT function in Power-Down mode"),
                (1, "Enable WDT function in Power-Down mode"),
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
        ("OP_SEG/IO", OptionInfo {
            byte_index: 0,
            bits_start: 1,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "P0.5~P0.6, P1.2, P1.4~P1.7, P2.0~P2.6, P3.0~P3.1, VDD-3V,segment output 40 mA current"),
                (1, "P0.5~P0.6, P1.2, P1.4~P1.7, P2.0~P2.6, P3.0~P3.1, VDD-3V,segment output 30 mA"),
                (2, "P0.5~P0.6, P1.2, P1.4~P1.7, P2.0~P2.6, P3.0~P3.1, VDD-3V,segment output 20 mA"),
                (3, "P0.5~P0.6, P1.2, P1.4~P1.7, P2.0~P2.6, P3.0~P3.1, VDD-3V,segment output 10 mA"),
            ]),
        }),
        ("OP_OSC_DRIVE1", OptionInfo {
            byte_index: 0,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "0 : OP_OSC_DRIVE2=10 : 2M ceramic ;  OP_OSC_DRIVE2 = 11 : 4M ceramic"),
                (1, "1: OP_OSC_DRIVE2 = 10 : 8M ceramic ;  OP_OSC_DRIVE2 = 11 : 12M ceramic ; OP_OSC_DRIVE2 = 00: 4M crystal ; OP_OSC_DRIVE2 = 01: 8M-12M crystal"),
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
                (0, "3.7V LVR level 1"),
                (1, "3.1V LVR level 2"),
                (2, "2.8V LVR level 3"),
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
        ("OP_MODSW", OptionInfo {
            byte_index: 1,
            bits_start: 2,
            bits_end: 2,
            editable: true,
            states: IndexMap::from([
                (0, "LED counter stop and data is preserved,if MODSW bit is 1"),
                (1, "LED counter run, if MODSW bit is 1"),
            ]),
        }),
        ("OP_COM/IO", OptionInfo {
            byte_index: 1,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "P0.7, P1.0~P1.2, P2.7, P3.0~P3.1,sink/drive ability large mode"),
                (1, "P0.7, P1.0~P1.2, P2.7, P3.0~P3.1, sink/drive ability normal mode"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator1 is internal 16.6M RC,Oscillator2 is disabled"),
                (3, "Oscillator1 is internal 128K rc, Oscillator2 is internal 16.6M RC"),
                (6, "Oscillator1 is internal 128K RC, Oscillator is 2M-12MHZ crystal/ceramic Oscillator"),
                (10, "Oscillator1 is 32.768K crystal,Oscillator2 is16.6M RC"),
                (14, "Oscillator1 is 2M-12M crystal/ceramic oscillator, Oscillator is disable"),
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
        ("OP_P0", OptionInfo {
            byte_index: 3,
            bits_start: 6,
            bits_end: 6,
            editable: false,
            states: IndexMap::from([
                (0, "P0.1-P0.4,sink ability normal mode"),
                (1, "P0.1-P0.4, sink ability large mode"),
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
        ("OP_OSC_DRIVE2", OptionInfo {
            byte_index: 3,
            bits_start: 3,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "00 : OP_OSC_DRIVE1 = 1 : 4M crystal"),
                (1, "01: OP_OSC_DRIVE1 =1 : 8M-12M crystal"),
                (2, "10: OP_OSC_DRIVE1 =0 : 2M ceramic ; OP_OSC_DRIVE1 =1 : 8M ceramic"),
                (3, "11: OP_OSC_DRIVE1 =0 : 4M ceramic ; OP_OSC_DRIVE1 =1 : 12M ceramic"),
            ]),
        }),
        ("OP_FEED_RES", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 2,
            editable: false,
            states: IndexMap::from([
                (0, "2M"),
                (1, "1M"),
                (2, "500K"),
                (3, "300K"),
            ]),
        }),
    ])
}
