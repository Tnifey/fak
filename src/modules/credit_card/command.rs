use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, help = "Vendor of the credit card")]
    pub vendor: Option<String>,
}

pub fn handle(_args: Arguments) -> Option<crate::types::Output> {
    super::generator::generate(None)
}
