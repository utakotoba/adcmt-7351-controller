//! ADCMT 7351A Digital Multimeter Controller
//!
//! A modern (maybe) Rust library for controlling the ADCMT 7351A/E+03
//! digital multimeter via USB interface.

mod device;
mod protocol;
mod transport;

// Re-exports
pub use device::{Device, DeviceManager};
pub use transport::UsbDeviceMetadata;
