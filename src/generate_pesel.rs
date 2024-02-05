use chrono::Datelike;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct PeselInput {
    pub year: Option<u16>,
    pub month: Option<u16>,
    pub day: Option<u16>,
    pub sex: Option<bool>,
}

impl PeselInput {
    pub fn sex_from_option_string(sex: Option<String>) -> Option<bool> {
        match sex {
            Some(sex) => match sex.to_lowercase().as_str() {
                "f" => Some(true),
                "m" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }
}


#[derive(Debug, Clone)]
pub struct PeselResult {
    pub date: String,
    pub pesel: String,
    pub sex: String,
}

pub fn generate_pesel(input: PeselInput) -> Option<PeselResult> {
    let PeselInput {
        year,
        month,
        day,
        sex,
        ..
    } = input;

    let date = chrono::NaiveDate::from_ymd_opt(
        year.unwrap_or(from_range!(1970..2024)).into(),
        month.unwrap_or(from_range!(1..12)).into(),
        day.unwrap_or(from_range!(1..28)).into(),
    ).unwrap();

    let (y, m, d, is_genz): (i32, i32, i32, bool) = (
        date.year(),
        (date.month0() + 1).try_into().unwrap(),
        date.day().try_into().unwrap(),
        date.year() >= 2000,
    );

    let year = match if is_genz { y - 2000 } else { y - 1900 } {
        0 => "00".into(),
        year => match year {
            1..=9 => format!("0{}", year),
            _ => year.to_string(),
        },
    };

    let month = if is_genz { m + 20 } else { m };
    let month = match month {
        0..=9 => format!("0{}", month),
        _ => month.to_string(),
    };

    let day = if d < 10 {
        format!("0{}", d)
    } else {
        d.to_string()
    };

    let ran = from_range!(100..999);
    let sex = match sex {
        Some(sex) => {
            let daps = (0..9)
                .filter(|i| i % 2 == if sex { 0 } else { 1 })
                .collect::<Vec<i32>>();
            daps[from_range!(0..daps.len())]
        }
        None => from_range!(0..9),
    };
    let parts = format!("{}{}{}{}{}", year, month, day, ran, sex);
    let controllist = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];
    let mut sum = 0;
    for (i, c) in parts.chars().enumerate() {
        let c = c.to_digit(10).unwrap();
        sum += c * controllist[i];
    }

    let control = match sum % 10 {
        0 => 0,
        sum => 10 - (sum % 10),
    };

    let result = PeselResult {
        pesel: format!("{}{}", parts, control), 
        date: date.format("%Y-%m-%d").to_string(),
        sex: (if sex % 2 == 0 { "F" } else { "M" }).into(),
    };

    Some(result)
}
