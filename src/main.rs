use chrono::Datelike;
use clap::{Parser, Subcommand};
use rand::Rng;

macro_rules! from_range {
    ($x:expr) => {
        rand::thread_rng().gen_range($x)
    };
}

#[derive(Debug, Parser)]
#[command(version, about)]
struct App {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
#[command(about)]
enum Commands {
    Pesel {
        #[arg(short, long)]
        count: Option<u16>,

        #[arg(short, long)]
        year: Option<u16>,

        #[arg(short, long)]
        month: Option<u16>,

        #[arg(short, long)]
        day: Option<u16>,

        #[arg(short, long)]
        sex: Option<String>,
    },

    Email {
        #[arg(short, long)]
        count: Option<u16>,

        #[arg(short, long)]
        pre: Option<String>,
    },
}

fn main() {
    let app = App::parse();
    if let Some(name) = app.command {
        match name {
            Commands::Pesel {
                count,
                year,
                month,
                day,
                sex,
            } => {
                let count = count.unwrap_or(10);
                let sex = match sex {
                    Some(sex) => match sex.to_lowercase().as_str() {
                        "m" => Some(true),
                        "f" => Some(false),
                        _ => None,
                    },
                    _ => None,
                };
                for _ in 0..count {
                    generate_pesel(year, month, day, sex);
                    println!()
                }
            }
            Commands::Email { count, pre } => {
                let count = count.unwrap_or(10);
                println!();
                if let Some(pre) = pre {
                    for _ in 0..count {
                        println!("{}", generate_pre_email(&pre));
                    }
                } else {
                    for _ in 0..count {
                        println!("{}", generate_email());
                    }
                }
                println!();
            }
        }
    }
}

fn generate_pre_email(email: &str) -> String {
    let [first, last] = match email.split_once('@') {
        Some((first, last)) => [first, last],
        None => panic!("Invalid email"),
    };

    let int = rand::random::<u32>();
    format!("{}+{}@{}", first, int, last)
}

fn generate_email() -> String {
    let mut rng = rand::thread_rng();
    let email: String = (0..10).map(|_| rng.gen_range(0..10).to_string()).collect();

    format!("xx@xxxxxxx.xxx {}", email)
}

fn generate_pesel(year: Option<u16>, month: Option<u16>, day: Option<u16>, sex: Option<bool>) {
    let date = match chrono::NaiveDate::from_ymd_opt(
        year.unwrap_or(from_range!(1970..2024)).into(),
        month.unwrap_or(from_range!(1..12)).into(),
        day.unwrap_or(from_range!(1..28)).into(),
    ) {
        Some(date) => date,
        None => panic!("Invalid date"),
    };

    let (y, m, d, is_genz) : (i32, i32, i32, bool)= (
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
            let m = if sex {11} else {10};
            from_range!(0..9) % m
        },
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

    println!();
    println!("pesel {}{}", parts, control);
    println!("date  {}", date.format("%Y-%m-%d"));
    println!("sex   {}", if sex % 2 == 0 { "F" } else { "M" });
}
