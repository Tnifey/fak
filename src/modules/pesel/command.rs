use clap::*;

use super::generator::{generate, Input};

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value = "1")]
    count: Option<u16>,

    #[arg(short, long)]
    year: Option<u16>,

    #[arg(short, long)]
    month: Option<u16>,

    #[arg(short, long)]
    day: Option<u16>,

    #[arg(short, long)]
    sex: Option<String>,
}

pub fn handle(args: Arguments) {
    let Arguments {
        count,
        year,
        month,
        day,
        sex,
    } = args;
    let input = Input {
        year,
        month,
        day,
        sex,
    };
    for _ in 0..count.unwrap_or(1) {
        let result = generate(input.clone());
        if let Some(result) = result {
            println!("{}", result.value)
        }
    }
}
