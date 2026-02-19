use glenda::cap::Endpoint;
use crate::interface::UartDriver;
use glenda::ipc::IPC_BUFFER_SIZE;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use crate::protocol::{UART_PROTO, uart};

pub struct UartClient {
    endpoint: Endpoint,
}

impl UartClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

impl UartDriver for UartClient {
    fn put_char(&mut self, c: u8) {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(UART_PROTO, uart::PUT_CHAR, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        utcb.set_mr(0, c as usize);

        let _ = self.endpoint.call(&mut utcb);
    }

    fn get_char(&mut self) -> Option<u8> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(UART_PROTO, uart::GET_CHAR, MsgFlags::NONE);
        utcb.set_msg_tag(tag);

        match self.endpoint.call(&mut utcb) {
            Ok(_) => Some(utcb.get_mr(0) as u8),
            Err(_) => None,
        }
    }

    fn put_str(&mut self, s: &str) {
        let bytes = s.as_bytes();
        for chunk in bytes.chunks(IPC_BUFFER_SIZE) {
            let mut utcb = unsafe { UTCB::new() };
            utcb.clear();
            let buf = &mut utcb.ipc_buffer();
            buf[..chunk.len()].copy_from_slice(chunk);
            let tag = MsgTag::new(UART_PROTO, uart::PUT_STR, MsgFlags::NONE);
            utcb.set_msg_tag(tag);
            utcb.set_size(chunk.len());

            let _ = self.endpoint.call(&mut utcb);
        }
    }
}
