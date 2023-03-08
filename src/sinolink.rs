use rusb::*;
use std::{time::Duration, io::Write};
use hex_literal::*;
use std::fs::File;

/// Device specific endpoints
/// TODO: given it's one device this could all be hard-coded
#[derive(Debug)]
struct Endpoints {
    control: Endpoint,
    read: Endpoint,
    write: Endpoint,
}

/// Internal endpoint representations
#[derive(Debug, PartialEq, Clone)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8
}

pub struct Sinolink {
  device: Device<GlobalContext>,
  handle: DeviceHandle<GlobalContext>
}

impl Sinolink {
  fn find_sinolink() -> Device<GlobalContext> {
    for device in devices().unwrap().iter() {
      let device_desc = device.device_descriptor().unwrap();
      if device_desc.vendor_id() == 0x258a && device_desc.product_id() == 0x5007 {
          return device;
      }
    }

    panic!("nope");
  }


  pub fn new() -> Self {
    let device = Self::find_sinolink();

    let device_desc = device.device_descriptor().unwrap();

    println!(
      "Bus {:03} Device {:03} ID {:04x}:{:04x}",
      device.bus_number(),
      device.address(),
      device_desc.vendor_id(),
      device_desc.product_id()
    );

    let mut handle = device.open().unwrap();

    handle.reset().unwrap();

    // Fetch base configuration
    // let languages = handle.read_languages(timeout).unwrap();
    let active_config = handle.active_configuration().unwrap();

    println!("Active configuration: {}", active_config);

    let config_desc = device.config_descriptor(0).unwrap();

    let (mut write, mut read) = (None, None);

    for interface in config_desc.interfaces() {
        for interface_desc in interface.descriptors() {
            for endpoint_desc in interface_desc.endpoint_descriptors() {

                // Create an endpoint container
                let e = Endpoint {
                    config: config_desc.number(),
                    iface: interface_desc.interface_number(),
                    setting: interface_desc.setting_number(),
                    address: endpoint_desc.address(),
                };

                println!("Endpoint: {:?}", e);

                // Find the relevant endpoints
                match (endpoint_desc.transfer_type(), endpoint_desc.direction()) {
                    (TransferType::Bulk, Direction::In) => read = Some(e),
                    (TransferType::Bulk, Direction::Out) => write = Some(e),
                    (_, _) => continue,
                }
            }
        }
    }

    handle.claim_interface(0).unwrap();
    handle.set_active_configuration(1).unwrap();

    let read_addr = read.unwrap().address;
    let write_addr = write.unwrap().address;

    println!("READ & WRITE: {:02x} {:02x}", read_addr, write_addr);

    return Self {
      device: device,
      handle: handle
    }
  }

  pub fn init(&self) {
    let mut bufff: [u8; 64] = [0; 64];
    let length = self.read_control(
      0xc0,
      0,
      0,
      0,
      &mut bufff,
    );

    let mut buf2: [u8; 16] = [0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x04];
    let mut buf: [u8; 1024] = [0; 1024];

    self.read_chip(17, 0, 0, 0x0000, 0x0400, &mut buf);

    let b: [u8; 0] = [];
    self.write_control(
      0x40,
      18,
      1,
      0,
      &b,
    );

    self.write_control(
      0x40,
      16,
      0,
      0,
      &buf2,
    );

    let config: [u8; 1024] = hex!("
      7887bd070002040204000005000003010620000000000000000800000000000000000000000000000000000000000008a4e063c00f00008800000000000000000000010040ff0000fd8f3600000000000000000000000100b36300000000000000000000000000000000000000000000000000000000000000000000000000000002000080000000000000000000000000000000000000000000000000000000081c1106080f09000aff00000000000009120000050068f90a0000000000000004000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000001200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000023030820360705500000000000000000
    ");
    self.handle.write_bulk(0x02, &config, Duration::new(60, 0)).unwrap();

    self.read_chip(17, 0, 0, 0x0000, 0x0400, &mut buf);

    let mut buf_1: [u8; 16] = [0; 16];
    self.read_control(0xc0, 24, 1, 0, &mut buf_1);

    let mut buf_2: [u8; 2] = [0; 2];
    self.read_control(0xc0, 21, 1, 0, &mut buf_2);

    let mut buf_3: [u8; 16] = [0; 16];
    self.read_control(0xc0, 64, 0x0101, 0, &mut buf_3);

    let mut buf_01: [u8; 0x1024] = [0; 0x1024];
    self.read_chip(70, 1, 0, 0, 0x10, &mut buf_01);

    self.read_chip(68, 1, 1, 0x1209, 0x40, &mut buf);

    let mut buf_4: [u8; 5] = [0; 5];
    self.read_control(0xc0, 22, 1, 0, &mut buf_4);

    self.read_chip(68, 1, 1, 0x1200, 0x10, &mut buf2);

    self.read_chip(68, 1, 1, 0x1006, 0x04, &mut buf2);

    let mut buf_5: [u8; 5] = [0; 5];
    self.read_control(0xc0, 22, 1, 0, &mut buf_5);

    self.read_chip(68, 1, 1, 0x1100, 0x04, &mut buf2);

    let mut buf_6: [u8; 5] = [0; 5];
    self.read_control(0xc0, 22, 1, 0, &mut buf_6);

    self.read_chip(68, 1, 1, 0x1000, 0x40, &mut buf);

    self.read_chip(68, 1, 1, 0x1040, 0x40, &mut buf);

    let mut buf_7: [u8; 5] = [0; 5];
    self.read_control(0xc0, 22, 1, 0, &mut buf_7);
  }

  pub fn read_control(
      &self,
      request_type: u8,
      request: u8,
      value: u16,
      index: u16,
      buf: &mut [u8]
  ) -> usize {
    println!("Read CONTROL: {:02x} {:02} {:04x} {:04x}", request_type, request, value, index);
    let result = self.handle.read_control(request_type, request, value, index, buf, Duration::new(60, 0)).unwrap();
    println!("RESULT {:02x?}", &buf[0..result]); 
    return result
  }

  pub fn write_control(
      &self,
      request_type: u8,
      request: u8,
      value: u16,
      index: u16,
      buf: &[u8]
  ) -> usize {
    println!("Write CONTROL: {:02x} {:02} {:04x} {:04x}", request_type, request, value, index);
    println!("COMMAND {:02x?}", buf);
    let result = self.handle.write_control(request_type, request, value, index, buf, Duration::new(60, 0)).unwrap();
    return result
  }

  pub fn read_chip(&self, request: u8, mode1: u8, mode2: u8, addr: u16, length: u16, buf: &mut [u8]) {
    println!("Read CHIP: {:02} {:02x} {:02x} {:04x} {:04x}", request, mode1, mode2, addr, length);
    let write_buf: [u8; 16] = [
      0x00,
      mode1,
      (addr & 0xff) as u8, (addr >> 8) as u8,
      0x00,0x00,
      mode2,
      0x00,0x00,0x00,0x00,0x00,0x00,0x00,
      (length & 0xff) as u8, (length >> 8) as u8
    ];

    let ln = self.write_control(
      0x40, 
      request, 
      0, 
      0, 
      &write_buf
    );
    println!("ding");
    let len = self.handle.read_bulk(0x81, buf, Duration::new(60, 0)).unwrap();
    println!("Read {:}", len);
}

  pub fn read_flash(&self) {
    let mut file = File::create("dump.hex").unwrap();

    for addr in (0..65536).step_by(64) {
      let mut buff: [u8; 64] = [0; 64];
      self.read_chip(68, 0x01, 0x00, addr as u16, 64, &mut buff);
      for chunk in (&buff[0..64]).chunks(16) {
        for x in &chunk[0..16] {
          write!(file, "{:02X}", x).unwrap();
        }
        write!(file, "\n").unwrap();
      }
    }
  }
}
