use clap::*;

#[derive(Debug, Clone, Parser)]
#[command(about = "Generate a random credit card number")]
pub struct Arguments {
    #[arg(short, long, help = "Vendor of the credit card (visa or mastercard)")]
    pub vendor: Option<String>,
}

pub fn handle(_args: Arguments) -> crate::types::Output {
    super::generator::generate(None)
}
