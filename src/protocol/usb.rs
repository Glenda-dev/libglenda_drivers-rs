//! USB Host Controller Protocol (0x307)

// IPC Operations
pub const RESET_PORT: usize = 0x01; // arg0: port_id
pub const CONTROL_XFER: usize = 0x02; // arg0: address << 16 | endpoint
pub const BULK_XFER: usize = 0x03; // arg0: address << 16 | endpoint
pub const INTR_XFER: usize = 0x04;
