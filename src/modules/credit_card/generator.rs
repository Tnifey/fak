use crate::types::Output;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {}

pub fn generate(_input: Option<Input>) -> Output {
    let preset = super::presets::random_preset();
    let (prefix, size) = (preset.random_prefix(), preset.random_size());
    let count = size - prefix.len() - 1;
    let holder_number = (0..count)
        .map(|_| from_range!(0..10).to_string())
        .collect::<String>();
    let card_number = format!("{prefix}{holder_number}");

    let checksum = {
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
    };

    let credit_card = format!("{card_number}{checksum}");

    Output::meta(
        &credit_card,
        vec![
            ("Vendor      ", &preset.vendor),
            ("Prefix      ", &prefix),
            ("Size        ", &size.to_string()),
            ("Checksum    ", &checksum.to_string()),
            ("Card number ", &credit_card),
        ],
    )
}

#[test]
fn test_generate() {
    let output = generate(None);
    assert!(output.value.len() >= 13 && output.value.len() <= 19);
}
