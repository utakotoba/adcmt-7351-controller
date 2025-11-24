//! USB transport layer for USB device communication

mod usb_context;
mod usb_device;
mod usb_device_metadata;

/// ADCMT 7351A USB Vendor ID
pub const VID: u16 = 0x1334;

/// ADCMT 7351A USB Product ID
pub const PID: u16 = 0x0203;
