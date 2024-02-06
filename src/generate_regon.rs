use rand::Rng;

#[derive(Debug)]
pub struct RegonInput {}

#[derive(Debug)]
pub struct RegonResult {
    pub regon: String,
    pub checksum: u32,
}

pub fn generate_regon(_input: RegonInput) -> Option<RegonResult> {
    let weights = [8, 9, 2, 3, 4, 5, 6, 7];
    let voivodeships = [
        "01", "03", "05", "07", "09", "11", "13", "15", "17", "19", "21", "23", "25", "27", "29",
        "31", "33", "35", "37", "39", "41", "43", "45", "47", "49", "51", "53", "55", "57", "59",
        "61", "63", "65", "67", "69", "71", "73", "75", "77", "79", "81", "83", "85", "87", "89",
        "91", "93", "95", "97",
    ];
    let voivodeship = voivodeships[rand::thread_rng().gen_range(0..voivodeships.len())];
    let stage = format!("{voivodeship}{}", from_range!(100000..=999999).to_string());

    let sum = stage
        .chars()
        .zip(weights.iter())
        .map(|(a, b)| a.to_digit(10).unwrap() * b)
        .sum::<u32>();
    let checksum = sum % 11;

    let checksum = if checksum == 10 { 0 } else { checksum };

    Some(RegonResult {
        regon: format!("{}{}", stage, checksum),
        checksum,
    })
}
