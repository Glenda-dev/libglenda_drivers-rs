use crate::protocol::fb::FbInfo;
use crate::protocol::input::InputEvent;
use crate::protocol::net::MacAddress;
use crate::protocol::pci::PciAddress;
use crate::protocol::sdio::SdioCommand;
use crate::protocol::thermal::ThermalZones;
use crate::protocol::usb::UsbSetupPacket;
use crate::protocol::wifi::WifiApInfo;
use alloc::string::String;
use alloc::vec::Vec;
use glenda::error::Error;
use glenda::ipc::Badge;
use glenda::protocol::device::DeviceDescNode;
pub trait DriverService {
    fn init(&mut self) -> Result<(), Error>;
    fn enable(&mut self);
    fn disable(&mut self);
}

pub trait DriverClient {
    fn connect(&mut self) -> Result<(), Error>;
    fn disconnect(&mut self) -> Result<(), Error>;
}

pub trait BlockDriver {
    fn read_blocks(&self, sector: u64, count: u32, buf: &mut [u8]) -> Result<(), Error>;
    fn write_blocks(&self, sector: u64, count: u32, buf: &[u8]) -> Result<(), Error>;
    fn block_size(&self) -> u32;
    fn capacity(&self) -> u64;
}

/// PciDriver provides PCI config space access.
pub trait PciDriver {
    fn read_config(&self, offset: usize, size: usize) -> Result<u32, Error>;
    fn write_config(&self, offset: usize, value: u32, size: usize) -> Result<(), Error>;
    fn enable_bus_master(&self) -> Result<(), Error>;
    fn enable_msi(&self, vector: u8, dest_id: u32) -> Result<(), Error>;
    fn get_address(&self) -> PciAddress;
}

/// NetDriver provides metadata and asynchronous IO ring setup for network packet transmission.
pub trait NetDriver {
    fn mac_address(&self) -> MacAddress;
}

/// UartDriver provides serial communication.
pub trait UartDriver {
    fn put_char(&mut self, c: u8);
    fn get_char(&mut self) -> Option<u8>;
    fn put_str(&mut self, s: &str);
}

/// WifiDriver provides wireless network management.
pub trait WifiDriver {
    fn scan(&mut self) -> Result<(), Error>;
    fn get_scan_results(&self, buf: &mut [WifiApInfo]) -> Result<usize, Error>;
    fn connect(&mut self, ssid: &str, password: &str, security: u8) -> Result<(), Error>;
    fn disconnect(&mut self) -> Result<(), Error>;
    fn status(&self) -> Result<u8, Error>;
}

/// InputDriver provides HID events (keyboard, mouse).
pub trait InputDriver {
    fn poll_event(&mut self) -> Option<InputEvent>;
}

/// FrameBufferDriver provides display management.
pub trait FrameBufferDriver {
    fn get_info(&self) -> FbInfo;
    fn flush(&mut self, x: u32, y: u32, w: u32, h: u32) -> Result<(), Error>;
}

/// UsbHostDriver provides USB bus management.
pub trait UsbHostDriver {
    /// Send a control packet (Setup stage + optional Data stage)
    fn control_transfer(
        &mut self,
        addr: u8,
        ep: u8,
        setup: UsbSetupPacket,
        data: &mut [u8],
    ) -> Result<usize, Error>;

    /// Perform a bulk transfer
    fn bulk_transfer(&mut self, addr: u8, ep: u8, data: &mut [u8]) -> Result<usize, Error>;

    /// Reset a root hub port
    fn reset_port(&mut self, port: u8) -> Result<(), Error>;
}

/// GpioDriver provides pin control.
pub trait GpioDriver {
    fn set_mode(&mut self, pin: u32, mode: u8) -> Result<(), Error>;
    fn write(&mut self, pin: u32, value: bool) -> Result<(), Error>;
    fn read(&self, pin: u32) -> Result<bool, Error>;
}

/// TimerDriver provides time and alarm services.
pub trait TimerDriver {
    /// Get current UNIX time in seconds.
    fn get_time(&self) -> u64;
    /// Set current UNIX time in seconds.
    fn set_time(&mut self, timestamp: u64) -> Result<(), Error>;
    /// Set an alarm (UNIX timestamp).
    fn set_alarm(&mut self, timestamp: u64) -> Result<(), Error>;
    fn stop_alarm(&mut self) -> Result<(), Error>;
}

/// RngDriver provides random numbers.
pub trait RngDriver {
    fn get_random_bytes(&mut self, buf: &mut [u8]) -> Result<usize, Error>;
}

/// SpiDriver provides SPI bus access.
pub trait SpiDriver {
    /// Full-duplex transfer. Data in place modification.
    fn transfer(&mut self, buf: &mut [u8]) -> Result<(), Error>;
    fn send(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn recv(&mut self, buf: &mut [u8]) -> Result<(), Error>;
}

/// I2cDriver provides I2C bus access.
pub trait I2cDriver {
    fn read(&mut self, addr: u16, buf: &mut [u8]) -> Result<(), Error>;
    fn write(&mut self, addr: u16, buf: &[u8]) -> Result<(), Error>;
    /// Write command then read response (Atomic repeated start if supported)
    fn write_read(&mut self, addr: u16, w_buf: &[u8], r_buf: &mut [u8]) -> Result<(), Error>;
}

/// SdioDriver provides SDIO bus access.
pub trait SdioDriver {
    fn send_command(&mut self, cmd: SdioCommand) -> Result<[u32; 4], Error>;
    fn read_blocks(&mut self, cmd: SdioCommand, buf: &mut [u8]) -> Result<(), Error>;
    fn write_blocks(&mut self, cmd: SdioCommand, buf: &[u8]) -> Result<(), Error>;
    fn set_bus_width(&mut self, width: u8) -> Result<(), Error>;
    fn set_clock(&mut self, hz: u32) -> Result<(), Error>;
}

/// IommuDriver provides DMA remapping.
pub trait IommuDriver {
    fn map(&mut self, iova: usize, paddr: usize, size: usize, flags: u32) -> Result<(), Error>;
    fn unmap(&mut self, iova: usize, size: usize) -> Result<(), Error>;
    fn flush(&mut self) -> Result<(), Error>;
}

/// PlatformDriver provides system-wide power management and control.
pub trait PlatformDriver {
    /// Set system sleep state (S1-S4).
    fn set_sleep_state(&mut self, state: u32) -> Result<(), Error>;

    /// Reset the system.
    fn reset(&mut self, warm: bool) -> Result<(), Error>;

    /// Shut down the system.
    fn shutdown(&mut self) -> Result<(), Error>;
}

/// ThermalDriver provides temperature monitoring.
pub trait ThermalDriver {
    /// Get thermal status in Kelvin/10.
    fn get_temperature(&self, zone: u32) -> Result<u32, Error>;
}

/// BatteryDriver provides power source and battery information.
pub trait BatteryDriver {
    /// Get power source status.
    /// Returns: 1 (AC), 0 (Battery)
    fn get_power_source(&self) -> Result<u32, Error>;

    /// Get battery level (0-100).
    fn get_level(&self) -> Result<u32, Error>;

    /// Get battery status (charging, etc.)
    fn get_status(&self) -> Result<u32, Error>;
}

/// AcpiDriver provides ACPI-specific services like method evaluation.
pub trait AcpiDriver {
    /// Evaluate an ACPI method (e.g. \_OSC, \_OSI).
    fn evaluate_method(&mut self, path: &str, args: &[u64]) -> Result<Vec<u64>, Error>;
}

pub trait BusDriver {
    fn probe(&mut self) -> Result<Vec<DeviceDescNode>, Error>;
}

pub trait ProbeDriver {
    fn probe(&mut self) -> Result<Vec<String>, Error>;
}

/// ThermalService provides system-wide thermal monitoring.
pub trait ThermalService {
    /// Get all thermal zones in the system.
    fn get_thermal_zones(&mut self) -> Result<ThermalZones, Error>;

    /// Report thermal zone information for a specific sensor.
    fn update_thermal_zones(&mut self, badge: Badge, zones: ThermalZones) -> Result<(), Error>;
}
