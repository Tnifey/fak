use super::generator::{generate, Input, format_pretty};
use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(long, help="Country code")]
    cc: Option<String>,
}

pub fn handle(args: Arguments, count: u16, pretty: bool) {
    let Arguments { cc } = args;
    for _ in 0..count {
        let result = generate(Input { country_code: cc.clone() });
        if let Some(result) = result {
            if pretty {
                println!("{}", format_pretty(result));
            } else {
                println!("{}", result.value)
            }
        }
    }
}
