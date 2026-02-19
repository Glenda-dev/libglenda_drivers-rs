//! Framebuffer Protocol (0x306)

pub const GET_INFO: usize = 0x1;
pub const FLUSH: usize = 0x2; // arg0: x, arg1: y, arg2: w, arg3: h