use glenda::cap::{Endpoint, Frame};
use glenda::error::Error;
use crate::interface::NetDriver;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use crate::protocol::net;
use glenda::protocol::device::net::MacAddress;

pub struct NetClient {
    endpoint: Endpoint,
}

impl NetClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }
}

impl NetDriver for NetClient {
    fn mac_address(&self) -> MacAddress {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(0, net::GET_MAC, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        if let Ok(_) = self.endpoint.call(&mut utcb) {
             let mut mac = [0u8; 6];
             let val1 = utcb.get_mr(0) as u32;
             let val2 = utcb.get_mr(1) as u32;
             mac[0] = (val1 & 0xFF) as u8;
             mac[1] = ((val1 >> 8) & 0xFF) as u8;
             mac[2] = ((val1 >> 16) & 0xFF) as u8;
             mac[3] = ((val1 >> 24) & 0xFF) as u8;
             mac[4] = (val2 & 0xFF) as u8;
             mac[5] = ((val2 >> 8) & 0xFF) as u8;
             MacAddress { octets: mac }
        } else {
             MacAddress { octets: [0; 6] }
        }
    }

    fn setup_ring(&mut self, sq_entries: u32, cq_entries: u32) -> Result<Frame, Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(0, net::SETUP_RING, MsgFlags::NONE);
        utcb.set_mr(0, sq_entries as usize);
        utcb.set_mr(1, cq_entries as usize);
        utcb.set_msg_tag(tag);
        
        self.endpoint.call(&mut utcb)?;
        
        let tag = utcb.get_msg_tag();
        if tag.flags().contains(MsgFlags::ERROR) {
            return Err(Error::Generic);
        }
        
        if !tag.flags().contains(MsgFlags::HAS_CAP) {
            return Err(Error::NotFound);
        }
        
        let frame_cap = utcb.get_cap_transfer();
        Ok(Frame::from(frame_cap))
    }
}
