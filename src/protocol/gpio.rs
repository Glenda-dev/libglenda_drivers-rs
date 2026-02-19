//! GPIO Protocol (0x308)

pub const SET_MODE: usize = 0x1; // arg0: pin, arg1: mode
pub const WRITE: usize = 0x2; // arg0: pin, arg1: value
pub const READ: usize = 0x3; // arg0: pin, ret: value
