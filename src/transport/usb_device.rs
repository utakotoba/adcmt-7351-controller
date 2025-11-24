use std::time::Duration;

use anyhow::{Context, Ok, Result, anyhow};
use rusb::{Context as RUsbContext, Device, DeviceHandle, TransferType};

use crate::transport::usb_device_metadata::UsbDeviceMetadata;

/// USB endpoints
#[allow(unused)]
struct UsbEndpoints {
    read_addr: u8,
    read_type: u8,
    write_addr: u8,
    write_type: u8,
}

/// USB device handle with endpoints
#[allow(unused)]
pub struct UsbDevice {
    handle: DeviceHandle<RUsbContext>,
    endpoints: UsbEndpoints,
    timeout: Duration,
}

#[allow(dead_code)]
impl UsbDevice {
    /// Open a USB device based on metadata
    pub fn open(metadata: &UsbDeviceMetadata) -> Result<Self> {
        let handle = metadata
            .device
            .open()
            .context("Failed to open given USB device")?;

        // Claim the device by claiming the primary interface
        handle
            .claim_interface(0)
            .context("Failed to claim primary interface for given USB device")?;

        // Get all endpoints of current device
        let endpoints = Self::get_endpoints(&metadata.device)
            .context("Failed to get USB endpoints for given device")?;

        let mut device = Self {
            handle,
            endpoints,
            timeout: Duration::from_secs(5),
        };

        // Send initialization control transfers
        device
            .send_init_control_transfers()
            .context("Failed to initialize device")?;

        // Delay after initialization for system to sync
        std::thread::sleep(Duration::from_millis(100));

        Ok(device)
    }

    /// Set timeout for all operations
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout
    }

    /// Get timeout used in current operation
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Write raw data to device
    pub fn write(&self, data: &[u8]) -> Result<usize> {
        // Transfer type ensure in endpoint getting stage - Interrupt or Bulk
        let transferred = if self.endpoints.write_type == TransferType::Interrupt as u8 {
            self.handle
                .write_interrupt(self.endpoints.write_addr, data, self.timeout)
        } else {
            self.handle
                .write_bulk(self.endpoints.write_addr, data, self.timeout)
        }
        .context("Failed to write data to device")?;

        // Wait some time for the multimeter to process
        std::thread::sleep(Duration::from_millis(20));

        Ok(transferred)
    }

    /// Read raw data from device
    pub fn read(&self, buffer: &mut [u8]) -> Result<usize> {
        // Transfer type ensure in endpoint getting stage - Interrupt or Bulk
        let transferred = if self.endpoints.read_type == rusb::TransferType::Interrupt as u8 {
            self.handle
                .read_interrupt(self.endpoints.read_addr, buffer, self.timeout)
        } else {
            self.handle
                .read_bulk(self.endpoints.read_addr, buffer, self.timeout)
        }
        .context("Failed to read data from device")?;

        Ok(transferred)
    }

    /// Clear halt on both endpoints
    pub fn clear_halt(&self) -> Result<()> {
        self.handle
            .clear_halt(self.endpoints.read_addr)
            .context("Failed to clear read endpoint halt")?;
        self.handle
            .clear_halt(self.endpoints.write_addr)
            .context("Failed to clear write endpoint halt")?;
        Ok(())
    }

    /// Read status byte via control transfer
    pub fn read_status(&self) -> Result<u8> {
        let mut status = [0u8; 1];
        let transferred = self
            .handle
            .read_control(
                rusb::constants::LIBUSB_REQUEST_TYPE_VENDOR | rusb::constants::LIBUSB_ENDPOINT_IN,
                0x00,
                0x00,
                0x00,
                &mut status,
                self.timeout,
            )
            .context("Failed to read status byte")?;

        if transferred != 1 {
            anyhow::bail!("Expected 1 byte status, got {}", transferred);
        }

        Ok(status[0])
    }

    /// Internal method: Get all endpoints
    fn get_endpoints(device: &Device<RUsbContext>) -> Result<UsbEndpoints> {
        // ADCMT 7351A/E+03 only have one config descriptor
        let config_descriptor = device
            .config_descriptor(0)
            .context("Failed to get config descriptor")?;

        let interface = config_descriptor
            .interfaces()
            .next()
            .ok_or_else(|| anyhow!("No interfaces found"))?;

        let interface_descriptor = interface
            .descriptors()
            .next()
            .ok_or_else(|| anyhow!("No interface descriptors found"))?;

        let (mut read_endpoint, mut write_endpoint) = (None, None);

        for endpoint_descriptor in interface_descriptor.endpoint_descriptors() {
            let address = endpoint_descriptor.address();
            let direction = address & rusb::constants::LIBUSB_ENDPOINT_DIR_MASK;
            let endpoint_type = endpoint_descriptor.transfer_type();

            // Only take care of Bulk and Interrupt type
            if !matches!(endpoint_type, TransferType::Bulk | TransferType::Interrupt) {
                continue;
            }

            if direction == rusb::constants::LIBUSB_ENDPOINT_IN {
                if read_endpoint.is_none() {
                    read_endpoint = Some((address, endpoint_type));
                }
            } else {
                if write_endpoint.is_none() {
                    write_endpoint = Some((address, endpoint_type));
                }
            }

            if read_endpoint.is_some() && write_endpoint.is_some() {
                break;
            }
        }

        let (read_addr, raw_read_type) =
            read_endpoint.ok_or_else(|| anyhow!("No suitable READ endpoint found"))?;

        let (write_addr, raw_write_type) =
            write_endpoint.ok_or_else(|| anyhow!("No suitable WRITE endpoint found"))?;

        let read_type: u8 = (raw_read_type as u8)
            .try_into()
            .context("Read TransferType failed to cast to u8")?;

        let write_type: u8 = (raw_write_type as u8)
            .try_into()
            .context("Write TransferType failed to cast to u8")?;

        Ok(UsbEndpoints {
            read_addr,
            read_type,
            write_addr,
            write_type,
        })
    }

    /// Init device by sending control transfers
    fn send_init_control_transfers(&mut self) -> Result<()> {
        // Vendor request (0xC1, 0xF5)
        let mut init_data = [0u8, 1];
        self.handle
            .read_control(
                rusb::constants::LIBUSB_REQUEST_TYPE_VENDOR | rusb::constants::LIBUSB_ENDPOINT_IN,
                0xF5,
                0x0000,
                0x0000,
                &mut init_data,
                self.timeout,
            )
            .context("Failed to send vendor control transfer")?;

        // Class request (0aA1, 0xA0)
        let mut init_data = [0u8; 1];
        self.handle
            .read_control(
                rusb::constants::LIBUSB_REQUEST_TYPE_CLASS | rusb::constants::LIBUSB_ENDPOINT_IN,
                0xA0,
                0x0001,
                0x0000,
                &mut init_data,
                self.timeout,
            )
            .context("Failed to send class control transfer")?;

        Ok(())
    }
}

impl Drop for UsbDevice {
    fn drop(&mut self) {
        // Release held resource
        let _ = self.handle.release_interface(0);
    }
}
