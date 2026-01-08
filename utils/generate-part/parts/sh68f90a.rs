// Auto-generated from GPT file for SH68F90A

use super::Part;
use hex_literal::hex;

pub const PART: Part = Part {
    part_number: hex!("68f90a0000"),
    model: hex!("000000000000"), // TODO: model bytes unknown from GPT
    chip_type: 0x07,
    custom_block: 0x03,
    product_block: 0x01,
    flash_size: 65536,
    default_code_options: &hex!("842060c00f000088"),
    code_option_mask: &hex!("20eb0f1f0f000000"),
    jtag_id: 0xf690,
    sector_size: 512,
};

// Additional chip information:
// External RAM: 4608 bytes
// EEPROM: 0 bytes
//
// Memory addresses (region, address):
// Customer ID: region=2, 0x1000
// Operation Number: region=2, 0x1004
// Customer Option: region=2, 0x1006
// Security: region=2, 0x100A
// Serial Number: region=2, 0x103C

// Address constants
pub const CUSTOMER_ID_ADDR: u32 = 0x1000;
pub const CUSTOMER_ID_REGION: u8 = 2;
pub const OPERATION_NUMBER_ADDR: u32 = 0x1004;
pub const OPERATION_NUMBER_REGION: u8 = 2;
pub const CUSTOMER_OPTION_ADDR: u32 = 0x1006;
pub const CUSTOMER_OPTION_REGION: u8 = 2;
pub const SECURITY_ADDR: u32 = 0x100A;
pub const SECURITY_REGION: u8 = 2;
pub const SERIAL_NUMBER_ADDR: u32 = 0x103C;
pub const SERIAL_NUMBER_REGION: u8 = 2;

/// Format user-editable code options
pub fn format_user_options(options: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let op_rst = (options[0] >> 5) & 0x01;
    let _ = write!(out, "OP_RST:: ");
    match op_rst {
        0 => { let _ = writeln!(out, "P0.2 used as RST pin"); }
        1 => { let _ = writeln!(out, "P0.2 used as IO pin"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_lvren = (options[1] >> 7) & 0x01;
    let _ = write!(out, "OP_LVREN:: ");
    match op_lvren {
        0 => { let _ = writeln!(out, "Disable LVR function"); }
        1 => { let _ = writeln!(out, "Enable LVR function"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_lvrlevel = (options[1] >> 5) & 0x03;
    let _ = write!(out, "OP_LVRLEVEL:: ");
    match op_lvrlevel {
        0 => { let _ = writeln!(out, "4.1V LVR level1"); }
        1 => { let _ = writeln!(out, "3.7V LVR level2"); }
        2 => { let _ = writeln!(out, "2.8V LVR level3"); }
        3 => { let _ = writeln!(out, "2.1V LVR level4"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_osc2sel = (options[1] >> 3) & 0x01;
    let _ = write!(out, "OP_OSC2SEL:: ");
    match op_osc2sel {
        0 => { let _ = writeln!(out, "OSC2 select 12M RC"); }
        1 => { let _ = writeln!(out, "OSC2 select 24M RC"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_iov1 = (options[1] >> 1) & 0x01;
    let _ = write!(out, "OP_IOV1:: ");
    match op_iov1 {
        0 => { let _ = writeln!(out, "P7.1/P7.2/P7.3/P7.4 input/output level is VUSB(5V)"); }
        1 => { let _ = writeln!(out, "P7.1/P7.2/P7.3/P7.4 input/output level is VDDR(3.3V)"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_iov0 = (options[1] >> 0) & 0x01;
    let _ = write!(out, "OP_IOV0:: ");
    match op_iov0 {
        0 => { let _ = writeln!(out, "P5.5/P5.6 input/output level is VUSB(5V)"); }
        1 => { let _ = writeln!(out, "P5.5/P5.6 input/output level is VDDR(3.3V)"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_osc = (options[2] >> 0) & 0x0f;
    let _ = write!(out, "OP_OSC:: ");
    match op_osc {
        0 => { let _ = writeln!(out, "Oscillator1 is internal 24MHz RC, oscillator2 is disabled"); }
        3 => { let _ = writeln!(out, "Oscillator1 is internal 128KHz RC, oscillator2 is internal 24MHz RC"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_wdt = (options[3] >> 1) & 0x0f;
    let _ = write!(out, "OP_WDT:: ");
    match op_wdt {
        5 => { let _ = writeln!(out, "Disable WDT function"); }
        0 => { let _ = writeln!(out, "Enable WDT function"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_wdtpd = (options[3] >> 0) & 0x01;
    let _ = write!(out, "OP_WDTPD:: ");
    match op_wdtpd {
        0 => { let _ = writeln!(out, "Disable WDT function in Power-Down mode"); }
        1 => { let _ = writeln!(out, "Enable WDT function in Power-Down mode"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_sink1 = (options[4] >> 2) & 0x03;
    let _ = write!(out, "OP_SINK1:: ");
    match op_sink1 {
        0 => { let _ = writeln!(out, "Port6[5:0] sink ability larger mode(350mA)"); }
        1 => { let _ = writeln!(out, "Port6[5:0] sink ability smaller mode(250mA)"); }
        2 => { let _ = writeln!(out, "Port6[5:0] sink ability smallest mode(normal IO)"); }
        3 => { let _ = writeln!(out, "Port6[5:0] sink ability largest mode(380mA)"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_sink0 = (options[4] >> 0) & 0x03;
    let _ = write!(out, "OP_SINK0:: ");
    match op_sink0 {
        0 => { let _ = writeln!(out, "P4.7/Port7[7:5] sink ability larger mode(40mA)"); }
        1 => { let _ = writeln!(out, "P4.7/Port7[7:5] sink ability smaller mode(30mA)"); }
        2 => { let _ = writeln!(out, "P4.7/Port7[7:5] sink ability smallest mode(normal IO)"); }
        3 => { let _ = writeln!(out, "P4.7/Port7[7:5] sink ability largest mode(50mA)"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    out
}

/// Format read-only/system code options (not user-editable)
pub fn format_system_options(options: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let op_oscdrive = (options[0] >> 6) & 0x03;
    let _ = write!(out, "OP_OSCDRIVE:: ");
    match op_oscdrive {
        0 => { let _ = writeln!(out, "2~12MHz ceramic"); }
        1 => { let _ = writeln!(out, "16MHz ceramic"); }
        2 => { let _ = writeln!(out, "4MHz crystal or 8~12MHz crystal with external capacitance(C1=C2)<20pF"); }
        3 => { let _ = writeln!(out, "16MHz crystal or 8~12MHz crystal with external capacitance(C1=C2)>20PF"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_wmt = (options[0] >> 3) & 0x03;
    let _ = write!(out, "OP_WMT:: ");
    match op_wmt {
        0 => { let _ = writeln!(out, "longest warm up time"); }
        1 => { let _ = writeln!(out, "longer warm up time"); }
        2 => { let _ = writeln!(out, "shorter warm up time"); }
        3 => { let _ = writeln!(out, "shortest warm up time"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_scmen = (options[0] >> 2) & 0x01;
    let _ = write!(out, "OP_SCMEN:: ");
    match op_scmen {
        0 => { let _ = writeln!(out, "Enable SCM"); }
        1 => { let _ = writeln!(out, "Disable SCM"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_oscrfb = (options[0] >> 0) & 0x03;
    let _ = write!(out, "OP_OSCRFB:: ");
    match op_oscrfb {
        0 => { let _ = writeln!(out, "150K"); }
        1 => { let _ = writeln!(out, "200K"); }
        2 => { let _ = writeln!(out, "300K"); }
        3 => { let _ = writeln!(out, "500K"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_scm = (options[1] >> 4) & 0x01;
    let _ = write!(out, "OP_SCM:: ");
    match op_scm {
        0 => { let _ = writeln!(out, "SCM is invalid in warm up period"); }
        1 => { let _ = writeln!(out, "SCM is valid in warm up period"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_scmsel = (options[2] >> 5) & 0x07;
    let _ = write!(out, "OP_SCMSEL:: ");
    match op_scmsel {
        0 => { let _ = writeln!(out, "2MHz"); }
        1 => { let _ = writeln!(out, "4MHZ"); }
        2 => { let _ = writeln!(out, "6MHZ"); }
        3 => { let _ = writeln!(out, "8MHZ"); }
        4 => { let _ = writeln!(out, "12MHZ"); }
        5 => { let _ = writeln!(out, "16MHZ"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_isp = (options[3] >> 7) & 0x01;
    let _ = write!(out, "OP_ISP:: ");
    match op_isp {
        0 => { let _ = writeln!(out, "Enable ISP function"); }
        1 => { let _ = writeln!(out, "Disable ISP function"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_isppin = (options[3] >> 6) & 0x01;
    let _ = write!(out, "OP_ISPPIN:: ");
    match op_isppin {
        0 => { let _ = writeln!(out, "Enter ISP mode directly regardless the condition of P6.3 and P6.4"); }
        1 => { let _ = writeln!(out, "Enter ISP mode only when P6.3 and P6.4 are connected to GND, simultaneously"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_boptp = (options[5] >> 4) & 0x07;
    let _ = write!(out, "OP_BOPTP:: ");
    match op_boptp {
        4 => { let _ = writeln!(out, "tr max"); }
        5 => { let _ = writeln!(out, "(1+42%)tr min"); }
        6 => { let _ = writeln!(out, "(1+35%)tr min"); }
        7 => { let _ = writeln!(out, "(1+28%)tr min"); }
        0 => { let _ = writeln!(out, "(1+21%)tr min"); }
        1 => { let _ = writeln!(out, "(1+14%)tr min"); }
        2 => { let _ = writeln!(out, "(1+7%)tr min"); }
        3 => { let _ = writeln!(out, "tr min"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_boptn = (options[5] >> 0) & 0x07;
    let _ = write!(out, "OP_BOPTN:: ");
    match op_boptn {
        4 => { let _ = writeln!(out, "tf max"); }
        5 => { let _ = writeln!(out, "(1+42%)tf min"); }
        6 => { let _ = writeln!(out, "(1+35%)tf min"); }
        7 => { let _ = writeln!(out, "(1+28%)tf min"); }
        0 => { let _ = writeln!(out, "(1+21%)tf min"); }
        1 => { let _ = writeln!(out, "(1+14%)tf min"); }
        2 => { let _ = writeln!(out, "(1+7%)tf min"); }
        3 => { let _ = writeln!(out, "tf min"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_ispsize = (options[7] >> 4) & 0x0f;
    let _ = write!(out, "OP_ISPSIZE:: ");
    match op_ispsize {
        0 => { let _ = writeln!(out, "8 x 1024Bytes"); }
        1 => { let _ = writeln!(out, "7 x 1024Bytes"); }
        2 => { let _ = writeln!(out, "6 x 1024Bytes"); }
        3 => { let _ = writeln!(out, "5 x 1024Bytes"); }
        4 => { let _ = writeln!(out, "4 x 1024Bytes"); }
        5 => { let _ = writeln!(out, "3 x 1024Bytes"); }
        6 => { let _ = writeln!(out, "2 x 1024Bytes"); }
        7 => { let _ = writeln!(out, "1 x 1024Bytes"); }
        8 => { let _ = writeln!(out, "0 Bytes"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_eepromsize = (options[7] >> 0) & 0x0f;
    let _ = write!(out, "OP_EEPROMSIZE:: ");
    match op_eepromsize {
        0 => { let _ = writeln!(out, "8 x 512Bytes"); }
        1 => { let _ = writeln!(out, "7 x 512Bytes"); }
        2 => { let _ = writeln!(out, "6 x 512Bytes"); }
        3 => { let _ = writeln!(out, "5 x 512Bytes"); }
        4 => { let _ = writeln!(out, "4 x 512Bytes"); }
        5 => { let _ = writeln!(out, "3 x 512Bytes"); }
        6 => { let _ = writeln!(out, "2 x 512Bytes"); }
        7 => { let _ = writeln!(out, "1 x 512Bytes"); }
        8 => { let _ = writeln!(out, "0 Bytes"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    out
}

/// Format all code options
pub fn format_code_options(options: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let _ = writeln!(out, "=== User Options ===");
    let op_rst = (options[0] >> 5) & 0x01;
    let _ = write!(out, "OP_RST:: ");
    match op_rst {
        0 => { let _ = writeln!(out, "P0.2 used as RST pin"); }
        1 => { let _ = writeln!(out, "P0.2 used as IO pin"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_lvren = (options[1] >> 7) & 0x01;
    let _ = write!(out, "OP_LVREN:: ");
    match op_lvren {
        0 => { let _ = writeln!(out, "Disable LVR function"); }
        1 => { let _ = writeln!(out, "Enable LVR function"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_lvrlevel = (options[1] >> 5) & 0x03;
    let _ = write!(out, "OP_LVRLEVEL:: ");
    match op_lvrlevel {
        0 => { let _ = writeln!(out, "4.1V LVR level1"); }
        1 => { let _ = writeln!(out, "3.7V LVR level2"); }
        2 => { let _ = writeln!(out, "2.8V LVR level3"); }
        3 => { let _ = writeln!(out, "2.1V LVR level4"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_osc2sel = (options[1] >> 3) & 0x01;
    let _ = write!(out, "OP_OSC2SEL:: ");
    match op_osc2sel {
        0 => { let _ = writeln!(out, "OSC2 select 12M RC"); }
        1 => { let _ = writeln!(out, "OSC2 select 24M RC"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_iov1 = (options[1] >> 1) & 0x01;
    let _ = write!(out, "OP_IOV1:: ");
    match op_iov1 {
        0 => { let _ = writeln!(out, "P7.1/P7.2/P7.3/P7.4 input/output level is VUSB(5V)"); }
        1 => { let _ = writeln!(out, "P7.1/P7.2/P7.3/P7.4 input/output level is VDDR(3.3V)"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_iov0 = (options[1] >> 0) & 0x01;
    let _ = write!(out, "OP_IOV0:: ");
    match op_iov0 {
        0 => { let _ = writeln!(out, "P5.5/P5.6 input/output level is VUSB(5V)"); }
        1 => { let _ = writeln!(out, "P5.5/P5.6 input/output level is VDDR(3.3V)"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_osc = (options[2] >> 0) & 0x0f;
    let _ = write!(out, "OP_OSC:: ");
    match op_osc {
        0 => { let _ = writeln!(out, "Oscillator1 is internal 24MHz RC, oscillator2 is disabled"); }
        3 => { let _ = writeln!(out, "Oscillator1 is internal 128KHz RC, oscillator2 is internal 24MHz RC"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_wdt = (options[3] >> 1) & 0x0f;
    let _ = write!(out, "OP_WDT:: ");
    match op_wdt {
        5 => { let _ = writeln!(out, "Disable WDT function"); }
        0 => { let _ = writeln!(out, "Enable WDT function"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_wdtpd = (options[3] >> 0) & 0x01;
    let _ = write!(out, "OP_WDTPD:: ");
    match op_wdtpd {
        0 => { let _ = writeln!(out, "Disable WDT function in Power-Down mode"); }
        1 => { let _ = writeln!(out, "Enable WDT function in Power-Down mode"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_sink1 = (options[4] >> 2) & 0x03;
    let _ = write!(out, "OP_SINK1:: ");
    match op_sink1 {
        0 => { let _ = writeln!(out, "Port6[5:0] sink ability larger mode(350mA)"); }
        1 => { let _ = writeln!(out, "Port6[5:0] sink ability smaller mode(250mA)"); }
        2 => { let _ = writeln!(out, "Port6[5:0] sink ability smallest mode(normal IO)"); }
        3 => { let _ = writeln!(out, "Port6[5:0] sink ability largest mode(380mA)"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_sink0 = (options[4] >> 0) & 0x03;
    let _ = write!(out, "OP_SINK0:: ");
    match op_sink0 {
        0 => { let _ = writeln!(out, "P4.7/Port7[7:5] sink ability larger mode(40mA)"); }
        1 => { let _ = writeln!(out, "P4.7/Port7[7:5] sink ability smaller mode(30mA)"); }
        2 => { let _ = writeln!(out, "P4.7/Port7[7:5] sink ability smallest mode(normal IO)"); }
        3 => { let _ = writeln!(out, "P4.7/Port7[7:5] sink ability largest mode(50mA)"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let _ = writeln!(out, "\n=== System Options (read-only) ===");
    let op_oscdrive = (options[0] >> 6) & 0x03;
    let _ = write!(out, "OP_OSCDRIVE:: ");
    match op_oscdrive {
        0 => { let _ = writeln!(out, "2~12MHz ceramic"); }
        1 => { let _ = writeln!(out, "16MHz ceramic"); }
        2 => { let _ = writeln!(out, "4MHz crystal or 8~12MHz crystal with external capacitance(C1=C2)<20pF"); }
        3 => { let _ = writeln!(out, "16MHz crystal or 8~12MHz crystal with external capacitance(C1=C2)>20PF"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_wmt = (options[0] >> 3) & 0x03;
    let _ = write!(out, "OP_WMT:: ");
    match op_wmt {
        0 => { let _ = writeln!(out, "longest warm up time"); }
        1 => { let _ = writeln!(out, "longer warm up time"); }
        2 => { let _ = writeln!(out, "shorter warm up time"); }
        3 => { let _ = writeln!(out, "shortest warm up time"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_scmen = (options[0] >> 2) & 0x01;
    let _ = write!(out, "OP_SCMEN:: ");
    match op_scmen {
        0 => { let _ = writeln!(out, "Enable SCM"); }
        1 => { let _ = writeln!(out, "Disable SCM"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_oscrfb = (options[0] >> 0) & 0x03;
    let _ = write!(out, "OP_OSCRFB:: ");
    match op_oscrfb {
        0 => { let _ = writeln!(out, "150K"); }
        1 => { let _ = writeln!(out, "200K"); }
        2 => { let _ = writeln!(out, "300K"); }
        3 => { let _ = writeln!(out, "500K"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_scm = (options[1] >> 4) & 0x01;
    let _ = write!(out, "OP_SCM:: ");
    match op_scm {
        0 => { let _ = writeln!(out, "SCM is invalid in warm up period"); }
        1 => { let _ = writeln!(out, "SCM is valid in warm up period"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_scmsel = (options[2] >> 5) & 0x07;
    let _ = write!(out, "OP_SCMSEL:: ");
    match op_scmsel {
        0 => { let _ = writeln!(out, "2MHz"); }
        1 => { let _ = writeln!(out, "4MHZ"); }
        2 => { let _ = writeln!(out, "6MHZ"); }
        3 => { let _ = writeln!(out, "8MHZ"); }
        4 => { let _ = writeln!(out, "12MHZ"); }
        5 => { let _ = writeln!(out, "16MHZ"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_isp = (options[3] >> 7) & 0x01;
    let _ = write!(out, "OP_ISP:: ");
    match op_isp {
        0 => { let _ = writeln!(out, "Enable ISP function"); }
        1 => { let _ = writeln!(out, "Disable ISP function"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_isppin = (options[3] >> 6) & 0x01;
    let _ = write!(out, "OP_ISPPIN:: ");
    match op_isppin {
        0 => { let _ = writeln!(out, "Enter ISP mode directly regardless the condition of P6.3 and P6.4"); }
        1 => { let _ = writeln!(out, "Enter ISP mode only when P6.3 and P6.4 are connected to GND, simultaneously"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_boptp = (options[5] >> 4) & 0x07;
    let _ = write!(out, "OP_BOPTP:: ");
    match op_boptp {
        4 => { let _ = writeln!(out, "tr max"); }
        5 => { let _ = writeln!(out, "(1+42%)tr min"); }
        6 => { let _ = writeln!(out, "(1+35%)tr min"); }
        7 => { let _ = writeln!(out, "(1+28%)tr min"); }
        0 => { let _ = writeln!(out, "(1+21%)tr min"); }
        1 => { let _ = writeln!(out, "(1+14%)tr min"); }
        2 => { let _ = writeln!(out, "(1+7%)tr min"); }
        3 => { let _ = writeln!(out, "tr min"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_boptn = (options[5] >> 0) & 0x07;
    let _ = write!(out, "OP_BOPTN:: ");
    match op_boptn {
        4 => { let _ = writeln!(out, "tf max"); }
        5 => { let _ = writeln!(out, "(1+42%)tf min"); }
        6 => { let _ = writeln!(out, "(1+35%)tf min"); }
        7 => { let _ = writeln!(out, "(1+28%)tf min"); }
        0 => { let _ = writeln!(out, "(1+21%)tf min"); }
        1 => { let _ = writeln!(out, "(1+14%)tf min"); }
        2 => { let _ = writeln!(out, "(1+7%)tf min"); }
        3 => { let _ = writeln!(out, "tf min"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_ispsize = (options[7] >> 4) & 0x0f;
    let _ = write!(out, "OP_ISPSIZE:: ");
    match op_ispsize {
        0 => { let _ = writeln!(out, "8 x 1024Bytes"); }
        1 => { let _ = writeln!(out, "7 x 1024Bytes"); }
        2 => { let _ = writeln!(out, "6 x 1024Bytes"); }
        3 => { let _ = writeln!(out, "5 x 1024Bytes"); }
        4 => { let _ = writeln!(out, "4 x 1024Bytes"); }
        5 => { let _ = writeln!(out, "3 x 1024Bytes"); }
        6 => { let _ = writeln!(out, "2 x 1024Bytes"); }
        7 => { let _ = writeln!(out, "1 x 1024Bytes"); }
        8 => { let _ = writeln!(out, "0 Bytes"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    let op_eepromsize = (options[7] >> 0) & 0x0f;
    let _ = write!(out, "OP_EEPROMSIZE:: ");
    match op_eepromsize {
        0 => { let _ = writeln!(out, "8 x 512Bytes"); }
        1 => { let _ = writeln!(out, "7 x 512Bytes"); }
        2 => { let _ = writeln!(out, "6 x 512Bytes"); }
        3 => { let _ = writeln!(out, "5 x 512Bytes"); }
        4 => { let _ = writeln!(out, "4 x 512Bytes"); }
        5 => { let _ = writeln!(out, "3 x 512Bytes"); }
        6 => { let _ = writeln!(out, "2 x 512Bytes"); }
        7 => { let _ = writeln!(out, "1 x 512Bytes"); }
        8 => { let _ = writeln!(out, "0 Bytes"); }
        v => { let _ = writeln!(out, "Unknown value: {}", v); }
    }
    out
}
