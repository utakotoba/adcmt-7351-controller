//! Simple example that opens the multimeter device and sends a *RST command

use adcmt_7351_controller::{Device, DeviceManager};
use anyhow::Result;

fn main() -> Result<()> {
    // Create device manager
    let manager = DeviceManager::new()?;

    // Get the first available device
    let device_metadata = manager.first_device()?;
    println!("Found device: {:?}", device_metadata);

    // Open the device
    let mut device = Device::open(&device_metadata)?;
    println!("Device opened successfully");

    // Send *RST command to reset the device
    device.write("*RST")?;
    println!("Sent *RST command");

    println!("Done!");
    Ok(())
}
