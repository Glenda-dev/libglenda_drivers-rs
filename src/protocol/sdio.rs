//! SDIO Device Protocol

pub const SEND_COMMAND: usize = 0x01; // arg0: SdioCommand, ret: [u32; 4]
pub const READ_BLOCKS: usize = 0x02; // arg0: SdioCommand, buf: [u8]
pub const WRITE_BLOCKS: usize = 0x03; // arg0: SdioCommand, buf: [u8]
pub const SET_BUS_WIDTH: usize = 0x04; // arg0: width
pub const SET_CLOCK: usize = 0x05; // arg0: clock_hz

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SdioCommand {
    pub cmd: u8,
    pub arg: u32,
    pub response_type: u8,
}

pub const RESP_NONE: u8 = 0;
pub const RESP_R1: u8 = 1;
pub const RESP_R1B: u8 = 2;
pub const RESP_R2: u8 = 3;
pub const RESP_R3: u8 = 4;
pub const RESP_R6: u8 = 5;
pub const RESP_R7: u8 = 6;

pub const BUS_WIDTH_1: u8 = 0;
pub const BUS_WIDTH_4: u8 = 1;
pub const BUS_WIDTH_8: u8 = 2;
