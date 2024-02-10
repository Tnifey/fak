use super::generator::{generate, Input};
use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {}

pub fn handle(args: Arguments, count: u16, pretty: bool) {
    for _ in 0..count {
        let result = generate(Input {});
        if let Some(result) = result {
            result.print(pretty);
        }
    }
}
