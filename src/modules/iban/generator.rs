use rand::Rng;
use regex::Regex;

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

    pub const COUNTRY_CODES: [&'static str; 1] = ["pl"];

    pub fn from_country_code(cc: Option<String>) -> Self {
        let cc = cc.unwrap_or("pl".to_string()).to_lowercase();
        let cc = Iban::COUNTRY_CODES
            .iter()
            .find(|&c| c == &cc)
            .map(|c| c.to_string())
            .unwrap_or(Iban::COUNTRY_CODES[from_range!(0..Iban::COUNTRY_CODES.len())].to_string());

        match cc.to_lowercase().as_str() {
            "pl" => Self::for_pl(),
            "cz" => Self::for_cz(),
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

    pub fn rand_alpha() -> String {
        Iban::ALPHA[rand::thread_rng().gen_range(0..Iban::ALPHA.len())].to_string()
    }

    pub fn rand_pattern10() -> String {
        Iban::PATTERN10[rand::thread_rng().gen_range(0..Iban::PATTERN10.len())].to_string()
    }

    pub fn rand_pattern100() -> String {
        Iban::PATTERN100[rand::thread_rng().gen_range(0..Iban::PATTERN100.len())].to_string()
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
            if c.is_alphabetic() {
                println!("{}", c);
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
    }

    pub fn gen(&self) -> Option<String> {
        let mut s: String = "".into();
        let mut count = 0;
        for bban in self.bban.iter() {
            let mut c = bban.count;
            count += bban.count;
            while c > 0 {
                match bban.r#type.as_str() {
                    "a" => {
                        s += &Iban::rand_alpha();
                    }
                    "c" => {
                        s += &Iban::rand_alpha();
                    }
                    _ => {
                        s += &rand::thread_rng().gen_range(0..10).to_string();
                    }
                }
                c -= 1;
            }
            s = s.chars().take(count as usize).collect::<String>();
        }

        let digit_string = Iban::to_digit_string(format!("{s}{}00", self.country));
        println!("digit_string_out: {}", digit_string);
        let mod97 = Iban::mod97(digit_string.clone());

        let checksum = 98 - mod97;

        let checksum = if checksum < 10 {
            format!("0{}", checksum)
        } else {
            checksum.to_string()
        };

        println!("{}{checksum}{s}", self.country);

        None
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub country_code: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub country_code: Option<String>,
    pub iban: String,
}

pub fn generate(input: Input) -> Option<Output> {
    let iban = Iban::from_country_code(input.country_code);
    let x = format!("{:?}", iban);
    let gen = iban.gen()?;
    println!("{}", x);
    println!("{:?}", gen);

    None
}
