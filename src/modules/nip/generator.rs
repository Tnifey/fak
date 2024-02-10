use rand::Rng;

use crate::types::Output;

#[derive(Debug, Clone)]
pub struct Input {}

pub fn generate(input: Input) -> Output {
    let weights = [6, 5, 7, 2, 3, 4, 5, 6, 7];
    let stage = from_range!(100000000..=999999999).to_string();

    let checksum = stage
        .chars()
        .zip(weights.iter())
        .map(|(a, b)| a.to_digit(10).unwrap() * b)
        .sum::<u32>()
        % 11;

    if checksum == 10 {
        return generate(input.clone());
    }

    let value = format!("{}{}", stage, checksum);

    Output::new(value)
}
