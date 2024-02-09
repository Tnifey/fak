use rand::Rng;

use crate::utils::code_point;

pub const ALPHA: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z",
];
pub const PATTERN10: [&str; 9] = ["01", "02", "03", "04", "05", "06", "07", "08", "09"];
pub const PATTERN100: [&str; 9] = [
    "001", "002", "003", "004", "005", "006", "007", "008", "009",
];

pub fn rand_alpha() -> String {
    ALPHA[from_range!(0..ALPHA.len())].into()
}

pub fn rand_pattern10() -> String {
    PATTERN10[from_range!(0..PATTERN10.len())].into()
}

pub fn rand_pattern100() -> String {
    PATTERN100[from_range!(0..PATTERN100.len())].into()
}

pub fn mod97(digit_str: String) -> u32 {
    let mut m = 0;
    for element in digit_str.chars() {
        if let Some(element) = element.to_digit(10) {
            m = (m * 10 + element) % 97;
        }
    }
    m
}

pub fn to_digit_string(s: String) -> String {
    let s = s.chars();
    s.map(|c| {
        if !c.is_alphabetic() {
            return c.into();
        }
        let c = c.to_uppercase().next();
        if let Some(c) = c {
            return match code_point(c) {
                Some(c) => (c - 55).to_string(),
                None => "".into(),
            };
        }
        "".into()
    })
    .collect::<String>()
}
