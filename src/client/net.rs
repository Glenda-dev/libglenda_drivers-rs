use crate::interface::NetDriver;
use crate::protocol::{NET_PROTO, net};
use core::sync::atomic::{AtomicU64, Ordering};
use glenda::cap::{CapPtr, Endpoint, Frame};
use glenda::error::Error;
use glenda::io::uring::IoUringClient;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use glenda::mem::shm::SharedMemory;
use glenda::protocol::device::net::MacAddress;

pub struct NetClient {
    endpoint: Endpoint,
    notify_ep: Option<Endpoint>,
    ring: Option<IoUringClient>,
    shm: Option<SharedMemory>,
    next_id: AtomicU64,
}

impl NetClient {
    pub const fn new(endpoint: Endpoint) -> Self {
        Self { endpoint, notify_ep: None, ring: None, shm: None, next_id: AtomicU64::new(0x1000) }
    }

    pub fn set_shm(&mut self, shm: SharedMemory) {
        self.shm = Some(shm);
    }

    pub fn shm(&self) -> Option<&SharedMemory> {
        self.shm.as_ref()
    }

    pub fn set_ring(&mut self, mut ring: IoUringClient) {
        ring.set_server_notify(self.endpoint);
        self.ring = Some(ring);
    }

    pub fn ring(&self) -> Option<&IoUringClient> {
        self.ring.as_ref()
    }

    fn next_user_data(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
    }

    pub fn send_packet(&self, buf: &[u8]) -> Result<(), Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotInitialized)?;
        let id = self.next_user_data();

        // Use SHM address if buffer is within SHM
        let addr = if let Some(shm) = &self.shm {
            if shm.contains_ptr(buf.as_ptr()) {
                shm.client_vaddr_at(buf.as_ptr()) as u64
            } else {
                buf.as_ptr() as u64
            }
        } else {
            buf.as_ptr() as u64
        };

        let sqe = net::sqe_send(addr, buf.len() as u32, id);

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

    pub fn recv_packet(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let ring = self.ring.as_ref().ok_or(Error::NotInitialized)?;
        let id = self.next_user_data();

        // Use SHM address if buffer is within SHM
        let addr = if let Some(shm) = &self.shm {
            if shm.contains_ptr(buf.as_ptr()) {
                shm.client_vaddr_at(buf.as_ptr()) as u64
            } else {
                buf.as_ptr() as u64
            }
        } else {
            buf.as_ptr() as u64
        };

        let sqe = net::sqe_recv(addr, buf.len() as u32, id);

        ring.submit(sqe)?;

        let wait_ep = self.notify_ep.as_ref().unwrap_or(&self.endpoint);
        loop {
            if let Some(cqe) = ring.peek_completion() {
                if cqe.user_data == id {
                    if cqe.res < 0 {
                        return Err(Error::Generic);
                    }
                    return Ok(cqe.res as usize);
                }
            }
            ring.wait_for_completions(wait_ep)?;
        }
    }
}

impl NetDriver for NetClient {
    fn mac_address(&self) -> MacAddress {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(NET_PROTO, net::GET_MAC, MsgFlags::NONE);
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

    fn setup_ring(
        &mut self,
        sq_entries: u32,
        cq_entries: u32,
        notify_ep: Endpoint,
        recv: CapPtr,
    ) -> Result<Frame, Error> {
        self.notify_ep = Some(notify_ep);
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        utcb.set_recv_window(recv);
        utcb.set_cap_transfer(notify_ep.cap());
        let tag = MsgTag::new(NET_PROTO, net::SETUP_RING, MsgFlags::HAS_CAP);
        utcb.set_mr(0, sq_entries as usize);
        utcb.set_mr(1, cq_entries as usize);
        utcb.set_msg_tag(tag);

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
        let tag = MsgTag::new(NET_PROTO, net::SETUP_BUFFER, MsgFlags::HAS_CAP);
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
