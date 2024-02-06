use rand::Rng;

#[derive(Debug)]
pub struct Input {}

#[derive(Debug)]
pub struct Output {
    pub regon: String,
    pub checksum: u32,
}

pub static WEIGHTS : [u32; 8]= [8, 9, 2, 3, 4, 5, 6, 7];

pub static VOIVODESHIPS : [&str; 49] = [
    "01", "03", "05", "07", "09", "11", "13", "15", "17", "19", "21", "23", "25", "27", "29",
    "31", "33", "35", "37", "39", "41", "43", "45", "47", "49", "51", "53", "55", "57", "59",
    "61", "63", "65", "67", "69", "71", "73", "75", "77", "79", "81", "83", "85", "87", "89",
    "91", "93", "95", "97",
];

pub fn handle(_input: Input) -> Option<Output> {
    let voivodeship = VOIVODESHIPS[rand::thread_rng().gen_range(0..VOIVODESHIPS.len())];
    let stage = format!("{voivodeship}{}", from_range!(100000..=999999).to_string());

    let sum = stage
        .chars()
        .zip(WEIGHTS.iter())
        .map(|(a, b)| a.to_digit(10).unwrap() * b)
        .sum::<u32>();
    let checksum = sum % 11;

    let checksum = if checksum == 10 { 0 } else { checksum };

    Some(Output {
        regon: format!("{}{}", stage, checksum),
        checksum,
    })
}
