use rand::Rng;

use crate::types::Output;

#[derive(Debug)]
pub struct Input {}

pub fn generate(_input: Input) -> Option<Output> {
    let voivodeship = super::consts::VOIVODESHIPS[rand::thread_rng().gen_range(0..super::consts::VOIVODESHIPS.len())];
    let stage = format!("{voivodeship}{}", from_range!(100000..=999999).to_string());

    let sum = stage
        .clone()
        .chars()
        .zip(super::consts::WEIGHTS.iter())
        .map(|(a, b)| a.to_digit(10).unwrap() * b)
        .sum::<u32>();
    let checksum = sum % 11;

    let checksum = if checksum == 10 { 0 } else { checksum };

    let result = Output {
        value: format!("{}{}", stage, checksum),
        meta: meta![
            ("voivodeship".into(), voivodeship.to_string()),
            ("stage".into(), stage.to_string()),
            ("checksum".into(), checksum.to_string()),
        ],
    };

    Some(result)
}
