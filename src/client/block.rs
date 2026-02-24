use crate::interface::BlockDriver;
use crate::protocol::{BLOCK_PROTO, block};
use core::sync::atomic::{AtomicU64, Ordering};
use glenda::cap::{CapPtr, Endpoint, Frame};
use glenda::error::Error;
use glenda::io::uring::IoUringClient;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use glenda::mem::shm::SharedMemory;

pub struct BlockClient {
    endpoint: Endpoint,
    notify_ep: Option<Endpoint>,
    ring: Option<IoUringClient>,
    shm: Option<SharedMemory>,
    block_size: u32,
    total_sectors: u64,
    next_id: AtomicU64,
}

impl BlockClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self {
            endpoint,
            notify_ep: None,
            ring: None,
            shm: None,
            block_size: 0,
            total_sectors: 0,
            next_id: AtomicU64::new(0x1000),
        }
    }

    pub fn init(&mut self) -> Result<(), Error> {
        let tag = MsgTag::new(BLOCK_PROTO, block::GET_BLOCK_SIZE, MsgFlags::NONE);
        let u = unsafe { UTCB::new() };
        u.set_msg_tag(tag);
        self.endpoint.call(u)?;

        if !u.get_msg_tag().flags().contains(MsgFlags::OK) {
            return Err(Error::Generic);
        }

        self.block_size = u.get_mr(0) as u32;

        let tag = MsgTag::new(BLOCK_PROTO, block::GET_CAPACITY, MsgFlags::NONE);
        u.set_msg_tag(tag);
        self.endpoint.call(u)?;

        if !u.get_msg_tag().flags().contains(MsgFlags::OK) {
            return Err(Error::Generic);
        }

        self.total_sectors = u.get_mr(0) as u64;

        Ok(())
    }

    pub fn block_size(&self) -> u32 {
        self.block_size
    }

    pub fn total_sectors(&self) -> u64 {
        self.total_sectors
    }

    pub fn endpoint(&self) -> Endpoint {
        self.endpoint
    }

    pub fn set_shm(&mut self, shm: SharedMemory) {
        self.shm = Some(shm);
    }

    pub fn shm(&self) -> Option<&SharedMemory> {
        self.shm.as_ref()
    }

    pub fn set_ring(&mut self, mut ring: IoUringClient) {
        ring.set_server_notify(self.endpoint);
        if self.notify_ep.is_some() {
            ring.set_notify_tag(MsgTag::new(
                BLOCK_PROTO,
                block::NOTIFY_SQ,
                glenda::ipc::MsgFlags::NONE,
            ));
        }
        self.ring = Some(ring);
    }

    pub fn ring(&self) -> Option<&IoUringClient> {
        self.ring.as_ref()
    }

    fn next_user_data(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Read data from disk directly to a shared memory address.
    /// This assumes shm_vaddr is within the shm region provided to set_shm.
    pub fn read_shm(&self, offset: u64, len: u32, shm_vaddr: usize) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotInitialized)?;
        let _shm = self.shm.as_ref().ok_or(Error::NotInitialized)?;

        // Ensure alignment to block_size
        if self.block_size == 0
            || offset % self.block_size as u64 != 0
            || len % self.block_size != 0
        {
            return Err(Error::InvalidArgs);
        }

        let id = self.next_user_data();

        let sqe = block::sqe_read(offset, shm_vaddr as u64, len, id);
        ring.submit(sqe)?;

        // Block until completion
        let wait_ep = self.notify_ep.as_ref().unwrap_or(&self.endpoint);
        loop {
            if let Some(cqe) = ring.peek_completion() {
                if cqe.user_data == id {
                    if cqe.res < 0 {
                        return Err(Error::Generic);
                    }
                    return Ok(());
                }
            }
            ring.wait_for_completions(wait_ep)?;
        }
    }

    /// Read data at byte offset and length.
    /// Both offset and len MUST be aligned to block_size.
    pub fn read_at(&self, offset: u64, len: u32, buf: &mut [u8]) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotInitialized)?;
        let shm = self.shm.as_ref().ok_or(Error::NotInitialized)?;

        // Ensure alignment to block_size
        if self.block_size == 0
            || offset % self.block_size as u64 != 0
            || len % self.block_size != 0
        {
            return Err(Error::InvalidArgs);
        }

        if len as usize > shm.size() {
            return Err(Error::InvalidArgs);
        }

        let id = self.next_user_data();

        // Use the beginning of SHM for synchronous operations
        // We use client_vaddr because that's what the server expects.
        let src_addr = shm.client_vaddr() as u64;

        let sqe = block::sqe_read(offset, src_addr, len, id);
        ring.submit(sqe)?;

        // Block until completion
        let wait_ep = self.notify_ep.as_ref().unwrap_or(&self.endpoint);
        loop {
            if let Some(cqe) = ring.peek_completion() {
                if cqe.user_data == id {
                    if cqe.res < 0 {
                        return Err(Error::Generic);
                    }
                    // Copy back from SHM
                    let shm_buf = unsafe {
                        core::slice::from_raw_parts(shm.vaddr() as *const u8, len as usize)
                    };
                    let copy_len = core::cmp::min(len as usize, buf.len());
                    buf[..copy_len].copy_from_slice(&shm_buf[..copy_len]);
                    return Ok(());
                }
            }
            ring.wait_for_completions(wait_ep)?;
        }
    }

    /// Write data at byte offset and length.
    /// Both offset and len MUST be aligned to block_size.
    pub fn write_at(&self, offset: u64, len: u32, buf: &[u8]) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotInitialized)?;
        let shm = self.shm.as_ref().ok_or(Error::NotInitialized)?;

        // Ensure alignment to block_size
        if self.block_size == 0
            || offset % self.block_size as u64 != 0
            || len % self.block_size != 0
        {
            return Err(Error::InvalidArgs);
        }

        if len as usize > shm.size() {
            return Err(Error::InvalidArgs);
        }

        let id = self.next_user_data();

        // Copy to SHM first
        let shm_buf =
            unsafe { core::slice::from_raw_parts_mut(shm.vaddr() as *mut u8, len as usize) };
        let copy_len = core::cmp::min(len as usize, buf.len());
        shm_buf[..copy_len].copy_from_slice(&buf[..copy_len]);

        // Use the beginning of SHM for synchronous operations
        // We use client_vaddr because that's what the server expects.
        let dst_addr = shm.client_vaddr() as u64;

        let sqe = block::sqe_write(offset, dst_addr, len, id);
        ring.submit(sqe)?;

        let wait_ep = self.notify_ep.as_ref().unwrap_or(&self.endpoint);
        loop {
            if let Some(cqe) = ring.peek_completion() {
                if cqe.user_data == id {
                    if cqe.res < 0 {
                        return Err(Error::Generic);
                    }
                    return Ok(());
                }
            }
            ring.wait_for_completions(wait_ep)?;
        }
    }

    /// Synchronous read using io_uring (compat).
    pub fn read_blocks(&self, sector: u64, count: u32, buf: &mut [u8]) -> Result<(), Error> {
        self.read_at(sector * self.block_size as u64, count * self.block_size, buf)
    }

    /// Synchronous write using io_uring (compat).
    pub fn write_blocks(&self, sector: u64, count: u32, buf: &[u8]) -> Result<(), Error> {
        self.write_at(sector * self.block_size as u64, count * self.block_size, buf)
    }
}

impl BlockDriver for BlockClient {
    fn capacity(&self) -> u64 {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(BLOCK_PROTO, block::GET_CAPACITY, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        if let Ok(_) = self.endpoint.call(&mut utcb) { utcb.get_mr(0) as u64 } else { 0 }
    }

    fn block_size(&self) -> u32 {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(BLOCK_PROTO, block::GET_BLOCK_SIZE, MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        if let Ok(_) = self.endpoint.call(&mut utcb) { utcb.get_mr(0) as u32 } else { 4096 }
    }

    fn setup_ring(
        &mut self,
        sq_entries: u32,
        cq_entries: u32,
        notify_ep: Endpoint,
        recv: CapPtr,
    ) -> Result<Frame, Error> {
        self.notify_ep = Some(notify_ep);
        if let Some(ref mut ring) = self.ring {
            ring.set_notify_tag(MsgTag::new(
                BLOCK_PROTO,
                block::NOTIFY_SQ,
                glenda::ipc::MsgFlags::NONE,
            ));
        }
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(BLOCK_PROTO, block::SETUP_RING, MsgFlags::HAS_CAP);
        utcb.set_mr(0, sq_entries as usize);
        utcb.set_mr(1, cq_entries as usize);
        utcb.set_msg_tag(tag);
        utcb.set_cap_transfer(notify_ep.cap());
        utcb.set_recv_window(recv);
        self.endpoint.call(&mut utcb)?;
        Ok(Frame::from(recv))
    }

    fn setup_shm(
        &mut self,
        frame: Frame,
        vaddr: usize,
        paddr: u64,
        size: usize,
    ) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        utcb.set_cap_transfer(frame.cap());
        let tag = MsgTag::new(BLOCK_PROTO, block::SETUP_BUFFER, MsgFlags::HAS_CAP);
        utcb.set_mr(0, vaddr);
        utcb.set_mr(1, size);
        utcb.set_mr(2, paddr as usize);
        utcb.set_msg_tag(tag);

        self.endpoint.call(&mut utcb)?;

        let mut shm = SharedMemory::new(frame, vaddr, size);
        shm.set_paddr(paddr);
        self.set_shm(shm);
        Ok(())
    }
}

impl BlockClient {
    pub fn request_shm(&self, recv: CapPtr) -> Result<(Frame, usize, usize, usize), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(BLOCK_PROTO, block::SETUP_BUFFER, glenda::ipc::MsgFlags::NONE);
        utcb.set_msg_tag(tag);
        // We are requesting a cap, so set recv window
        utcb.set_recv_window(recv);

        self.endpoint.call(&mut utcb)?;

        let fossil_vaddr = utcb.get_mr(0);
        let size = utcb.get_mr(1);
        let paddr = utcb.get_mr(2);

        Ok((Frame::from(recv), fossil_vaddr, size, paddr))
    }
}
