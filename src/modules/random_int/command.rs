use clap::*;

#[derive(Debug, Clone, Parser)]
#[command(about = "Generate random int, or coin flip", alias = "int")]
pub struct Arguments {
    #[arg(short, long, default_value = "0")]
    pub min: Option<u32>,

    #[arg(short, long, default_value = "1")]
    pub max: Option<u32>,
}

pub fn handle(args: Arguments) -> crate::types::Output {
    super::generator::generate(super::generator::Input {
        min: args.min,
        max: args.max,
    })
}
