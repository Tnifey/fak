use super::generator::{generate, Input, format_pretty};
use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value = "1")]
    count: Option<u16>,

    #[arg(long)]
    cc: Option<String>,

    #[arg(long)]
    pretty: Option<bool>,
}

pub fn handle(args: Arguments) {
    let Arguments { count, cc, pretty } = args;
    let count = count.unwrap_or(3);
    for _ in 0..count {
        let result = generate(Input { country_code: cc.clone() });
        if let Some(result) = result {
            if pretty.is_some_and(|x| x) {
                println!("{}", format_pretty(result));
            } else {
                println!("{}", result.value)
            }
        }
    }
}
