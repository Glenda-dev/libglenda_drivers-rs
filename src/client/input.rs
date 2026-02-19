use glenda::cap::Endpoint;
use crate::interface::InputDriver;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use glenda::protocol::device::input::InputEvent;
use crate::protocol::{INPUT_PROTO, input};

pub struct InputClient {
    endpoint: Endpoint,
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
