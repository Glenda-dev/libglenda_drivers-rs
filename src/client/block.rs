use crate::interface::BlockDriver;
use crate::io_uring::IoRingClient;
use crate::protocol::block;
use glenda::cap::{Endpoint, Frame};
use glenda::error::Error;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};

pub struct BlockClient {
    endpoint: Endpoint,
    ring: Option<IoRingClient>,
}

impl BlockClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint, ring: None }
    }

    pub fn set_ring(&mut self, ring: IoRingClient) {
        self.ring = Some(ring);
    }

    pub fn ring(&self) -> Option<&IoRingClient> {
        self.ring.as_ref()
    }

    /// Synchronous read using io_uring if available.
    pub fn read_blocks(&self, sector: u64, count: u32, buf: &mut [u8]) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotImplemented)?;
        let sqe = block::sqe_read(sector, buf.as_ptr() as u64, count, 0x1234);

        ring.submit(sqe)?;

        // Block until completion (simple poll for now)
        loop {
            if let Some(cqe) = ring.peek_completion() {
                if cqe.user_data == 0x1234 {
                    if cqe.res < 0 {
                        return Err(Error::Generic);
                    }
                    return Ok(());
                }
            }
            // In a real no_std environment, we might need a pause/yield or notify wait.
            // For probing, we can just block as we're the only user of this ring.
        }
    }

    /// Synchronous write using io_uring if available.
    pub fn write_blocks(&self, sector: u64, count: u32, buf: &[u8]) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotImplemented)?;
        let sqe = block::sqe_write(sector, buf.as_ptr() as u64, count, 0x5678);

        ring.submit(sqe)?;

        loop {
            if let Some(cqe) = ring.peek_completion() {
                if cqe.user_data == 0x5678 {
                    if cqe.res < 0 {
                        return Err(Error::Generic);
                    }
                    return Ok(());
                }
            }
        }
    }
}

impl BlockDriver for BlockClient {
    fn capacity(&self) -> u64 {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(0, block::GET_CAPACITY, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        if let Ok(_) = self.endpoint.call(&mut utcb) { utcb.get_mr(0) as u64 } else { 0 }
    }

    fn block_size(&self) -> u32 {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(0, block::GET_BLOCK_SIZE, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        if let Ok(_) = self.endpoint.call(&mut utcb) { utcb.get_mr(0) as u32 } else { 512 }
    }

    fn setup_ring(&mut self, sq_entries: u32, cq_entries: u32) -> Result<Frame, Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(0, block::SETUP_RING, MsgFlags::NONE);
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
