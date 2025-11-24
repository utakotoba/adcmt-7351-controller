//! Instrument operations

use std::time::Duration;

use anyhow::{Context, Ok, Result};

use crate::{
    protocol::{Packet, SequenceCounter},
    transport::{UsbDevice, UsbDeviceMetadata},
};

pub struct Device {
    usb_device: UsbDevice,
    sequence: SequenceCounter,
}

impl Device {
    /// Open a multimeter device using device metadata
    pub fn open(metadata: &UsbDeviceMetadata) -> Result<Self> {
        let usb_device = UsbDevice::open(metadata).context("Failed to open USB device")?;

        Ok(Self {
            usb_device,
            sequence: SequenceCounter::new(),
        })
    }

    /// Set timeout for operation IO
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.usb_device.set_timeout(timeout);
    }

    /// Get timeout of operation IO
    pub fn timeout(&self) -> Duration {
        self.usb_device.timeout()
    }

    /// Write a command to current device
    pub fn write(&mut self, command: &str) -> Result<()> {
        let sequence = self.sequence.next();
        let packet =
            Packet::encode_write(command, sequence).context("Failed to encode write packet")?;

        self.usb_device
            .write(&packet)
            .context("Failed to write command to current device")?;

        Ok(())
    }

    /// Read a response from the device
    pub fn read(&mut self) -> Result<String> {
        // Send read request
        let sequence = self.sequence.next();
        let read_request = Packet::encode_read(sequence);

        self.usb_device
            .write(&read_request)
            .context("Failed to send read request")?;

        // Wait for device to interact
        std::thread::sleep(Duration::from_millis(10));

        // Read response
        let mut buffer = vec![0u8; 128];
        let transferred = self
            .usb_device
            .read(&mut buffer)
            .context("Failed to read from device")?;

        // Decode packet
        let decoded = Packet::decode_read(&buffer[..transferred])
            .context("Failed to decode read response")?;

        // Convert to String
        String::from_utf8(decoded).context("Response contains invalid UTF-8 character")
    }

    /// Clear device input/output buffers
    pub fn clear(&mut self) -> Result<()> {
        self.usb_device
            .clear_halt()
            .context("Failed to clear device buffers")?;

        Ok(())
    }
}
