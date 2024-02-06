use rand::Rng;

#[derive(Debug)]
pub struct NipInput {}

#[derive(Debug)]
pub struct NipResult {
    pub nip: String,
    pub checksum: u32,
}

pub fn generate_nip(_input: NipInput) -> Option<NipResult> {
    let weights = [6, 5, 7, 2, 3, 4, 5, 6, 7];
    let stage = from_range!(100000000..=999999999).to_string();

    let checksum = stage
        .chars()
        .zip(weights.iter())
        .map(|(a, b)| a.to_digit(10).unwrap() * b)
        .sum::<u32>()
        % 11;

    if checksum == 10 {
        return generate_nip(_input);
    }

    Some(NipResult {
        nip: format!("{}{}", stage, checksum),
        checksum,
    })
}
