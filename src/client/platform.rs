use glenda::cap::Endpoint;
use glenda::error::Error;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use crate::interface::{PlatformDriver, DriverClient};
use crate::protocol::{PLATFORM_PROTO, platform};

pub struct PlatformClient {
    endpoint: Endpoint,
}

impl DriverClient for PlatformClient {
    fn connect(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl PlatformClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

impl PlatformDriver for PlatformClient {
    fn set_sleep_state(&mut self, state: u32) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(PLATFORM_PROTO, platform::SET_SLEEP_STATE, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        utcb.set_mr(0, state as usize);

        self.endpoint.call(&mut utcb)?;
        Ok(())
    }

    fn reset(&mut self, warm: bool) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(PLATFORM_PROTO, platform::SYSTEM_RESET, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        utcb.set_mr(0, if warm { 0 } else { 1 });

        self.endpoint.call(&mut utcb)?;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(PLATFORM_PROTO, platform::SYSTEM_OFF, MsgFlags::NONE);
        utcb.set_msg_tag(tag);

        self.endpoint.call(&mut utcb)?;
        Ok(())
    }
}
