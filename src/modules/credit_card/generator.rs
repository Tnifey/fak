use crate::types::Output;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {
    vendor: Option<String>,
}

pub fn generate(input: Option<Input>) -> Option<Output> {
    let input = input.unwrap_or(Input { vendor: None });

    let preset = super::presets::random_preset();
    let prefix = preset.random_prefix();
    let number = (0..preset.digit_count)
        .map(|_| from_range!(0..10).to_string())
        .collect::<String>();
    let number_with_prefix = format!("{prefix}{number}");
    let checksum = calculate_checksum(number_with_prefix);

    let credit_card = format!("{prefix}{number}{checksum}");

    let is_valid = validate_checksum(credit_card.clone());

    println!("{} {} {}", credit_card, is_valid, preset.vendor);

    let pretty = credit_card
        .chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    let pretty = format!("{} ({}) ({})", pretty, preset.vendor, credit_card.len());

    Some(Output {
        value: pretty,
        meta: None,
    })
}

pub fn calculate_checksum(card_number: String) -> u32 {
    let reverse_card_number = card_number.chars().rev();
    let sum = reverse_card_number
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>();

    (((sum / 10) + 1) * 10 - sum) % 10
}

pub fn validate_checksum(card_number: String) -> bool {
    let (number, checksum) = card_number.split_at(card_number.len() - 1);
    let checksum = checksum.parse::<u32>().unwrap();
    let rev = number.chars().rev();
    let sum = rev
        .map(|c| c.to_digit(10).unwrap_or(0) * 2)
        .map(|c| if c > 9 { c - 9 } else { c })
        .sum::<u32>();
    sum % 10 == checksum
}
