use crate::interface::AcpiDriver;
use crate::protocol::{ACPI_PROTO, acpi};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use glenda::cap::Endpoint;
use glenda::error::Error;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};

pub struct AcpiClient {
    endpoint: Endpoint,
}

impl AcpiClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

impl AcpiDriver for AcpiClient {
    fn evaluate_method(&mut self, path: &str, args: &[u64]) -> Result<Vec<u64>, Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();

        let args = (path.to_string(), Vec::from(args));

        unsafe { utcb.write_postcard::<(String, Vec<u64>)>(&args) }?;

        let tag = MsgTag::new(ACPI_PROTO, acpi::EVAL_METHOD, MsgFlags::HAS_BUFFER);
        utcb.set_msg_tag(tag);

        self.endpoint.call(&mut utcb)?;
        let results = unsafe { utcb.read_vec::<u64>().unwrap() };
        Ok(results)
    }
}
