//! SPI Protocol (0x30A)

pub const TRANSFER: usize = 0x01; // Full duplex transfer
pub const CONFIG: usize = 0x02; // arg0: speed_hz, arg1: mode

// SPI Modes
pub const MODE_0: u8 = 0; // CPOL=0, CPHA=0
pub const MODE_1: u8 = 1; // CPOL=0, CPHA=1
pub const MODE_2: u8 = 2; // CPOL=1, CPHA=0
pub const MODE_3: u8 = 3; // CPOL=1, CPHA=1
