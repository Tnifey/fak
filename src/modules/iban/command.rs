use clap::*;

#[derive(Debug, Clone, Parser)]
#[command(about = "Generate a random IBAN number")]
pub struct Arguments {
    #[arg(long, help = "Country code in ISO 3166-1 alpha-2 format")]
    cc: Option<String>,
}

pub fn handle(args: Arguments) -> Option<crate::types::Output> {
    super::generator::generate(super::generator::Input {
        country_code: args.cc,
    })
}
