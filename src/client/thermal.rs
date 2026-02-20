use glenda::cap::Endpoint;
use glenda::error::Error;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use crate::interface::ThermalDriver;
use crate::protocol::{THERMAL_PROTO, thermal};

pub struct ThermalClient {
    endpoint: Endpoint,
}

impl ThermalClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

impl ThermalDriver for ThermalClient {
    fn get_temperature(&self, zone: u32) -> Result<u32, Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(THERMAL_PROTO, thermal::GET_TEMPERATURE, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        utcb.set_mr(0, zone as usize);

        self.endpoint.call(&mut utcb)?;
        Ok(utcb.get_mr(0) as u32)
    }
}
