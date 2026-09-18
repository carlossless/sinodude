pub mod sinodude_serial;
pub mod sinolink;
pub use sinodude_serial::*;
pub use sinolink::*;

/// Per-group read and write protection, plus the raw security record it was decoded from.
pub struct Protection {
    pub read: Vec<bool>,
    pub write: Vec<bool>,
    pub record: Vec<u8>,
}

/// Backends surface their own error types as `Box<dyn Error>` so the CLI drives any of them
/// through one interface.
pub type ProgResult<T> = Result<T, Box<dyn std::error::Error>>;

/// What every programmer backend has to provide for the CLI to drive it.
pub trait Programmer {
    fn set_unlock_key(&mut self, key: [u8; 8]);
    fn read_init(&mut self) -> ProgResult<()>;
    fn write_init(&mut self) -> ProgResult<()>;
    fn erase_init(&mut self) -> ProgResult<()>;
    fn read_flash(&mut self) -> ProgResult<Vec<u8>>;
    fn read_eeprom(&mut self) -> ProgResult<Vec<u8>>;
    fn write_eeprom(&mut self, data: &[u8]) -> ProgResult<()>;
    fn erase_eeprom(&mut self) -> ProgResult<()>;
    fn mass_erase(&mut self) -> ProgResult<()>;
    fn mass_erase_with_mode(&mut self, mode: u8) -> ProgResult<()>;
    fn erase_sectors(&mut self, start_addr: u32, end_addr: u32) -> ProgResult<()>;
    #[allow(clippy::too_many_arguments)]
    fn write_custom_fields(
        &mut self,
        customer_id: Option<&[u8; 4]>,
        operation_number: Option<&[u8; 2]>,
        customer_option: Option<&[u8]>,
        security: Option<&[u8]>,
        serial_number: Option<&[u8; 4]>,
        use_stored_defaults: bool,
    ) -> ProgResult<()>;
    fn write_flash(&mut self, firmware: &[u8]) -> ProgResult<()>;
    fn write_flash_range(&mut self, firmware: &[u8], start: usize, end: usize) -> ProgResult<()>;
    fn read_protection(&mut self) -> ProgResult<Protection>;
    fn apply_protection(
        &mut self,
        read: &[bool],
        write: &[bool],
        new_password: Option<&[u8; crate::parts::PROTECTION_PASSWORD_LEN]>,
    ) -> ProgResult<()>;
    fn finish(&mut self) -> ProgResult<()>;
}

/// Implement [`Programmer`] for a backend by forwarding to its inherent methods of the same name.
macro_rules! impl_programmer {
    ($t:ty) => {
        impl Programmer for $t {
            fn set_unlock_key(&mut self, key: [u8; 8]) {
                <$t>::set_unlock_key(self, key)
            }
            fn read_init(&mut self) -> ProgResult<()> {
                Ok(<$t>::read_init(self)?)
            }
            fn write_init(&mut self) -> ProgResult<()> {
                Ok(<$t>::write_init(self)?)
            }
            fn erase_init(&mut self) -> ProgResult<()> {
                Ok(<$t>::erase_init(self)?)
            }
            fn read_flash(&mut self) -> ProgResult<Vec<u8>> {
                Ok(<$t>::read_flash(self)?)
            }
            fn read_eeprom(&mut self) -> ProgResult<Vec<u8>> {
                Ok(<$t>::read_eeprom(self)?)
            }
            fn write_eeprom(&mut self, data: &[u8]) -> ProgResult<()> {
                Ok(<$t>::write_eeprom(self, data)?)
            }
            fn erase_eeprom(&mut self) -> ProgResult<()> {
                Ok(<$t>::erase_eeprom(self)?)
            }
            fn mass_erase(&mut self) -> ProgResult<()> {
                Ok(<$t>::mass_erase(self)?)
            }
            fn mass_erase_with_mode(&mut self, mode: u8) -> ProgResult<()> {
                Ok(<$t>::mass_erase_with_mode(self, mode)?)
            }
            fn erase_sectors(&mut self, start_addr: u32, end_addr: u32) -> ProgResult<()> {
                Ok(<$t>::erase_sectors(self, start_addr, end_addr)?)
            }
            fn write_custom_fields(
                &mut self,
                customer_id: Option<&[u8; 4]>,
                operation_number: Option<&[u8; 2]>,
                customer_option: Option<&[u8]>,
                security: Option<&[u8]>,
                serial_number: Option<&[u8; 4]>,
                use_stored_defaults: bool,
            ) -> ProgResult<()> {
                Ok(<$t>::write_custom_fields(
                    self,
                    customer_id,
                    operation_number,
                    customer_option,
                    security,
                    serial_number,
                    use_stored_defaults,
                )?)
            }
            fn write_flash(&mut self, firmware: &[u8]) -> ProgResult<()> {
                Ok(<$t>::write_flash(self, firmware)?)
            }
            fn write_flash_range(
                &mut self,
                firmware: &[u8],
                start: usize,
                end: usize,
            ) -> ProgResult<()> {
                Ok(<$t>::write_flash_range(self, firmware, start, end)?)
            }
            fn read_protection(&mut self) -> ProgResult<Protection> {
                Ok(<$t>::read_protection(self)?)
            }
            fn apply_protection(
                &mut self,
                read: &[bool],
                write: &[bool],
                new_password: Option<&[u8; crate::parts::PROTECTION_PASSWORD_LEN]>,
            ) -> ProgResult<()> {
                Ok(<$t>::apply_protection(self, read, write, new_password)?)
            }
            fn finish(&mut self) -> ProgResult<()> {
                Ok(<$t>::finish(self)?)
            }
        }
    };
}

impl_programmer!(SinodudeSerialProgrammer);
impl_programmer!(SinoLinkProgrammer);
