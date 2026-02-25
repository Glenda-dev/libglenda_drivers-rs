//! WiFi Device Protocol (0x304)
//! Used for managing 802.11 wireless interfaces (Scanning, Association, Auth).
//! Data transmission should still use NET_PROTO.

pub const SCAN: usize = 0x01; // Trigger a background scan
pub const GET_SCAN_RESULTS: usize = 0x02; // Retrieve results
pub const CONNECT: usize = 0x03; // Associate with an AP
pub const DISCONNECT: usize = 0x04; // Disassociate
pub const GET_STATUS: usize = 0x05; // Get current connection info

// Security Capability Flags
pub const SEC_OPEN: u8 = 0;
pub const SEC_WEP: u8 = 1;
pub const SEC_WPA2: u8 = 2;
pub const SEC_WPA3: u8 = 3;

// Connection Status
pub const STATUS_DISCONNECTED: u8 = 0;
pub const STATUS_CONNECTING: u8 = 1;
pub const STATUS_CONNECTED: u8 = 2;
pub const STATUS_FAILED: u8 = 3;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct WifiApInfo {
    pub ssid: [u8; 32],
    pub ssid_len: u8,
    pub bssid: [u8; 6], // MAC Address of AP
    pub security: u8,
    pub channel: u8,
    pub rssi: i8, // Signal strength in dBm
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WifiConnectReq {
    pub ssid: [u8; 32],
    pub ssid_len: u8,
    pub password: [u8; 64],
    pub password_len: u8,
    pub security: u8,
}
