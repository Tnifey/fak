use rand::Rng;
use crate::{types::Output, utils::code_point};
use super::ibans::Ibans;

#[derive(Debug, Clone)]
pub struct Bban {
    pub r#type: String,
    pub count: u32,
}

#[derive(Debug, Clone)]
pub struct Iban {
    pub country: String,
    pub total: u32,
    pub bban: Vec<Bban>,
    pub format: String,
}

impl Iban {
    pub const ALPHA: [&'static str; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    pub const PATTERN10: [&'static str; 9] = ["01", "02", "03", "04", "05", "06", "07", "08", "09"];
    pub const PATTERN100: [&'static str; 9] = [
        "001", "002", "003", "004", "005", "006", "007", "008", "009",
    ];

    pub fn from_country_code(cc: Option<String>) -> Self {
        Ibans::from_country_code(cc)
    }

    pub fn rand_alpha() -> String {
        Self::ALPHA[from_range!(0..Self::ALPHA.len())].to_string()
    }

    pub fn rand_pattern10() -> String {
        Self::PATTERN10[from_range!(0..Self::PATTERN10.len())].to_string()
    }

    pub fn rand_pattern100() -> String {
        Self::PATTERN100[from_range!(0..Self::PATTERN100.len())].to_string()
    }

    pub fn mod97(digit_str: String) -> u32 {
        let mut m = 0;
        for element in digit_str.chars() {
            if let Some(element) = element.to_digit(10) {
                m = (m * 10 + element) % 97;
            }
        }
        m
    }

    pub fn to_digit_string(s: String) -> String {
        let s = s.chars();
        s.map(|c| {
            if !c.is_alphabetic() {
                return c.to_string();
            }
            let c = c.to_uppercase().next();
            if let Some(c) = c {
                return match code_point(c) {
                    Some(c) => (c - 55).to_string(),
                    None => "".to_string(),
                }
            }
            "".to_string()
        })
        .collect::<String>()
    }

    pub fn gen(&self) -> Option<String> {
        let mut s = "".to_string();
        let mut count = 0;
        for bban in self.bban.iter() {
            let mut c = bban.count;
            count += bban.count;
            while c > 0 {
                match bban.r#type.as_str() {
                    "a" => s += &Self::rand_alpha(),
                    "c" => {
                        if probability!(0.8) {
                            s += &Self::rand_alpha();
                        } else {
                            s += &from_range!(0..10).to_string();
                        }
                    }
                    _ => {
                        if c >= 3 && probability!(0.3) {
                            if probability!(0.5) {
                                s += &Self::rand_pattern100();
                                c -= 2;
                            } else {
                                s += &Self::rand_pattern10();
                                c -= 1;
                            }
                        } else {
                            s += &from_range!(0..10).to_string();
                        }
                    }
                };
                c -= 1;
            }
            s = s.chars().take(count as usize).collect::<String>();
        }

        let country = self.country.clone();
        let format = format!("{s}{}00", self.country);
        let digit_string = Self::to_digit_string(format);
        let mod97 = Self::mod97(digit_string.clone());

        let checksum = 98 - mod97;

        let checksum = if checksum < 10 {
            format!("0{}", checksum)
        } else {
            checksum.to_string()
        };

        Some(format!("{country}{checksum}{s}"))
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub country_code: Option<String>,
}

pub fn generate(input: Input) -> Option<Output> {
    let generator = Iban::from_country_code(input.country_code);
    let iban = generator.gen()?;

    let result = Output {
        value: iban,
        meta: None,
    };

    Some(result)
}

pub fn format_pretty(output: Output) -> String {
    println!("pretty");
    let iban = output.value.clone();
    let bban = iban.chars().skip(4).collect::<String>();
    let country = iban.chars().take(2).collect::<String>();
    let checksum = iban.chars().skip(2).take(2).collect::<String>();
    let pretty = iban
        .clone()
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");
    [
        format!("IBAN:     {}", iban),
        format!("Country:  {}", country),
        format!("Checksum: {}", checksum),
        format!("BBAN:     {}", bban),
        format!("Pretty:   {}", pretty),
    ].join("\n")
}