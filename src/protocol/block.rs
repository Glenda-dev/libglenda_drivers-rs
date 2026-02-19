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
/// Notify the driver that new requests are in the SQ.
pub const NOTIFY_SQ: usize = 0x11;

use glenda::mem::io_uring::{IORING_OP_READ, IORING_OP_SYNC, IORING_OP_WRITE, IoUringSqe};

pub fn sqe_read(sector: u64, addr: u64, len: u32, user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: IORING_OP_READ, off: sector, addr, len, user_data, ..Default::default() }
}

pub fn sqe_write(sector: u64, addr: u64, len: u32, user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: IORING_OP_WRITE, off: sector, addr, len, user_data, ..Default::default() }
}

pub fn sqe_sync(user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: IORING_OP_SYNC, user_data, ..Default::default() }
}
