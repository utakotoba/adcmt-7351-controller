//! Device layer for instrument communication

mod manager;
mod operations;

// Re-exports
pub use manager::DeviceManager;
pub use operations::Device;
