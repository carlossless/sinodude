// Auto-generated from GPT file for SH79F6442

use super::{AddressField, OptionInfo, Options, Part, Voltage};
use hex_literal::hex;
use indexmap::IndexMap;

pub const PART: Part = Part {
    part_number: hex!("79f6442000"),
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 65536,
    eeprom_size: 4096,
    default_code_options: &hex!("000000f860000080"),
    code_option_mask: &hex!("bfef0f3fe0f2000f"),
    jtag_id: 0x4426,
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
                (0, "4.1V LVR Level 1"),
                (1, "3.7V LVR Level 2"),
                (2, "2.8V LVR Level 3"),
                (3, "2.1V LVR Level 4"),
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
        ("OP_P1_P0DRV", OptionInfo {
            byte_index: 1,
            bits_start: 0,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "P1.0~P1.5, P0.6~P0.7 Sink/Driving current is maximum level"),
                (1, "P1.0~P1.5, P0.6~P0.7 Sink/Driving current is larger level"),
                (2, "P1.0~P1.5, P0.6~P0.7 Sink/Driving current is smaller level"),
                (3, "P1.0~P1.5, P0.6~P0.7 Sink/Driving current is minimum level"),
            ]),
        }),
        ("OP_OSC", OptionInfo {
            byte_index: 2,
            bits_start: 0,
            bits_end: 3,
            editable: true,
            states: IndexMap::from([
                (0, "Oscillator 1 is Internal 24MHz RC,oscillator 2 is disable"),
                (3, "Oscillator 1 is Internal 128KHz RC,oscillator 2 is Internal 24MHz RC"),
                (6, "Oscillator 1 is Internal 128KHz RC,oscillator 2 is 2M-16MHz crystal/ceramic oscillator"),
                (10, "Oscillator 1 is 32.768kHz crystal oscillator,oscillator 2 is Internal 24MHz RC"),
                (14, "Oscillator 1 is 2M-16MHz crystal/ceramic oscillator,oscillator 2 is disable"),
                (15, "Oscillator 1 is External input clock oscillator,oscillator 2 is disable"),
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
                (1, "Enter ISP mode only when P0.4 and P0.5 are connected to GND,simultaneously"),
            ]),
        }),
        ("OP_OSCRFB", OptionInfo {
            byte_index: 3,
            bits_start: 4,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "150K"),
                (1, "200K"),
                (2, "300K"),
                (3, "500K"),
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
        ("OP_32KCAP", OptionInfo {
            byte_index: 3,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "32k crystal Internal 7pF cap is disable"),
                (1, "32k crystal Internal 7pF cap is enable"),
            ]),
        }),
        ("OP_32KDRIVE", OptionInfo {
            byte_index: 3,
            bits_start: 0,
            bits_end: 0,
            editable: true,
            states: IndexMap::from([
                (0, "32k crystal drive ability normal mode"),
                (1, "32k crystal drive ability large mode(for load cap > 15pF case)"),
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
        ("OP_P2DRV", OptionInfo {
            byte_index: 5,
            bits_start: 6,
            bits_end: 7,
            editable: true,
            states: IndexMap::from([
                (0, "P2.1~P2.7, P1.6 Sink/Driving current is maximum level"),
                (1, "P2.1~P2.7, P1.6 Sink/Driving current is larger level"),
                (2, "P2.1~P2.7, P1.6 Sink/Driving current is smaller level"),
                (3, "P2.1~P2.7, P1.6 Sink/Driving current is minimum level"),
            ]),
        }),
        ("OP_P47-P45P50", OptionInfo {
            byte_index: 5,
            bits_start: 5,
            bits_end: 5,
            editable: true,
            states: IndexMap::from([
                (0, "P4[7:5],P5.0 Sink/Driving ability select normal level"),
                (1, "P4[7:5],P5.0 Sink/Driving ability select larger level"),
            ]),
        }),
        ("OP_P44-P41", OptionInfo {
            byte_index: 5,
            bits_start: 4,
            bits_end: 4,
            editable: true,
            states: IndexMap::from([
                (0, "P4[4:1] Sink/Driving ability select normal level"),
                (1, "P4[4:1] Sink/Driving ability select larger level"),
            ]),
        }),
        ("OP_MODSW", OptionInfo {
            byte_index: 5,
            bits_start: 1,
            bits_end: 1,
            editable: true,
            states: IndexMap::from([
                (0, "LCD/LED Counter RUN,when MODSW set"),
                (1, "LCD/LED Counter STOP and data is preserved,when MODSW set"),
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
                (8, "0 x 1024Bytes"),
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
                (8, "0 x 512Bytes"),
            ]),
        }),
    ])
}
