/// Models the GSP 10A SmartAmp electroplating power supply.
///
/// The real machine has 4 dials: Coarse Voltage, Fine Voltage, Coarse Current, Fine Current.
/// Voltage and current are linked by Ohm's law (V = I × R) through the load (plating tank).
///
/// Two modes of operation:
/// - **Voltage limiting**: When the voltage setting is lower than I×R, the machine
///   delivers the set voltage and the current is V/R (lower than the current dial setting).
/// - **Current limiting**: When the current setting is lower than V/R, the machine
///   delivers the set current and the voltage drops to I×R.
///
/// In practice for **tank plating**, operators max out voltage and control current.
/// For **brush plating**, operators max out current and control voltage.
///
/// The simulated load resistance represents the electrochemical cell (solution + anode + cathode).
/// Typical resistance for a small tank: ~1-5 ohms depending on solution, distance, surface area.
#[derive(Clone, Debug)]
pub struct MachineState {
    /// Coarse voltage dial position: 0.0 to 1.0
    pub coarse_voltage: f64,
    /// Fine voltage dial position: 0.0 to 1.0
    pub fine_voltage: f64,
    /// Coarse current dial position: 0.0 to 1.0
    pub coarse_current: f64,
    /// Fine current dial position: 0.0 to 1.0
    pub fine_current: f64,
    /// Simulated load resistance in ohms.
    /// This represents the electrochemical cell resistance.
    pub load_resistance: f64,
}

impl MachineState {
    pub fn new() -> Self {
        Self {
            coarse_voltage: 0.0,
            fine_voltage: 0.0,
            coarse_current: 0.0,
            fine_current: 0.0,
            // Default ~1.2 ohms: realistic for a small electrocleaner/plating tank
            load_resistance: 1.2,
        }
    }

    pub fn all_down(&mut self) {
        self.coarse_voltage = 0.0;
        self.fine_voltage = 0.0;
        self.coarse_current = 0.0;
        self.fine_current = 0.0;
    }

    /// The voltage the dial is set to (before Ohm's law limiting).
    /// Coarse: 0-10V range, Fine: +/- 0.5V trim.
    pub fn voltage_dial_setting(&self) -> f64 {
        let coarse = self.coarse_voltage * 10.0;
        let fine = (self.fine_voltage - 0.5) * 1.0;
        (coarse + fine).clamp(0.0, 10.0)
    }

    /// The current the dial is set to (before Ohm's law limiting).
    /// Coarse: 0-10A range, Fine: +/- 0.5A trim.
    pub fn current_dial_setting(&self) -> f64 {
        let coarse = self.coarse_current * 10.0;
        let fine = (self.fine_current - 0.5) * 1.0;
        (coarse + fine).clamp(0.0, 10.0)
    }

    /// Actual voltage delivered, respecting Ohm's law.
    ///
    /// The machine delivers the LOWER of:
    /// - The voltage dial setting
    /// - Current dial setting × load resistance (V = I × R)
    ///
    /// When R is very low, even small current settings produce low voltage.
    /// When R is high, the voltage limit kicks in before the current limit.
    pub fn voltage_reading(&self) -> f64 {
        let v_set = self.voltage_dial_setting();
        let i_set = self.current_dial_setting();
        let r = self.load_resistance;

        if r <= 0.0 {
            return 0.0;
        }

        // Voltage limited by current setting: V = I_set × R
        let v_from_current = i_set * r;

        // The actual voltage is the minimum of the two limits
        v_set.min(v_from_current)
    }

    /// Actual current delivered, respecting Ohm's law.
    ///
    /// The machine delivers the LOWER of:
    /// - The current dial setting
    /// - Voltage dial setting / load resistance (I = V / R)
    pub fn amperage_reading(&self) -> f64 {
        let v_set = self.voltage_dial_setting();
        let i_set = self.current_dial_setting();
        let r = self.load_resistance;

        if r <= 0.0 {
            return 0.0;
        }

        // Current limited by voltage setting: I = V_set / R
        let i_from_voltage = v_set / r;

        // The actual current is the minimum of the two limits
        i_set.min(i_from_voltage)
    }

    /// Returns which factor is currently limiting the output.
    /// Useful for UI feedback.
    pub fn limiting_factor(&self) -> LimitingFactor {
        let v_set = self.voltage_dial_setting();
        let i_set = self.current_dial_setting();
        let r = self.load_resistance;

        if r <= 0.0 || (v_set == 0.0 && i_set == 0.0) {
            return LimitingFactor::Both;
        }

        let v_from_current = i_set * r;

        if (v_set - v_from_current).abs() < 0.01 {
            LimitingFactor::Both
        } else if v_set < v_from_current {
            LimitingFactor::Voltage
        } else {
            LimitingFactor::Current
        }
    }
}

impl Default for MachineState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LimitingFactor {
    /// Voltage dial is the active limit (brush plating mode)
    Voltage,
    /// Current dial is the active limit (tank plating mode)
    Current,
    /// Both limits are roughly equal
    Both,
}
