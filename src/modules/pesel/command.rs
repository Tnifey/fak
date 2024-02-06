use clap::*;

use super::generator::{generate, Input};

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value = "3")]
    count: Option<u16>,

    #[arg(short, long)]
    year: Option<u16>,

    #[arg(short, long)]
    month: Option<u16>,

    #[arg(short, long)]
    day: Option<u16>,

    #[arg(short, long)]
    sex: Option<String>,

    #[arg(short, long)]
    raw: Option<bool>,
}

pub fn handle(args: Arguments) {
    let Arguments {
        count,
        year,
        month,
        day,
        sex,
        raw,
    } = args;

    let raw = raw.unwrap_or(false);
    let count = count.unwrap_or(3);
    let input = Input {
        year,
        month,
        day,
        sex,
    };
    for _ in 0..count {
        let result = generate(input.clone());
        if let Some(result) = result {
            match raw {
                true => println!("{}", result.value),
                _ => println!("{}", result.value),
                // _ => println!("→ {} {}  →  {}", result.sex, result.date, result.pesel),
            }
        }
    }
}
