use super::generator::{generate, Input};
use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value = "1")]
    count: Option<u16>,
}

pub fn handle(args: Arguments) {
    let Arguments { count } = args;
    for _ in 0..count.unwrap_or(1) {
        let result = generate(Input {});
        if let Some(result) = result {
            println!("{}", result.regon)
        }
    }
}
