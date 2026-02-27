use crate::interface::{DriverClient, InputDriver};
use crate::protocol::input::{InputEvent, SETUP_URING};
use crate::protocol::{INPUT_PROTO, input};
use glenda::cap::{Endpoint, Frame};
use glenda::error::Error;
use glenda::io::uring::IoUringBuffer;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};

pub struct InputClient {
    endpoint: Endpoint,
    ring: Option<IoUringBuffer>,
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
        Self { endpoint, ring: None }
    }

    /// Setup io_uring for zero-copy event delivery.
    pub fn setup_uring(&mut self, entries: u32) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(INPUT_PROTO, SETUP_URING, MsgFlags::NONE);
        // tag.set_mr(0, entries as u64) is incorrect, should use utcb.set_mr
        utcb.set_mr(0, entries as usize);
        utcb.set_msg_tag(tag);

        self.endpoint.call(&mut utcb)?;

        let resp_tag = utcb.get_msg_tag();
        if resp_tag.label() != 0 {
            return Err(Error::InvalidType);
        }

        // The server should have returned a Frame capability in our CSpace
        // and also the address in the message registers.
        let frame_cap = utcb.get_cap_transfer();
        if frame_cap.is_null() {
            return Err(Error::OutOfMemory);
        }

        let frame = Frame::from(frame_cap);
        let size = (entries as usize * 80 + 4095) & !4095;

        // Map the frame into our address space.
        // In a real userspace app, we'd use a VSpace manager, here we assume a fixed or managed region.
        let vaddr = 0x80000000; // Placeholder for client-side mapping
        glenda::cap::VSPACE_CAP.map(
            frame,
            vaddr,
            glenda::mem::Perms::READ | glenda::mem::Perms::WRITE,
            (size + 4095) / 4096,
        )?;

        self.ring = Some(unsafe { IoUringBuffer::new(vaddr as *mut u8, size, entries, entries) });

        Ok(())
    }
}

impl InputDriver for InputClient {
    fn poll_event(&mut self) -> Option<InputEvent> {
        if let Some(ref mut _ring) = self.ring {
            // Try to get from ring
            None // TODO: implement ring polling
        } else {
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
}
