//! Timer Device Protocol

/// Get current UNIX time in seconds. Returns: arg0: seconds
pub const GET_TIME: usize = 0x01;
/// Set current UNIX time in seconds. arg0: seconds
pub const SET_TIME: usize = 0x02;
/// Set an alarm (UNIX timestamp). arg0: timestamp
pub const SET_ALARM: usize = 0x03;
/// Stop the alarm
pub const STOP_ALARM: usize = 0x04;
