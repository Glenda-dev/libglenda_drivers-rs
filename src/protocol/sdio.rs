//! SDIO Device Protocol

pub const SEND_COMMAND: usize = 0x01; // arg0: SdioCommand, ret: [u32; 4]
pub const READ_BLOCKS: usize = 0x02; // arg0: SdioCommand, buf: [u8]
pub const WRITE_BLOCKS: usize = 0x03; // arg0: SdioCommand, buf: [u8]
pub const SET_BUS_WIDTH: usize = 0x04; // arg0: width
pub const SET_CLOCK: usize = 0x05; // arg0: clock_hz
