use super::presets::{iban_from_country_code, Iban};
use super::utils::{mod97, rand_alpha, rand_pattern10, rand_pattern100, to_digit_string};
use crate::types::Output;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {
    pub country_code: Option<String>,
}

pub fn generate(input: Input) -> Output {
    let iban = iban_from_country_code(input.country_code);
    let value = generator(iban.clone());

    let iban = value.clone();
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

    Output::meta(
        &value.to_string(),
        vec![
            ("IBAN", &iban),
            ("Country", &country),
            ("Checksum", &checksum),
            ("BBAN", &bban),
            ("Pretty", &pretty),
        ],
    )
}

pub fn generator(iban: Iban) -> String {
    let mut s = "".to_string();
    let mut count = 0;
    for bban in iban.bban.iter() {
        let mut c = bban.count;
        count += bban.count;
        while c > 0 {
            match bban.r#type.as_str() {
                "a" => s += &rand_alpha(),
                "c" => {
                    if probability!(0.8) {
                        s += &rand_alpha();
                    } else {
                        s += &from_range!(0..10).to_string();
                    }
                }
                _ => {
                    if c >= 3 && probability!(0.3) {
                        if probability!(0.5) {
                            s += &rand_pattern100();
                            c -= 2;
                        } else {
                            s += &rand_pattern10();
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

    let country = iban.country.clone();
    let format = format!("{s}{}00", iban.country);
    let digit_string = to_digit_string(format);
    let checksum = 98 - mod97(digit_string.clone());
    let checksum = match checksum {
        0..=9 => format!("0{}", checksum),
        _ => checksum.to_string(),
    };

    format!("{country}{checksum}{s}")
}
