use rand::Rng;

use crate::types::Output;

#[derive(Debug)]
pub struct Input {}

pub fn generate(_input: Input) -> Output {
    let voivodeship = super::presets::VOIVODESHIPS[rand::thread_rng().gen_range(0..super::presets::VOIVODESHIPS.len())];
    let stage = format!("{voivodeship}{}", from_range!(100000..=999999).to_string());

    let sum = stage
        .clone()
        .chars()
        .zip(super::presets::WEIGHTS.iter())
        .map(|(a, b)| a.to_digit(10).unwrap() * b)
        .sum::<u32>();
    let checksum = sum % 11;

    let checksum = if checksum == 10 { 0 } else { checksum };

    Output::meta(
        &format!("{}{}", stage, checksum),
        vec![
            ("voivodeship", &voivodeship),
            ("stage", &stage),
            ("checksum", &checksum.to_string()),
        ],
    )
}
