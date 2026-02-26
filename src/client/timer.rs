use glenda::cap::Endpoint;
use glenda::error::Error;
use crate::interface::{TimerDriver, DriverClient};
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use crate::protocol::{TIMER_PROTO, timer};
use glenda::set_mrs;

pub struct TimerClient {
    endpoint: Endpoint,
    freq: u64,
}

impl DriverClient for TimerClient {
    fn connect(&mut self) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(TIMER_PROTO, timer::GET_FREQ, MsgFlags::NONE);
        utcb.set_msg_tag(tag);

        self.endpoint.call(&mut utcb)?;
        self.freq = utcb.get_mr(0) as u64;
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl TimerClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint, freq: 0 }
    }

    pub fn freq(&self) -> u64 {
        self.freq
    }
}

impl TimerDriver for TimerClient {
    fn get_time(&self) -> u64 {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(TIMER_PROTO, timer::GET_TIME, MsgFlags::NONE);
        utcb.set_msg_tag(tag);

        if self.endpoint.call(&mut utcb).is_ok() { utcb.get_mr(0) as u64 } else { 0 }
    }

    fn set_time(&mut self, timestamp: u64) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(TIMER_PROTO, timer::SET_TIME, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        set_mrs!(utcb, timestamp as usize);
        self.endpoint.call(&mut utcb)
    }

    fn set_alarm(&mut self, timestamp: u64) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(TIMER_PROTO, timer::SET_ALARM, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        set_mrs!(utcb, timestamp as usize);
        self.endpoint.call(&mut utcb)
    }

    fn stop_alarm(&mut self) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(TIMER_PROTO, timer::STOP_ALARM, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        self.endpoint.call(&mut utcb)
    }
}
