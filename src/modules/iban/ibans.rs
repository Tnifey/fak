use super::generator::{Bban, Iban};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Ibans {}

impl Ibans {
    pub fn random_country_code() -> String {
        Self::COUNTRY_CODES[from_range!(0..Self::COUNTRY_CODES.len())].to_string()
    }

    pub const COUNTRY_CODES: [&'static str; 7] = ["al", "at", "az", "bh", "be", "pl", "cz"];

    pub fn from_country_code(cc: Option<String>) -> Iban {
        let cc = cc.unwrap_or(Self::random_country_code()).to_lowercase();
        let cc = Self::COUNTRY_CODES
            .iter()
            .find(|&c| c == &cc)
            .map(|c| c.to_string())
            .unwrap_or(Self::random_country_code());

        match cc.to_lowercase().as_str() {
            "al" => Self::for_al(),
            "at" => Self::for_at(),
            "az" => Self::for_az(),
            "bh" => Self::for_bh(),
            "be" => Self::for_be(),
            "pl" => Self::for_pl(),
            _ => Self::for_pl(),
        }
    }

    pub fn for_pl() -> Iban {
        Iban {
            country: "PL".to_string(),
            total: 28,
            bban: vec![
                Bban {
                    r#type: "n".to_string(),
                    count: 8,
                },
                Bban {
                    r#type: "n".to_string(),
                    count: 16,
                },
            ],
            format: "PLkk bbbs sssx cccc cccc cccc cccc".to_string(),
        }
    }

    pub fn for_cz() -> Iban {
        Iban {
            country: "CZ".into(),
            total: 24,
            bban: vec![
                Bban {
                    r#type: "n".to_string(),
                    count: 10,
                },
                Bban {
                    r#type: "n".to_string(),
                    count: 10,
                },
            ],
            format: "CZkk bbbb ssss sscc cccc cccc".into(),
        }
    }

    pub fn for_al() -> Iban {
        Iban {
            country: "AL".into(),
            total: 28,
            bban: vec![
                Bban {
                    r#type: "n".into(),
                    count: 8,
                },
                Bban {
                    r#type: "c".into(),
                    count: 16,
                },
            ],
            format: "ALkk bbbs sssx cccc cccc cccc cccc".into(),
        }
    }

    pub fn for_at() -> Iban {
        Iban {
            country: "AT".into(),
            total: 20,
            bban: vec![
                Bban {
                    r#type: "n".into(),
                    count: 5,
                },
                Bban {
                    r#type: "c".into(),
                    count: 11,
                },
            ],
            format: "ATkk bbbb bccc cccc cccc".into(),
        }
    }

    pub fn for_az() -> Iban {
        Iban {
            country: "AZ".into(),
            total: 28,
            bban: vec![
                Bban {
                    r#type: "a".into(),
                    count: 4,
                },
                Bban {
                    r#type: "c".into(),
                    count: 20,
                },
            ],
            format: "AZkk bbbb cccc cccc cccc cccc cccc".into(),
        }
    }

    pub fn for_bh() -> Iban {
        Iban {
            country: "BH".into(),
            total: 22,
            bban: vec![
                Bban {
                    r#type: "a".into(),
                    count: 4,
                },
                Bban {
                    r#type: "c".into(),
                    count: 14,
                },
            ],
            format: "BHkk bbbb cccc cccc cccc cc".into(),
        }
    }

    pub fn for_be() -> Iban {
        Iban {
            country: "BE".into(),
            total: 16,
            bban: vec![
                Bban {
                    r#type: "n".into(),
                    count: 3,
                },
                Bban {
                    r#type: "n".into(),
                    count: 9,
                },
            ],
            format: "BEkk bbbc cccc ccxx".into(),
        }
    }
}
