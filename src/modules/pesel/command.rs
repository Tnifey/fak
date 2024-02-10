use clap::*;

use super::generator::{generate, Input};

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, help = "Year of birth")]
    year: Option<u16>,

    #[arg(short, long, help = "Month of birth")]
    month: Option<u16>,

    #[arg(short, long, help = "Day of birth")]
    day: Option<u16>,

    #[arg(short, long, help = "Male or female")]
    gender: Option<String>,
}

pub fn handle(args: Arguments, count: u16, pretty: bool) {
    for _ in 0..count {
        let result = generate(Input {
            year: args.year,
            month: args.month,
            day: args.day,
            gender: args.gender.clone(),
        });
        if let Some(result) = result {
            result.print(pretty);
            println!("{}", result.value)
        }
    }
}
