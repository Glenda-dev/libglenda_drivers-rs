use crate::interface::{BlockDriver, DriverClient};
use crate::protocol::{BLOCK_PROTO, block};
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};
use glenda::cap::{Endpoint, Frame};
use glenda::client::ResourceClient;
use glenda::error::Error;
use glenda::interface::MemoryService;
use glenda::io::uring::{IoUringBuffer, IoUringClient, RingParams};
use glenda::ipc::{Badge, MsgFlags, MsgTag, UTCB};
use glenda::mem::shm::{SharedMemory, ShmParams};

#[derive(Clone)]
pub struct BlockClient {
    endpoint: Endpoint,
    notify_ep: Option<Endpoint>,
    ring: Option<IoUringClient>,
    shm: Option<SharedMemory>,
    block_size: u32,
    total_sectors: u64,
    next_id: Arc<AtomicU64>,
    ring_params: RingParams,
    shm_params: ShmParams,
    res_client: ResourceClient,
}

impl DriverClient for BlockClient {
    fn connect(&mut self) -> Result<(), Error> {
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

        self.setup_ring_internal()?;
        self.setup_shm_internal()?;

        Ok(())
    }

    fn disconnect(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

impl BlockClient {
    pub fn new(
        endpoint: Endpoint,
        res_client: &mut ResourceClient,
        ring_params: RingParams,
        shm_params: ShmParams,
    ) -> Self {
        Self {
            endpoint,
            notify_ep: None,
            ring: None,
            shm: None,
            block_size: 0,
            total_sectors: 0,
            next_id: Arc::new(AtomicU64::new(0x1000)),
            ring_params,
            shm_params,
            res_client: res_client.clone(),
        }
    }

    pub fn total_sectors(&self) -> u64 {
        self.total_sectors
    }

    pub fn block_size(&self) -> u32 {
        self.block_size
    }

    pub fn ring(&self) -> Option<&IoUringClient> {
        self.ring.as_ref()
    }

    fn next_user_data(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn read_shm(&self, offset: u64, len: u32, shm_vaddr: usize) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotInitialized)?;
        if self.block_size == 0
            || offset % self.block_size as u64 != 0
            || len % self.block_size != 0
        {
            return Err(Error::InvalidArgs);
        }

        let id = self.next_user_data();
        let sqe = block::sqe_read(offset, shm_vaddr as u64, len, id);
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

    fn setup_ring_internal(&mut self) -> Result<(), Error> {
        let sq_entries = self.ring_params.sq_entries;
        let cq_entries = self.ring_params.cq_entries;
        let notify_ep = self.ring_params.notify_ep;
        let recv = self.ring_params.recv_slot;
        let vaddr = self.ring_params.vaddr;
        let size = self.ring_params.size;

        self.notify_ep = Some(notify_ep);
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(BLOCK_PROTO, block::SETUP_RING, MsgFlags::HAS_CAP);
        utcb.set_mr(0, sq_entries as usize);
        utcb.set_mr(1, cq_entries as usize);
        utcb.set_msg_tag(tag);
        utcb.set_cap_transfer(notify_ep.cap());
        utcb.set_recv_window(recv);
        self.endpoint.call(&mut utcb)?;

        let frame = Frame::from(recv);
        self.res_client.mmap(Badge::null(), frame.clone(), vaddr, size)?;
        let ring_buf = unsafe {
            IoUringBuffer::new(vaddr as *mut u8, size, sq_entries as u32, cq_entries as u32)
        };
        let mut ring = IoUringClient::new(ring_buf);
        ring.set_server_notify(self.endpoint);
        self.ring = Some(ring);
        Ok(())
    }

    fn setup_shm_internal(&mut self) -> Result<(), Error> {
        let frame = self.shm_params.frame.clone();
        let vaddr = self.shm_params.vaddr;
        let paddr = self.shm_params.paddr;
        let size = self.shm_params.size;
        let recv = self.shm_params.recv_slot;

        if frame.cap().is_null() {
            // BlockClient (for bare devices) expects to be provided a Frame (with physical address)
            // to pass down to the hardware-level driver.
            return Err(Error::NotInitialized);
        }

        // Send memory frame and physical address to the driver server.
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        utcb.set_cap_transfer(frame.cap());
        let tag = MsgTag::new(BLOCK_PROTO, block::SETUP_BUFFER, MsgFlags::HAS_CAP);
        utcb.set_mr(0, vaddr);
        utcb.set_mr(1, size);
        utcb.set_mr(2, paddr as usize);
        utcb.set_msg_tag(tag);
        utcb.set_recv_window(recv);
        self.endpoint.call(&mut utcb)?;

        if !utcb.get_msg_tag().flags().contains(MsgFlags::OK) {
            return Err(Error::Generic);
        }

        let mut shm = SharedMemory::new(frame, vaddr, size);
        shm.set_client_vaddr(vaddr);
        shm.set_paddr(paddr);
        self.shm = Some(shm);

        Ok(())
    }
}

impl BlockDriver for BlockClient {
    fn read_blocks(&self, sector: u64, count: u32, buf: &mut [u8]) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotInitialized)?;
        let shm = self.shm.as_ref().ok_or(Error::NotInitialized)?;

        if self.block_size == 0 {
            return Err(Error::NotInitialized);
        }

        let len = (count * self.block_size) as usize;
        if len > shm.size() {
            glenda::println!("Requested read length {} exceeds SHM size {}", len, shm.size());
            return Err(Error::InvalidArgs);
        }

        let id = self.next_user_data();
        let sqe = block::sqe_read(sector, shm.client_vaddr() as u64, count * self.block_size, id);
        ring.submit(sqe)?;

        let wait_ep = self.notify_ep.as_ref().unwrap_or(&self.endpoint);
        loop {
            if let Some(cqe) = ring.peek_completion() {
                if cqe.user_data == id {
                    if cqe.res < 0 {
                        return Err(Error::Generic);
                    }
                    // Copy back from SHM
                    let shm_buf =
                        unsafe { core::slice::from_raw_parts(shm.vaddr() as *const u8, len) };
                    let copy_len = core::cmp::min(len, buf.len());
                    buf[..copy_len].copy_from_slice(&shm_buf[..copy_len]);
                    return Ok(());
                }
            }
            ring.wait_for_completions(wait_ep)?;
        }
    }

    fn write_blocks(&self, sector: u64, count: u32, buf: &[u8]) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotInitialized)?;
        let shm = self.shm.as_ref().ok_or(Error::NotInitialized)?;

        if self.block_size == 0 {
            return Err(Error::NotInitialized);
        }

        let len = (count * self.block_size) as usize;
        if len > shm.size() {
            return Err(Error::InvalidArgs);
        }

        // Copy to SHM first
        let shm_buf = unsafe { core::slice::from_raw_parts_mut(shm.vaddr() as *mut u8, len) };
        let copy_len = core::cmp::min(len, buf.len());
        shm_buf[..copy_len].copy_from_slice(&buf[..copy_len]);

        let id = self.next_user_data();
        let sqe = block::sqe_write(sector, shm.client_vaddr() as u64, count * self.block_size, id);
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

    fn capacity(&self) -> u64 {
        self.total_sectors
    }

    fn block_size(&self) -> u32 {
        self.block_size
    }
}
