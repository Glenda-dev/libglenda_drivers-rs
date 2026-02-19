//! WiFi Device Protocol (0x304)
//! Used for managing 802.11 wireless interfaces (Scanning, Association, Auth).
//! Data transmission should still use NET_PROTO.

pub const SCAN: usize = 0x01; // Trigger a background scan
pub const GET_SCAN_RESULTS: usize = 0x02; // Retrieve results
pub const CONNECT: usize = 0x03; // Associate with an AP
pub const DISCONNECT: usize = 0x04; // Disassociate
pub const GET_STATUS: usize = 0x05; // Get current connection info
