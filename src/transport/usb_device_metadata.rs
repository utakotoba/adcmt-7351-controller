//! Metadata represents a USB device

use anyhow::{Context, Ok, Result};
use rusb::{Context as RUsbContext, Device, DeviceDescriptor};

#[derive(Debug, Clone)]
pub struct UsbDeviceMetadata {
    pub device: Device<RUsbContext>,
    pub vendor_id: u16,
    pub product_id: u16,
    pub serial_number: Option<String>,
}

impl UsbDeviceMetadata {
    /// Populate metadata of a USB device
    pub fn from_device(
        device: &Device<RUsbContext>,
        descriptor: &DeviceDescriptor,
    ) -> Result<Self> {
        let handle = device
            .open()
            .context("Failed to open device for descriptor read")?;

        let serial_number = descriptor
            .serial_number_string_index()
            .and_then(|index| handle.read_string_descriptor_ascii(index).ok());

        Ok(Self {
            device: device.clone(),
            vendor_id: descriptor.vendor_id(),
            product_id: descriptor.product_id(),
            serial_number,
        })
    }
}
