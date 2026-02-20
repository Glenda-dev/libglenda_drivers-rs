use glenda::cap::Endpoint;
use glenda::error::Error;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use crate::interface::BatteryDriver;
use crate::protocol::{BATTERY_PROTO, battery};

pub struct BatteryClient {
    endpoint: Endpoint,
}

impl BatteryClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

impl BatteryDriver for BatteryClient {
    fn get_power_source(&self) -> Result<u32, Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(BATTERY_PROTO, battery::GET_POWER_SOURCE, MsgFlags::NONE);
        utcb.set_msg_tag(tag);

        self.endpoint.call(&mut utcb)?;
        Ok(utcb.get_mr(0) as u32)
    }

    fn get_level(&self) -> Result<u32, Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(BATTERY_PROTO, battery::GET_LEVEL, MsgFlags::NONE);
        utcb.set_msg_tag(tag);

        self.endpoint.call(&mut utcb)?;
        Ok(utcb.get_mr(0) as u32)
    }

    fn get_status(&self) -> Result<u32, Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(BATTERY_PROTO, battery::GET_STATUS, MsgFlags::NONE);
        utcb.set_msg_tag(tag);

        self.endpoint.call(&mut utcb)?;
        Ok(utcb.get_mr(0) as u32)
    }
}
