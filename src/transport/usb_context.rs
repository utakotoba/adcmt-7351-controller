//! Low-level control of USB host context

use anyhow::{Context, Ok, Result};
use rusb::{Context as RUsbContext, UsbContext as RUsbContextTrait};

use crate::transport::{PID, VID, usb_device_metadata::UsbDeviceMetadata};

/// USB context wrapper around rusb
pub struct UsbContext {
    ctx: RUsbContext,
}

impl UsbContext {
    /// Create a new USB host context
    pub fn new() -> Result<Self> {
        let ctx = RUsbContext::new().context("Failed to create RUSB context")?;
        Ok(Self { ctx })
    }

    /// Get the internal RUSB context
    #[allow(unused)]
    pub fn get_rusb_ctx(&self) -> &RUsbContext {
        &self.ctx
    }

    /// Enumerate devices by VID and PID
    pub fn enumerate_devices(&self) -> Result<Vec<UsbDeviceMetadata>> {
        let devices =
            RUsbContextTrait::devices(&self.ctx).context("Failed to get USB device list")?;

        let mut found = Vec::new();
        for device in devices.iter() {
            let device_descriptor = device
                .device_descriptor()
                .context("Failed to get device descriptor")?;

            if device_descriptor.vendor_id() == VID && device_descriptor.product_id() == PID {
                let metadata = UsbDeviceMetadata::from_device(&device, &device_descriptor)?;
                found.push(metadata);
            }
        }

        Ok(found)
    }
}
