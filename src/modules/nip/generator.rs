use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {}

#[derive(Debug, Clone)]
pub struct Output {
    pub nip: String,
    pub checksum: u32,
}

pub fn generate(input: Input) -> Option<Output> {
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

    Some(Output {
        nip: format!("{}{}", stage, checksum),
        checksum,
    })
}
