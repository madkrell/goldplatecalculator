#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GoldType {
    Gold24K,
    Gold18K,
    RoseGold,
    Gold14K,
}

impl GoldType {
    pub fn label(&self) -> &'static str {
        match self {
            GoldType::Gold24K => "24K Gold",
            GoldType::Gold18K => "18K Gold",
            GoldType::RoseGold => "Rose Gold",
            GoldType::Gold14K => "14K Gold",
        }
    }

    pub fn seconds_per_micron(&self) -> f64 {
        match self {
            GoldType::Gold24K => 190.0,
            GoldType::Gold18K | GoldType::RoseGold | GoldType::Gold14K => 300.0,
        }
    }

    pub fn all() -> &'static [GoldType] {
        &[
            GoldType::Gold24K,
            GoldType::Gold18K,
            GoldType::RoseGold,
            GoldType::Gold14K,
        ]
    }
}

pub const VERMEIL_MICRONS: f64 = 2.5;

pub fn plating_time_seconds(gold_type: GoldType, microns: f64) -> f64 {
    gold_type.seconds_per_micron() * microns
}

pub fn format_duration(total_seconds: f64) -> String {
    let total = total_seconds.round() as u64;
    let mins = total / 60;
    let secs = total % 60;
    if mins > 0 {
        format!("{} min {} sec", mins, secs)
    } else {
        format!("{} sec", secs)
    }
}
