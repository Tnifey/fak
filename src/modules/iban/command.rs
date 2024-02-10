use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(long, help = "Country code")]
    cc: Option<String>,
}

pub fn handle(args: Arguments) -> Option<crate::types::Output> {
    super::generator::generate(super::generator::Input {
        country_code: args.cc,
    })
}
