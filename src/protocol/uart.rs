//! UART Device Protocol

/// Write a single character
pub const PUT_CHAR: usize = 0x01;
/// Read a single character (blocking?)
pub const GET_CHAR: usize = 0x02;
/// Write a string
pub const PUT_STR: usize = 0x03;
/// Configuration
pub const SET_BAUD_RATE: usize = 4;
