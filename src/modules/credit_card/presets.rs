use rand::Rng;
use once_cell::sync::Lazy;

pub const VENDOR: [&str; 2] = ["visa", "mastercard"];

#[derive(Debug, Clone)]
pub struct Preset {
    pub vendor: String,
    pub digit_count: u16,
    pub prefixes: Vec<String>,
}

impl Preset {
    pub fn random_prefix(&self) -> String {
        self.prefixes[from_range!(0..self.prefixes.len())].clone()
    }
}

pub fn random_preset() -> Preset {
    match from_range!(0..VENDOR.len()) {
        0 => PRESET_VISA.clone(),
        1 => PRESET_MASTERCARD.clone(),
        _ => PRESET_VISA.clone(),
    }
}

pub static PRESET_VISA: Lazy<Preset> = once_cell::sync::Lazy::new(|| Preset {
    vendor: "visa".to_string(),
    digit_count: 16,
    prefixes: vec![
        "4".to_string(),
    ],
});

pub static PRESET_VISA_ELECTRON: Lazy<Preset> = once_cell::sync::Lazy::new(|| Preset {
    vendor: "visa_electron".to_string(),
    digit_count: 16,
    prefixes: vec![
        "4539".to_string(),
        "4556".to_string(),
        "4916".to_string(),
        "4532".to_string(),
        "4929".to_string(),
        "4485".to_string(),
        "4716".to_string(),
    ],
});

pub static PRESET_MASTERCARD: Lazy<Preset> = once_cell::sync::Lazy::new(|| Preset {
    vendor: "mastercard".to_string(),
    digit_count: 16,
    prefixes: vec![
        "51".to_string(),
        "52".to_string(),
        "53".to_string(),
        "54".to_string(),
        "55".to_string(),
    ],
});

