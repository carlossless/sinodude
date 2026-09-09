pub mod sinodude_serial;
pub mod sinolink;
pub use sinodude_serial::*;
pub use sinolink::*;

/// Result type for the [`Programmer`] trait; backends surface their errors as `Box<dyn Error>` so the CLI drives any backend uniformly.
pub type ProgResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Uniform interface implemented by every programmer backend, so `main.rs` dispatches operations identically.
pub trait Programmer {
    fn set_unlock_key(&mut self, key: [u8; 8]);
    fn read_init(&mut self) -> ProgResult<()>;
    fn write_init(&mut self) -> ProgResult<()>;
    fn erase_init(&mut self) -> ProgResult<()>;
    fn read_flash(&mut self) -> ProgResult<Vec<u8>>;
    fn mass_erase(&mut self) -> ProgResult<()>;
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
            fn mass_erase(&mut self) -> ProgResult<()> {
                Ok(<$t>::mass_erase(self)?)
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
            fn finish(&mut self) -> ProgResult<()> {
                Ok(<$t>::finish(self)?)
            }
        }
    };
}

impl_programmer!(SinodudeSerialProgrammer);
impl_programmer!(SinoLinkProgrammer);
