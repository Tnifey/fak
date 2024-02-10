use rand::Rng;
use once_cell::sync::Lazy;

pub const VENDOR: [&str; 2] = ["visa", "mastercard"];

#[derive(Debug, Clone)]
pub struct Preset {
    pub vendor: String,
    pub sizes: Vec<usize>,
    pub prefixes: Vec<String>,
}

impl Preset {
    pub fn random_prefix(&self) -> String {
        self.prefixes[from_range!(0..self.prefixes.len())].clone()
    }

    pub fn random_size(&self) -> usize {
        self.sizes[from_range!(0..self.sizes.len())]
    }
}

pub fn random_preset() -> Preset {
    let preset = VENDOR[from_range!(0..VENDOR.len())];
    match preset {
        "mastercard" => PRESET_MASTERCARD.clone(),
        _ => PRESET_VISA.clone(),
    }
}

pub static PRESET_VISA: Lazy<Preset> = once_cell::sync::Lazy::new(|| Preset {
    vendor: "visa".to_string(),
    sizes: vec![16],
    prefixes: vec!["4".to_string()],
});

pub static PRESET_MASTERCARD: Lazy<Preset> = once_cell::sync::Lazy::new(|| Preset {
    vendor: "mastercard".to_string(),
    sizes: vec![16],
    prefixes: vec![
        "51".to_string(),
        "52".to_string(),
        "53".to_string(),
        "54".to_string(),
        "55".to_string(),
    ],
});

