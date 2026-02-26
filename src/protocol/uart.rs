//! UART Device Protocol

/// Write a single character
pub const PUT_CHAR: usize = 0x01;
/// Read a single character (blocking?)
pub const GET_CHAR: usize = 0x02;
/// Write a string
pub const PUT_STR: usize = 0x03;
/// Configuration
pub const SET_BAUD_RATE: usize = 0x04;
pub const GET_CONFIG: usize = 0x05;

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

use glenda::io::uring::{IOURING_OP_READ, IOURING_OP_WRITE, IoUringSqe};

pub fn sqe_read(addr: u64, len: u32, user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: IOURING_OP_READ, addr, len, user_data, ..Default::default() }
}

pub fn sqe_write(addr: u64, len: u32, user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: IOURING_OP_WRITE, addr, len, user_data, ..Default::default() }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UartConfig {
    pub baud_rate: u32,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub parity: u8,
}
