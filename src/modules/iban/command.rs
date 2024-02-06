use super::generator::{generate, Input};
use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value = "1")]
    count: Option<u16>,

    #[arg(long)]
    cc: Option<String>,
}

pub fn handle(args: Arguments) {
    let Arguments { count, cc } = args;
    let count = count.unwrap_or(3);
    for _ in 0..count {
        let result = generate(Input { country_code: cc.clone() });
        if let Some(result) = result {
            println!("{:?}", result)
        }
    }
}
