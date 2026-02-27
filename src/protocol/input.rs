//! Input Device Protocol (0x305).
//!
//! Perfected using a unified io_uring architecture for zero-copy event delivery.

/// Classic syscall for single-event retrieval (Slow-path fallback)
pub const READ_EVENT: usize = 0x1;

/// Request to map the shared IoUring buffer for zero-copy event delivery.
///
/// MR[0]: Request entries (Power of two).
/// MR[1]: SHM Cap Slot (where to place the SHM frame).
/// Result: Shared Memory Frame (mapped locally in caller VSpace).
pub const SETUP_URING: usize = 0x2;

/// OpCodes for Input URing (Used in SQE opcode)
pub const INPUT_OP_READ: u8 = 0x1;
pub const INPUT_OP_CONFIG: u8 = 0x2;

// Event Types (Matches Linux input.h)
pub const EV_SYN: u16 = 0x00;
pub const EV_KEY: u16 = 0x01;
pub const EV_REL: u16 = 0x02;
pub const EV_ABS: u16 = 0x03;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct InputEvent {
    pub type_: u16,
    pub code: u16,
    pub value: i32,
    pub time_ms: u64,
}

// Common codes
pub const REL_X: u16 = 0x00;
pub const REL_Y: u16 = 0x01;
pub const REL_WHEEL: u16 = 0x08;

// Key Codes (Subset for example)
pub const KEY_ESC: u16 = 1;
pub const KEY_ENTER: u16 = 28;
pub const KEY_SPACE: u16 = 57;
