use anyhow::{Result, anyhow};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::Device;

/// Trigger source mapping enum
#[derive(Debug, FromPrimitive)]
pub enum TriggerSource {
    /// Immediate trigger
    IMMEDIATE = 0,

    /// Manual trigger
    MANUAL = 1,

    /// External trigger
    EXTERNAL = 2,

    /// Bus trigger
    BUS = 3,
}

impl Device {
    /// Start: leave the IDLE state
    ///
    /// ADC command: `INI`
    pub fn start(&mut self) -> Result<()> {
        self.write("INI")
    }

    /// Abort: enter the IDLE state
    ///
    /// ADC command: `ABO`
    pub fn abort(&mut self) -> Result<()> {
        self.write("ABO")
    }

    /// Continuously measure: check if continuous measurement is enabled
    ///
    /// ADC command: `INIC?`
    pub fn continuously_measure(&mut self) -> Result<bool> {
        self.write("INIC?")?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("INIC").unwrap_or(trimmed);
        let num: u8 = numeric_part.parse().map_err(|e| {
            anyhow!(
                "Failed to parse continuously measure value '{}': {}",
                response,
                e
            )
        })?;

        Ok(num == 1)
    }

    /// Continuously measure: enable continuous measurement
    ///
    /// ADC command: `INIC1`
    pub fn continuously_measure_enable(&mut self) -> Result<()> {
        self.write("INIC1")
    }

    /// Continuously measure: disable continuous measurement
    ///
    /// ADC command: `INIC0`
    pub fn continuously_measure_disable(&mut self) -> Result<()> {
        self.write("INIC0")
    }

    /// Trigger source: get current trigger source
    ///
    /// ADC command: `TRS?`
    pub fn trigger_source(&mut self) -> Result<TriggerSource> {
        self.write("TRS?")?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("TRS").unwrap_or(trimmed);
        let num: u8 = numeric_part
            .parse()
            .map_err(|e| anyhow!("Failed to parse trigger source value '{}': {}", response, e))?;

        TriggerSource::from_u8(num)
            .ok_or_else(|| anyhow!("Failed to convert trigger source value to TriggerSource"))
    }

    /// Trigger source: set current trigger source
    ///
    /// ADC command: `TRS<trigger_source>`
    pub fn trigger_source_set(&mut self, trigger_source: TriggerSource) -> Result<()> {
        self.write(&format!("TRS{}", trigger_source as u8))
    }

    /// Trigger delay: get current trigger delay
    ///
    /// ADC command: `TRD?`
    pub fn trigger_delay(&mut self) -> Result<String> {
        self.write("TRD?")?;
        self.read()
    }

    /// Trigger delay: set current trigger delay
    ///
    /// ADC command: `TRD<trigger_delay>`
    pub fn trigger_delay_set(&mut self, trigger_delay: u16) -> Result<()> {
        self.write(&format!("TRD{}", trigger_delay))
    }

    /// Sampling count: get current sampling count
    ///
    /// ADC command: `SPN?`
    pub fn sampling_count(&mut self) -> Result<u16> {
        self.write("SPN?")?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("SPN").unwrap_or(trimmed);
        let num: u16 = numeric_part
            .parse()
            .map_err(|e| anyhow!("Failed to parse sampling count value '{}': {}", response, e))?;
        Ok(num)
    }

    /// Sampling count: set current sampling count
    ///
    /// ADC command: `SPN<sampling_count>`
    pub fn sampling_count_set(&mut self, sampling_count: u16) -> Result<()> {
        self.write(&format!("SPN{}", sampling_count))
    }
}
