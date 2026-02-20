//! Battery and Power status Protocol

/// Get power status (AC/Battery). Returns: arg0: 1 (AC), 0 (Battery)
pub const GET_POWER_SOURCE: usize = 0x01;

/// Get battery level (0-100). Returns: arg0: percentage
pub const GET_LEVEL: usize = 0x02;

/// Get battery status (charging/discharging). Returns: arg0: status code
pub const GET_STATUS: usize = 0x03;

/// Get battery temperature. Returns: arg0: temp in K/10
pub const GET_TEMPERATURE: usize = 0x04;
