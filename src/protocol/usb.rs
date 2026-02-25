//! USB Host Controller Protocol (0x307)

// IPC Operations
pub const RESET_PORT: usize = 0x01; // arg0: port_id
pub const CONTROL_XFER: usize = 0x02; // arg0: address << 16 | endpoint
pub const BULK_XFER: usize = 0x03; // arg0: address << 16 | endpoint
pub const INTR_XFER: usize = 0x04;

// USB Constants

// Request Types (bmRequestType)
pub const REQ_DIR_OUT: u8 = 0x00;
pub const REQ_DIR_IN: u8 = 0x80;
pub const REQ_TYPE_STANDARD: u8 = 0x00;
pub const REQ_TYPE_CLASS: u8 = 0x20;
pub const REQ_TYPE_VENDOR: u8 = 0x40;
pub const REQ_RECIP_DEVICE: u8 = 0x00;
pub const REQ_RECIP_INTERFACE: u8 = 0x01;
pub const REQ_RECIP_ENDPOINT: u8 = 0x02;

// Standard Requests (bRequest)
pub const REQ_GET_STATUS: u8 = 0x00;
pub const REQ_CLEAR_FEATURE: u8 = 0x01;
pub const REQ_SET_ADDRESS: u8 = 0x05;
pub const REQ_GET_DESCRIPTOR: u8 = 0x06;
pub const REQ_SET_DESCRIPTOR: u8 = 0x07;
pub const REQ_GET_CONFIGURATION: u8 = 0x08;
pub const REQ_SET_CONFIGURATION: u8 = 0x09;

// Descriptor Types
pub const DESC_DEVICE: u8 = 0x01;
pub const DESC_CONFIGURATION: u8 = 0x02;
pub const DESC_STRING: u8 = 0x03;
pub const DESC_INTERFACE: u8 = 0x04;
pub const DESC_ENDPOINT: u8 = 0x05;

// Endpoint Attributes
pub const EP_ATTR_CONTROL: u8 = 0x00;
pub const EP_ATTR_ISOCH: u8 = 0x01;
pub const EP_ATTR_BULK: u8 = 0x02;
pub const EP_ATTR_INTR: u8 = 0x03;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UsbSetupPacket {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UsbDeviceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub bcd_usb: u16,
    pub device_class: u8,
    pub device_subclass: u8,
    pub device_protocol: u8,
    pub max_packet_size0: u8,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: u16,
    pub i_manufacturer: u8,
    pub i_product: u8,
    pub i_serial_number: u8,
    pub num_configurations: u8,
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct UsbEndpointDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub endpoint_address: u8,
    pub attributes: u8,
    pub max_packet_size: u16,
    pub interval: u8,
}
