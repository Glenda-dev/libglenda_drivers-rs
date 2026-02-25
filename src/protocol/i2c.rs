//! I2C Protocol (0x30B)

pub const READ: usize = 0x1; // arg0: addr, arg1: len
pub const WRITE: usize = 0x2; // arg0: addr, arg1: len
pub const WRITE_READ: usize = 0x3; // arg0: addr, arg1: w_len, arg2: r_len

// Flags
pub const I2C_ADDR_10BIT: u16 = 0x8000;
