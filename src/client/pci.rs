use crate::interface::PciDriver;
use crate::protocol::pci::PciAddress;
use crate::protocol::{PCI_PROTO, pci};
use glenda::cap::Endpoint;
use glenda::error::Error;
use glenda::ipc::{MsgFlags, MsgTag, UTCB};
use glenda::set_mrs;

pub struct PciClient {
    endpoint: Endpoint,
    address: PciAddress,
}

impl PciClient {
    pub const fn new(endpoint: Endpoint, address: PciAddress) -> Self {
        Self { endpoint, address }
    }
}

impl PciDriver for PciClient {
    fn read_config(&self, offset: usize, size: usize) -> Result<u32, Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(PCI_PROTO, pci::READ_CONFIG, MsgFlags::NONE);
        set_mrs!(utcb, offset, size);
        utcb.set_msg_tag(tag);
        self.endpoint.call(&mut utcb)?;

        Ok(utcb.get_mr(0) as u32)
    }

    fn write_config(&self, offset: usize, value: u32, size: usize) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(PCI_PROTO, pci::WRITE_CONFIG, MsgFlags::NONE);
        set_mrs!(utcb, offset, value as usize, size);
        utcb.set_msg_tag(tag);
        self.endpoint.call(&mut utcb)
    }

    fn enable_bus_master(&self) -> Result<(), Error> {
        let tag = MsgTag::new(PCI_PROTO, pci::ENABLE_BUS_MASTER, MsgFlags::NONE);
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        utcb.set_msg_tag(tag);
        self.endpoint.call(&mut utcb)
    }

    fn enable_msi(&self, vector: u8, dest_id: u32) -> Result<(), Error> {
        let mut utcb = unsafe { UTCB::new() };
        utcb.clear();
        let tag = MsgTag::new(PCI_PROTO, pci::ENABLE_MSI, MsgFlags::NONE);
        set_mrs!(utcb, vector as usize, dest_id as usize);
        utcb.set_msg_tag(tag);
        self.endpoint.call(&mut utcb)
    }

    fn get_address(&self) -> PciAddress {
        self.address
    }
}
