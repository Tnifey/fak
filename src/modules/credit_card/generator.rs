use std::collections::HashMap;

use crate::types::Output;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input { }

pub fn generate(input: Option<Input>) -> Option<Output> {
    let _ = input.unwrap_or(Input { });

    let preset = super::presets::random_preset();
    let (prefix, size) = (preset.random_prefix(), preset.random_size());
    let count = size - prefix.len() - 1;
    let holder_number = (0..count)
        .map(|_| from_range!(0..10).to_string())
        .collect::<String>();
    let card_number = format!("{prefix}{holder_number}");

    let checksum = calculate_checksum(card_number.clone());

    let credit_card = format!("{card_number}{checksum}");

    let meta: HashMap<String, String> = HashMap::from([
        ("vendor".into(), preset.vendor),
        ("prefix".into(), prefix),
        ("size".into(), size.to_string()),
    ]);

    let output = Output {
        value: credit_card,
        meta: Some(meta),
    };

    Some(output)
}

pub fn calculate_checksum(card_number: String) -> u32 {
    let reverse_card_number = card_number.chars().rev();
    let sum = reverse_card_number
        .enumerate()
        .map(|(i, c)| {
            let num = c.to_digit(10).unwrap();
            let num = if i % 2 == 0 { num * 2 } else { num };
            match num {
                0..=9 => num,
                _ => num - 9,
            }
        })
        .sum::<u32>();

    let div = sum / 10 + 1;
    let cal = div * 10 - sum;
    cal % 10
}
