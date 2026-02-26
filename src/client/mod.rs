// use glenda::cap::{CapPtr, Endpoint, Frame};

pub mod acpi;
pub mod battery;
pub mod block;
pub mod fb;
pub mod input;
pub mod net;
pub mod pci;
pub mod platform;
pub mod thermal;
pub mod timer;
pub mod uart;

pub use glenda::io::uring::RingParams;
pub use glenda::mem::shm::ShmParams;
