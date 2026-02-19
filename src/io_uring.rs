use glenda::cap::{Endpoint, VSpace};
use glenda::error::Error;
use glenda::ipc::UTCB;
use glenda::mem::Perms;
use glenda::mem::io_uring::{
    CQE_SIZE, HEADER_SIZE, IoUringBuffer, IoUringCqe, IoUringSqe, SQE_SIZE,
};
use glenda::mem::shm::SharedMemory;

/// IoRing provides a unified management layer for shared-memory ring buffers.
/// It wraps the basic IoUringBuffer from glenda and adds OS-level integration.
pub struct IoRing {
    shm: SharedMemory,
}

impl IoRing {
    /// Create a new IoRing using a SharedMemory region and initialize its metadata.
    pub fn new(shm: SharedMemory, sq_entries: u32, cq_entries: u32) -> Result<Self, Error> {
        let required_size =
            HEADER_SIZE + (sq_entries as usize * SQE_SIZE) + (cq_entries as usize * CQE_SIZE);
        if shm.size() < required_size {
            return Err(Error::InvalidArgs);
        }

        unsafe {
            IoUringBuffer::new(shm.as_ptr(), shm.size(), sq_entries, cq_entries);
        }
        Ok(Self { shm })
    }

    /// Open an existing IoRing in-place from shared memory.
    pub fn attach(shm: SharedMemory) -> Result<Self, Error> {
        Ok(Self { shm })
    }

    /// Get the underlying low-level ring buffer controller.
    pub fn buffer(&self) -> IoUringBuffer {
        unsafe { IoUringBuffer::attach(self.shm.as_ptr(), self.shm.size()) }
    }

    /// Get the shared memory region information.
    pub fn shm(&self) -> &SharedMemory {
        &self.shm
    }

    /// Map the io_uring into the specified virtual memory space.
    pub fn map(&self, vspace: &VSpace, _vaddr: usize, perms: Perms) -> Result<(), Error> {
        self.shm.map(vspace, perms)
    }
}

/// Driver-side (Server) of an IoRing.
/// Handles popping requests and pushing completions with signaling.
pub struct IoRingServer {
    ring: IoRing,
    client_notify: Option<Endpoint>,
}

impl IoRingServer {
    pub fn new(ring: IoRing) -> Self {
        Self { ring, client_notify: None }
    }

    /// Configure an endpoint to notify the client when new completions are available.
    pub fn set_client_notify(&mut self, endpoint: Endpoint) {
        self.client_notify = Some(endpoint);
    }

    /// Pull the next submission queue entry (SQE) from the client.
    pub fn next_request(&self) -> Option<IoUringSqe> {
        self.ring.buffer().pop_sqe()
    }

    /// Push a completion queue entry (CQE) and optionally signal the client.
    pub fn complete(&self, user_data: u64, result: i32) -> Result<(), Error> {
        let cqe = IoUringCqe { user_data, res: result, flags: 0 };
        self.ring.buffer().push_cqe(cqe).map_err(|_| Error::OutOfMemory)?;

        if let Some(ref ep) = self.client_notify {
            let mut utcb = unsafe { UTCB::new() };
            let _ = ep.notify(&mut utcb);
        }
        Ok(())
    }

    /// Blocking wait for new submissions from the client.
    /// The provided endpoint should be the one the client uses for signaling.
    pub fn wait_for_requests(&self, endpoint: &Endpoint) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        endpoint.recv(&mut utcb)
    }

    pub fn ring(&self) -> &IoRing {
        &self.ring
    }
}

/// User-side (Client) of an IoRing.
/// Handles submitting requests and popping completions with signaling.
pub struct IoRingClient {
    ring: IoRing,
    server_notify: Option<Endpoint>,
}

impl IoRingClient {
    pub fn new(ring: IoRing) -> Self {
        Self { ring, server_notify: None }
    }

    /// Configure an endpoint to notify the server when new submissions are ready.
    pub fn set_server_notify(&mut self, endpoint: Endpoint) {
        self.server_notify = Some(endpoint);
    }

    /// Submit a new operation into the ring and signal the driver.
    pub fn submit(&self, sqe: IoUringSqe) -> Result<(), Error> {
        self.ring.buffer().push_sqe(sqe).map_err(|_| Error::OutOfMemory)?;
        if let Some(ref ep) = self.server_notify {
            let mut utcb = unsafe { UTCB::new() };
            let _ = ep.notify(&mut utcb);
        }
        Ok(())
    }

    /// Retrieve a completion from the driver, if any.
    pub fn peek_completion(&self) -> Option<IoUringCqe> {
        self.ring.buffer().pop_cqe()
    }

    /// Blocking wait for new completions from the driver.
    pub fn wait_for_completions(&self, endpoint: &Endpoint) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        endpoint.recv(&mut utcb)
    }

    pub fn ring(&self) -> &IoRing {
        &self.ring
    }
}
