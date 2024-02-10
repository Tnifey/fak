use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {}

pub fn handle(args: Arguments, count: u16, pretty: bool) {
    for _ in 0..count {
        let result = super::generator::generate(super::generator::Input {});
        if let Some(result) = result {
            println!("{}", result.value)
        }
    }
}
