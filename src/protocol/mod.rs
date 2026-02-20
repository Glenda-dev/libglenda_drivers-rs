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
pub const PCI_PROTO: usize = 0x301;
pub const IOMMU_PROTO: usize = 0x302;
pub const UART_PROTO: usize = 0x303;
pub const TIMER_PROTO: usize = 0x30F;
pub const SDIO_PROTO: usize = 0x310;
pub const PLATFORM_PROTO: usize = 0x311;
pub const ACPI_PROTO: usize = 0x312;
pub const THERMAL_PROTO: usize = 0x313;
pub const BATTERY_PROTO: usize = 0x314;

// Storage & Network
pub const BLOCK_PROTO: usize = 0x304;
pub const NET_PROTO: usize = 0x305;
pub const IB_PROTO: usize = 0x306;
pub const WIFI_PROTO: usize = 0x307;

// Human Interface
pub const INPUT_PROTO: usize = 0x308;
pub const FB_PROTO: usize = 0x309;

// Peripheral Bus
pub const USB_PROTO: usize = 0x30A;
pub const SPI_PROTO: usize = 0x30B;
pub const I2C_PROTO: usize = 0x30C;
pub const GPIO_PROTO: usize = 0x30D;
pub const RNG_PROTO: usize = 0x30E;

// Driver Interface
pub const GET_DESC: usize = 1;
pub const GET_MMIO: usize = 2;
pub const MAP_MMIO: usize = 2;
pub const GET_IRQ: usize = 3;
pub const MAP_IRQ: usize = 3;
pub const SCAN_PLATFORM: usize = 4;
pub const FIND_COMPATIBLE: usize = 5;
pub const ALLOC_DMA: usize = 6;
pub const FREE_DMA: usize = 7;
