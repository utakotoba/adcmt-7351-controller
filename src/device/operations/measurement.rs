use anyhow::{Result, anyhow};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::Device;

/// Function code mapping enum
#[derive(Debug, FromPrimitive)]
pub enum FunctionCode {
    /// DC voltage measurement (DCV) mode
    DCV = 1,

    /// AC voltage measurement (ACV) mode
    ACV = 2,

    /// Resistance measurement (2WΩ) mode
    Resistance = 3,

    /// DC current measurement (DCI) mode
    DCI = 5,

    /// AC current measurement (ACI) mode
    ACI = 6,

    /// AC voltage (AC+DC coupling) measurement (ACV(AC+DC)) mode
    ACVCoupling = 7,

    /// AC current (AC+DC coupling) measurement (ACI(AC+DC)) mode
    ACICoupling = 8,

    /// Diode measurement (Diode) mode
    Diode = 13,

    /// Low power resistance measurement (2WΩ(Low)) mode
    ResistanceLowPower = 20,

    /// Continuity test (Cont) mode
    Continuity = 22,

    /// Frequency measurement (Freq) mode
    Frequency = 50,
}

/// Raw range mapping enum
///
/// Used to set the range of the measurement based on the code in manual.
#[derive(Debug, FromPrimitive)]
pub enum RawRange {
    AUTO = 0,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
}

/// DC voltage range mapping enum
#[derive(Debug, FromPrimitive)]
pub enum VoltageDCRange {
    /// Auto range
    AUTO = 0,

    /// 200mV range
    V200m = 3,

    /// 2V range
    V2 = 4,

    /// 20V range
    V20 = 5,

    /// 200V range
    V200 = 6,

    /// 1000V range
    V1000 = 7,
}

/// AC voltage range mapping enum
#[derive(Debug, FromPrimitive)]
pub enum VoltageACRange {
    /// Auto range
    AUTO = 0,

    /// 200mV range
    V200m = 3,

    /// 2V range
    V2 = 4,

    /// 20V range
    V20 = 5,

    /// 200V range
    V200 = 6,

    /// 700V range
    V700 = 7,
}

/// Current range mapping enum
#[derive(Debug, FromPrimitive)]
pub enum CurrentRange {
    /// Auto range
    AUTO = 0,

    /// 200mA range
    I200m = 6,

    /// 2000mA range
    I2000 = 7,

    /// 20A range
    I10 = 8,
}

/// Resistance range mapping enum
#[derive(Debug, FromPrimitive)]
pub enum ResistanceRange {
    /// Auto range
    AUTO = 0,

    /// 200Ω range
    R200 = 3,

    /// 2000Ω range
    R2000 = 4,

    /// 20kΩ range
    R20k = 5,

    /// 200kΩ range
    R200k = 6,

    /// 2000kΩ range
    R2000k = 7,

    /// 20MΩ range
    R20M = 8,

    /// 200MΩ range
    R200M = 9,
}

/// Low power resistance range mapping enum
#[derive(Debug, FromPrimitive)]
pub enum ResistanceLowPowerRange {
    /// Auto range
    AUTO = 0,

    /// 200Ω range
    R200 = 3,

    /// 2000Ω range
    R2000 = 4,

    /// 20kΩ range
    R20k = 5,

    /// 200kΩ range
    R200k = 6,

    /// 2000kΩ range
    R2000k = 7,

    /// 20MΩ range
    R20M = 8,
}

/// Frequency range mapping enum
#[derive(Debug, FromPrimitive)]
pub enum FrequencyVoltageRange {
    /// 200mV range
    V200m = 3,

    /// 2000mV range
    V2000m = 4,

    /// 20V range
    V20 = 5,

    /// 200V range
    V200 = 6,

    /// 700V range
    V700 = 7,
}

/// Mode and range shorthand mapping enum
#[derive(Debug)]
pub enum ShortHand {
    /// DC voltage measurement (DCV) mode
    DCV(VoltageDCRange),

    /// AC voltage measurement (ACV) mode
    ACV(VoltageACRange),

    /// Resistance measurement (2WΩ) mode
    Resistance(ResistanceRange),

    /// DC current measurement (DCI) mode
    DCI(CurrentRange),

    /// AC current measurement (ACI) mode
    ACI(CurrentRange),

    /// AC voltage (AC+DC coupling) measurement (ACV(AC+DC)) mode
    ACVCoupling(VoltageACRange),

    /// AC current (AC+DC coupling) measurement (ACI(AC+DC)) mode
    ACICoupling(CurrentRange),

    /// Diode measurement (Diode) mode
    Diode,

    /// Low power resistance measurement (2WΩ(Low)) mode
    ResistanceLowPower(ResistanceLowPowerRange),

    /// Continuity test (Cont) mode
    Continuity,

    /// Frequency measurement (Freq) mode
    Frequency(FrequencyVoltageRange),
}

/// Sampling rate mapping enum
#[derive(Debug, FromPrimitive)]
pub enum SamplingRate {
    /// Fast sampling rate
    FAST = 1,

    /// Medium sampling rate
    MEDIUM = 2,

    /// Slow 1 sampling rate
    SLOW1 = 3,

    /// Slow 2 sampling rate
    SLOW2 = 4,
}

/// Number of display digits mapping enum
#[derive(Debug, FromPrimitive)]
pub enum NumberOfDisplayDigits {
    /// 3 1/2 digits
    ThreeAndAHalf = 3,

    /// 4 1/2 digits
    FourAndAHalf = 4,

    /// 5 1/2 digits
    FiveAndAHalf = 5,
}

/// Auto zero mapping enum
#[derive(Debug, FromPrimitive)]
pub enum AutoZero {
    /// Manual auto zero
    Disable = 0,

    /// Auto zero
    Enable = 1,

    /// Auto zero only once
    Once = 2,
}

impl Device {
    /// Function: query current measurement mode
    ///
    /// ADC command: `F?`
    pub fn function(&mut self) -> Result<FunctionCode> {
        self.write("F?")?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("F").unwrap_or(trimmed);
        let function_code: u8 = numeric_part
            .parse()
            .map_err(|e| anyhow!("Failed to parse function code '{}': {}", response, e))?;

        FunctionCode::from_u8(function_code)
            .ok_or_else(|| anyhow!("Failed to convert function code to FunctionCode"))
    }

    /// Function: change to the given function code
    ///
    /// ADC command: `F<function_code>`
    pub fn function_set(&mut self, function_code: FunctionCode) -> Result<()> {
        self.write(&format!("F{}", function_code as u8))
    }

    /// Function: check if the given function code is ready
    ///
    /// For `0` means the function is ready, `1` means the function is not ready.
    ///
    /// ADC command: `INH?<function_code>`
    pub fn function_ready(&mut self, function_code: FunctionCode) -> Result<bool> {
        self.write(&format!("INH?{}", function_code as u8))?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("INH?").unwrap_or(trimmed);
        Ok(numeric_part == "0")
    }

    /// Range: get current range of the measurement
    ///
    /// ADC command: `R?`
    pub fn range(&mut self) -> Result<RawRange> {
        self.write("R?")?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("R").unwrap_or(trimmed);
        let num: u8 = numeric_part
            .parse()
            .map_err(|e| anyhow!("Failed to parse range value '{}': {}", response, e))?;

        RawRange::from_u8(num).ok_or_else(|| anyhow!("Failed to convert range value to RawRange"))
    }

    /// Range: set current range of the measurement
    ///
    /// ADC command: `R<raw_range>`
    pub fn range_set(&mut self, raw_range: RawRange) -> Result<()> {
        self.write(&format!("R{}", raw_range as u8))
    }

    /// Range: fix automatic range by switch to manual range
    ///
    /// ADC command: `RX`
    pub fn range_fix(&mut self) -> Result<()> {
        self.write("RX")
    }

    /// ShortHand: set the mode and range of the measurement
    ///
    /// ADC command: `F<function_code>,R<range_code>`
    pub fn shorthand(&mut self, shorthand: ShortHand) -> Result<()> {
        match shorthand {
            ShortHand::DCV(range) => self.write(&format!("F1,R{}", range as u8)),
            ShortHand::ACV(range) => self.write(&format!("F2,R{}", range as u8)),
            ShortHand::Resistance(range) => self.write(&format!("F3,R{}", range as u8)),
            ShortHand::DCI(range) => self.write(&format!("F5,R{}", range as u8)),
            ShortHand::ACI(range) => self.write(&format!("F6,R{}", range as u8)),
            ShortHand::ACVCoupling(range) => self.write(&format!("F7,R{}", range as u8)),
            ShortHand::ACICoupling(range) => self.write(&format!("F8,R{}", range as u8)),
            ShortHand::Diode => self.write("F13"),
            ShortHand::ResistanceLowPower(range) => self.write(&format!("F20,R{}", range as u8)),
            ShortHand::Continuity => self.write("F22"),
            ShortHand::Frequency(range) => self.write(&format!("F50,R{}", range as u8)),
        }
    }

    /// Sampling Rate: get current sampling rate
    ///
    /// ADC command: `PR?`
    pub fn sampling_rate(&mut self) -> Result<SamplingRate> {
        self.write("PR?")?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("PR").unwrap_or(trimmed);
        let num: u8 = numeric_part
            .parse()
            .map_err(|e| anyhow!("Failed to parse sampling rate value '{}': {}", response, e))?;

        SamplingRate::from_u8(num)
            .ok_or_else(|| anyhow!("Failed to convert sampling rate value to SamplingRate"))
    }

    /// Sampling Rate: set current sampling rate
    ///
    /// ADC command: `PR<sampling_rate>`
    pub fn sampling_rate_set(&mut self, sampling_rate: SamplingRate) -> Result<()> {
        self.write(&format!("PR{}", sampling_rate as u8))
    }

    /// Number of Display Digits: get current number of display digits
    ///
    /// ADC command: `RE?`
    pub fn number_of_display_digits(&mut self) -> Result<NumberOfDisplayDigits> {
        self.write("RE?")?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("RE").unwrap_or(trimmed);
        let num: u8 = numeric_part.parse().map_err(|e| {
            anyhow!(
                "Failed to parse number of display digits value '{}': {}",
                response,
                e
            )
        })?;

        NumberOfDisplayDigits::from_u8(num).ok_or_else(|| {
            anyhow!("Failed to convert number of display digits value to NumberOfDisplayDigits")
        })
    }

    /// Number of Display Digits: set current number of display digits
    ///
    /// ADC command: `RE<number_of_display_digits>`
    pub fn number_of_display_digits_set(
        &mut self,
        number_of_display_digits: NumberOfDisplayDigits,
    ) -> Result<()> {
        self.write(&format!("RE{}", number_of_display_digits as u8))
    }

    /// Auto Zero: get current auto zero setting
    ///
    /// ADC command: `AZ?`
    pub fn auto_zero(&mut self) -> Result<AutoZero> {
        self.write("AZ?")?;
        let response = self.read()?;
        let trimmed = response.trim();
        let numeric_part = trimmed.strip_prefix("AZ").unwrap_or(trimmed);
        let num: u8 = numeric_part
            .parse()
            .map_err(|e| anyhow!("Failed to parse auto zero value '{}': {}", response, e))?;

        AutoZero::from_u8(num)
            .ok_or_else(|| anyhow!("Failed to convert auto zero value to AutoZero"))
    }

    /// Auto Zero: set current auto zero setting
    ///
    /// ADC command: `AZ<auto_zero>`
    pub fn auto_zero_set(&mut self, auto_zero: AutoZero) -> Result<()> {
        self.write(&format!("AZ{}", auto_zero as u8))
    }

    /// Continuity threshold constant: get current continuity threshold constant
    ///
    /// ADC command: `KOM?`
    pub fn continuity_threshold_constant(&mut self) -> Result<String> {
        self.write("KOM?")?;
        self.read()
    }

    /// Continuity threshold constant: set current continuity threshold constant
    ///
    /// ADC command: `KOM<continuity_threshold_constant>`
    pub fn continuity_threshold_constant_set(
        &mut self,
        continuity_threshold_constant: u16,
    ) -> Result<()> {
        self.write(&format!("KOM{}", continuity_threshold_constant))
    }

    // TODO: measurement data memory related commands
}
