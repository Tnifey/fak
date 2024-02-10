use crate::types::Output;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Input {}

pub fn generate(_args: Input) -> Option<Output> {
    let weights = [7, 3, 1, 9, 7, 3, 1, 7, 3];
    let pattern = [
        from_range!(10..=13), // A-D
        from_range!(10..=35), // A-Z
        from_range!(10..=35), // A-Z
        from_range!(0..=9),   // 0-9
        from_range!(0..=9),   // 0-9
        from_range!(0..=9),   // 0-9
        from_range!(0..=9),   // 0-9
        from_range!(0..=9),   // 0-9
                              // 0-9 checksum is calculated later
    ];
    let weight = pattern
        .iter()
        .zip(weights.iter())
        .map(|(x, w)| x * w)
        .sum::<u32>();
    let checksum = ((weight % 10) * 3) % 10;
    let value = pattern
        .map(|x| {
            std::char::from_digit(x, 36)
                .unwrap()
                .to_uppercase()
                .to_string()
        })
        .join("");
    let (series, number) = value.split_at(3);

    Output::meta(
        &format!("{series}{number}{checksum}"),
        vec![
            ("checksum", &format!("{checksum}")),
            ("series", &series),
            ("number", &number),
            ("value", &format!("{series}{number}{checksum}")),
        ],
    ).some()
}
