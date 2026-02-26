use crate::interface::{InputDriver, DriverClient};
use crate::protocol::input::InputEvent;
use crate::protocol::{INPUT_PROTO, input};
use glenda::cap::Endpoint;
use glenda::error::Error;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};

pub struct InputClient {
    endpoint: Endpoint,
}

impl DriverClient for InputClient {
    fn connect(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl InputClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

impl InputDriver for InputClient {
    fn poll_event(&mut self) -> Option<InputEvent> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(INPUT_PROTO, input::READ_EVENT, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        if self.endpoint.call(&mut utcb).is_ok() {
            unsafe { utcb.read_obj::<InputEvent>().ok() }
        } else {
            None
        }
    }
}
