//! Ethernet Device Protocol (Ring Only)

/// Get MAC Address
pub const GET_MAC: usize = 0x1;

/// Setup io_uring (Primary IO Channel).
/// Args: sq_entries, cq_entries
/// Resp: Cap Transfer (Frame)
pub const SETUP_RING: usize = 0x10;

/// Setup shared memory buffer for packet data.
pub const SETUP_BUFFER: usize = 0x11;

/// Notify submission queue update
pub const NOTIFY_SQ: usize = 0x12;

/// Async notification for packet RX/TX completion
pub const NOTIFY_IO: usize = 0x20;

/// Network-specific ring opcodes for io_uring
pub mod opcodes {
    pub const SEND: u8 = 10;
    pub const RECV: u8 = 11;
}

use glenda::io::uring::IoUringSqe;

pub fn sqe_send(addr: u64, len: u32, user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: opcodes::SEND, addr, len, user_data, ..Default::default() }
}

pub fn sqe_recv(addr: u64, len: u32, user_data: u64) -> IoUringSqe {
    IoUringSqe { opcode: opcodes::RECV, addr, len, user_data, ..Default::default() }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MacAddress {
    pub octets: [u8; 6],
}
