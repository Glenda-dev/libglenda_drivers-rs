//! PCI/PCIe Protocol (0x30E)

pub const READ_CONFIG: usize = 0x01; // arg0: offset, arg1: size
pub const WRITE_CONFIG: usize = 0x02; // arg0: offset, arg1: val, arg2: size
pub const ENABLE_BUS_MASTER: usize = 0x03;
pub const ENABLE_MSI: usize = 0x04; // arg0: vector, arg1: dest_id

// Standard Config Space Offsets
pub const PCI_VENDOR_ID: usize = 0x00;
pub const PCI_DEVICE_ID: usize = 0x02;
pub const PCI_COMMAND: usize = 0x04;
pub const PCI_STATUS: usize = 0x06;
pub const PCI_CLASS_REV: usize = 0x08;
pub const PCI_HEADER_TYPE: usize = 0x0E;
pub const PCI_BAR0: usize = 0x10;
pub const PCI_CAPABILITY_LIST: usize = 0x34;

// Command Register Bits
pub const PCI_CMD_IO: u16 = 0x1;
pub const PCI_CMD_MEM: u16 = 0x2;
pub const PCI_CMD_BUS_MASTER: u16 = 0x4;
pub const PCI_CMD_INTX_DISABLE: u16 = 0x400;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PciAddress {
    pub segment: u16,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}
