use crate::types::Output;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {}

pub fn generate(_args: Input) -> Option<Output> {
    let weights = [7, 3, 1, 9, 7, 3, 1, 7, 3];
    let pattern = [
        vec![
            from_range!(10..=13), // A-D
            from_range!(10..=35), // A-Z
            from_range!(10..=35), // A-Z
        ],
        (0..5).map(|_| from_range!(0..=9)).collect::<Vec<u32>>(),
    ]
    .concat();
    let cost = pattern
        .iter()
        .zip(weights.iter())
        .map(|(x, w)| x * w)
        .sum::<u32>();
    let checksum = find_checksum(cost, weights[weights.len() - 1]);
    let series = pattern[0..3]
        .iter()
        .map(|x| {
            std::char::from_digit(*x, 36)
                .unwrap()
                .to_uppercase()
                .to_string()
        })
        .collect::<Vec<String>>();
    let number = pattern[3..8]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let identity_card = format!("{}{}{checksum}", series.join(""), number.join(""));

    Some(Output {
        value: identity_card.clone(),
        meta: meta!(
            ("checksum".into(), format!("{checksum}")),
            ("series".into(), series.join("")),
            ("number".into(), number.join("")),
            ("value".into(), identity_card),
        ),
    })
}

pub fn find_checksum(cost: u32, weight: u32) -> u32 {
    for i in 0..10 {
        if (cost + (i * weight)) % 10 == 0 {
            return i;
        }
    }
    0
}
