//! Framebuffer Protocol (0x306)

pub const GET_INFO: usize = 0x1;
pub const FLUSH: usize = 0x2; // arg0: x, arg1: y, arg2: w, arg3: h

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FbInfo {
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub format: u32,
    pub bpp: u32,
    pub paddr: usize,
    pub size: usize,
}

pub const MODE_INPUT: u8 = 0;
pub const MODE_OUTPUT: u8 = 1;
pub const MODE_ALT: u8 = 2;
