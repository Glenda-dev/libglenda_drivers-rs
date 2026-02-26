use crate::interface::{FrameBufferDriver, DriverClient};
use crate::protocol::fb::FbInfo;
use crate::protocol::{FB_PROTO, fb};
use glenda::cap::Endpoint;
use glenda::error::Error;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use glenda::set_mrs;

pub struct FbClient {
    endpoint: Endpoint,
    info: FbInfo,
}

impl DriverClient for FbClient {
    fn connect(&mut self) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(FB_PROTO, fb::GET_INFO, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        self.endpoint.call(&mut utcb)?;
        self.info = unsafe { utcb.read_obj::<FbInfo>().unwrap_or(FbInfo::default()) };
        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl FbClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { 
            endpoint, 
            info: FbInfo {
                width: 0,
                height: 0,
                pitch: 0,
                format: 0,
                bpp: 0,
                paddr: 0,
                size: 0,
            } 
        }
    }

    pub fn info(&self) -> &FbInfo {
        &self.info
    }
}

impl FrameBufferDriver for FbClient {
    fn get_info(&self) -> FbInfo {
        self.info.clone()
    }

    fn flush(&mut self, x: u32, y: u32, w: u32, h: u32) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(FB_PROTO, fb::FLUSH, MsgFlags::NONE);
        set_mrs!(utcb, x, y, w, h);
        utcb.set_msg_tag(tag);
        self.endpoint.call(&mut utcb)
    }
}
