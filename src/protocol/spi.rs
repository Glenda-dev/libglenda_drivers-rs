//! SPI Protocol (0x30A)

pub const TRANSFER: usize = 0x01; // Full duplex transfer
pub const CONFIG: usize = 0x02; // arg0: speed_hz, arg1: mode
