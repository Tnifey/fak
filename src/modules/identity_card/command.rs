use clap::*;
use crate::types::Output;

#[derive(Debug, Clone, Parser)]
#[command(about = "Generate a random polish identity card number", alias="dowod")]
pub struct Arguments {}

pub fn handle(_args: Arguments) -> Output {
   super::generator::generate(super::generator::Input {})
}
