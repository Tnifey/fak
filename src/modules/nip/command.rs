use clap::*;

#[derive(Debug, Clone, Parser)]
#[command(name = "nip", about = "Generate random polish tax id (NIP) number")]
pub struct Arguments {}

pub fn handle(_args: Arguments) -> crate::types::Output {
    super::generator::generate(super::generator::Input {})
}
