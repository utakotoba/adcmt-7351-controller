//! Device enumeration and management

use anyhow::{Context, Ok, Result};

use crate::transport::{UsbContext, UsbDeviceMetadata};

/// Device manager for enumerating available devices
pub struct DeviceManager {
    ctx: UsbContext,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> Result<Self> {
        let ctx = UsbContext::new().context("Failed to initialize USB context")?;
        Ok(Self { ctx })
    }

    /// List all available ADCMT 7351 devices
    pub fn list_devices(&self) -> Result<Vec<UsbDeviceMetadata>> {
        self.ctx
            .enumerate_devices()
            .context("Failed to enumerate USB devices")
    }

    /// Get the first available device info
    pub fn first_device(&self) -> Result<UsbDeviceMetadata> {
        let devices = self.list_devices()?;
        devices
            .first()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No ADCMT 7351 device found"))
    }
}

impl Default for DeviceManager {
    fn default() -> Self {
        Self::new().expect("Failed to create device manager")
    }
}
