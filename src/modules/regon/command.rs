use clap::*;

#[derive(Debug, Clone, Parser)]
#[command(name = "regon", about = "Generate random polish REGON number")]
pub struct Arguments {}

pub fn handle(_args: Arguments) -> crate::types::Output {
    super::generator::generate(super::generator::Input {})
}
