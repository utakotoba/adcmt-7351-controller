use anyhow::Result;

use crate::Device;

impl Device {
    /// Function: change to DC voltage measurement (DCV) mode
    pub fn mode_dcv(&mut self) -> Result<()> {
        self.write("F1")
    }

    /// Function: change to AC voltage measurement (ACV) mode
    pub fn mode_acv(&mut self) -> Result<()> {
        self.write("F2")
    }

    /// Function: change to Resistance measurement (2WΩ) mode
    pub fn mode_resistance(&mut self) -> Result<()> {
        self.write("F3")
    }

    /// Function: change to DC current measurement (DCI) mode
    pub fn mode_dci(&mut self) -> Result<()> {
        self.write("F5")
    }

    /// Function: change to AC current measurement (ACI) mode
    pub fn mode_aci(&mut self) -> Result<()> {
        self.write("F6")
    }

    /// Function: change to AC voltage (AC+DC coupling) measurement (ACV(AC+DC)) mode
    pub fn mode_acv_coupling(&mut self) -> Result<()> {
        self.write("F7")
    }

    /// Function: change to AC current (AC+DC coupling) measurement (ACI(AC+DC)) mode
    pub fn mode_aci_coupling(&mut self) -> Result<()> {
        self.write("F8")
    }

    /// Function: change to diode measurement (Diode) mode
    pub fn mode_diode(&mut self) -> Result<()> {
        self.write("F13")
    }

    /// Function: change to low power resistance measurement (2WΩ(Low)) mode
    pub fn mode_resistance_low(&mut self) -> Result<()> {
        self.write("F20")
    }

    /// Function: change to continuity test (Cont) mode
    pub fn mode_continuity(&mut self) -> Result<()> {
        self.write("F22")
    }

    /// Function: change to frequency measurement (Freq) mode
    pub fn mode_frequency(&mut self) -> Result<()> {
        self.write("F50")
    }
}
