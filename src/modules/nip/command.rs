use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {}

pub fn handle(_args: Arguments) -> Option<crate::types::Output> {
    super::generator::generate(super::generator::Input {})
}
