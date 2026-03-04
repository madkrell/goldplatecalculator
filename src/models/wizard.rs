#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WizardStep {
    Pretreatment,
    Preparation,
    ElectrocleanVoltage,
    ElectrocleanAmperage,
    Rinse,
    Palladium,
    Gold,
    Finish,
}

impl WizardStep {
    pub fn number(&self) -> usize {
        match self {
            WizardStep::Pretreatment => 1,
            WizardStep::Preparation => 2,
            WizardStep::ElectrocleanVoltage => 3,
            WizardStep::ElectrocleanAmperage => 4,
            WizardStep::Rinse => 5,
            WizardStep::Palladium => 6,
            WizardStep::Gold => 7,
            WizardStep::Finish => 8,
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            WizardStep::Pretreatment => "Pre-treatment: Silver Deoxidising",
            WizardStep::Preparation => "Preparation: Copper Wire & Surface Area",
            WizardStep::ElectrocleanVoltage => "Electrocleaner: Voltage Clean",
            WizardStep::ElectrocleanAmperage => "Electrocleaner: Amperage Setting",
            WizardStep::Rinse => "Rinse & Water Break Test",
            WizardStep::Palladium => "Palladium Tank",
            WizardStep::Gold => "Gold Tank",
            WizardStep::Finish => "Finish",
        }
    }

    pub fn short_title(&self) -> &'static str {
        match self {
            WizardStep::Pretreatment => "Pre-treat",
            WizardStep::Preparation => "Prep",
            WizardStep::ElectrocleanVoltage => "EC Voltage",
            WizardStep::ElectrocleanAmperage => "EC Amps",
            WizardStep::Rinse => "Rinse",
            WizardStep::Palladium => "Palladium",
            WizardStep::Gold => "Gold",
            WizardStep::Finish => "Finish",
        }
    }

    pub fn all_steps() -> &'static [WizardStep] {
        &[
            WizardStep::Pretreatment,
            WizardStep::Preparation,
            WizardStep::ElectrocleanVoltage,
            WizardStep::ElectrocleanAmperage,
            WizardStep::Rinse,
            WizardStep::Palladium,
            WizardStep::Gold,
            WizardStep::Finish,
        ]
    }

    pub fn next(&self) -> Option<WizardStep> {
        let steps = Self::all_steps();
        let idx = steps.iter().position(|s| s == self)?;
        steps.get(idx + 1).copied()
    }

    pub fn prev(&self) -> Option<WizardStep> {
        let steps = Self::all_steps();
        let idx = steps.iter().position(|s| s == self)?;
        if idx > 0 {
            steps.get(idx - 1).copied()
        } else {
            None
        }
    }

    pub const TOTAL: usize = 8;
}
