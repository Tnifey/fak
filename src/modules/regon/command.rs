use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, default_value = "3")]
    count: Option<u16>,
}

pub fn handle(args: Arguments) {
    let Arguments { count } = args;

    let count = count.unwrap_or(3);
    for _ in 0..count {
        let result =
            super::generator::handle(super::generator::Input {});
        if let Some(result) = result {
            println!("{}", result.regon)
        }
    }
}
