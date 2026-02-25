//! Block Device Protocol (Ring Only)

/// Get device capacity in blocks
pub const GET_CAPACITY: usize = 0x1;
/// Get block size in bytes
pub const GET_BLOCK_SIZE: usize = 0x2;
/// Setup Partition Info (Optional, for multi-partition devices)
/// Args: start_sector, num_sectors
pub const SETUP_PARTITION: usize = 0x3;
/// Setup io_uring (Primary IO Channel).
/// Args: sq_entries, cq_entries
/// Resp: Cap Transfer (Frame)
pub const SETUP_RING: usize = 0x10;
/// Setup shared memory buffer for IO data.
/// Args: pages (if creating) or empty (if requesting)
/// Resp: Cap Transfer (Frame)
pub const SETUP_BUFFER: usize = 0x11;
/// Notify the driver that new requests are in the SQ.
pub const NOTIFY_SQ: usize = 0x12;

/// Async notification for IO completion
pub const NOTIFY_IO: usize = 0x20;

use glenda::io::uring::{IOURING_OP_READ, IOURING_OP_SYNC, IOURING_OP_WRITE, IoUringSqe};

pub fn sqe_read(sector: u64, addr: u64, len: u32, user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: IOURING_OP_READ, off: sector, addr, len, user_data, ..Default::default() }
}

pub fn sqe_write(sector: u64, addr: u64, len: u32, user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: IOURING_OP_WRITE, off: sector, addr, len, user_data, ..Default::default() }
}

pub fn sqe_sync(user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: IOURING_OP_SYNC, user_data, ..Default::default() }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BlockRequest {
    pub sector: u64,
    pub count: u32,
    pub flags: u32,
}
