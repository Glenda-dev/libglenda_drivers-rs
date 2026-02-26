pub mod acpi;
pub mod battery;
pub mod block;
pub mod fb;
pub mod gpio;
pub mod i2c;
pub mod input;
pub mod kernel;
pub mod net;
pub mod pci;
pub mod platform;
pub mod rng;
pub mod sdio;
pub mod spi;
pub mod thermal;
pub mod timer;
pub mod uart;
pub mod usb;
pub mod wifi;

pub const KERNEL_PROTO: usize = 0x100;

// Core System
pub const PCI_PROTO: usize = 0x401;
pub const IOMMU_PROTO: usize = 0x402;
pub const UART_PROTO: usize = 0x403;
pub const TIMER_PROTO: usize = 0x40F;
pub const SDIO_PROTO: usize = 0x410;
pub const PLATFORM_PROTO: usize = 0x411;
pub const ACPI_PROTO: usize = 0x412;
pub const THERMAL_PROTO: usize = 0x413;
pub const BATTERY_PROTO: usize = 0x414;

// Communication
pub const SMBUS_PROTO: usize = 0x415;
pub const CAN_PROTO: usize = 0x416;

// Storage & Network
pub const BLOCK_PROTO: usize = 0x404;
pub const NET_PROTO: usize = 0x405;
pub const IB_PROTO: usize = 0x406;
pub const WIFI_PROTO: usize = 0x407;

// Human Interface
pub const INPUT_PROTO: usize = 0x408;
pub const FB_PROTO: usize = 0x409;

// Peripheral Bus
pub const USB_PROTO: usize = 0x40A;
pub const SPI_PROTO: usize = 0x40B;
pub const I2C_PROTO: usize = 0x40C;
pub const GPIO_PROTO: usize = 0x40D;
pub const RNG_PROTO: usize = 0x40E;
