//! Low-level control of USB host context

use anyhow::{Context, Result};
use rusb::Context as RUsbContext;

/// USB context wrapper around rusb
#[allow(unused)]
pub struct UsbContext {
    ctx: RUsbContext,
}

#[allow(dead_code)]
impl UsbContext {
    /// Create a new USB host context
    pub fn new() -> Result<Self> {
        let ctx = RUsbContext::new().context("Failed to create RUSB context")?;
        Ok(Self { ctx })
    }

    /// Get the internal RUSB context
    pub fn get_rusb_ctx(&self) -> &RUsbContext {
        &self.ctx
    }
}
