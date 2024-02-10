use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long)]
    pub vendor: Option<String>,
}

pub fn handle(args: Arguments) {
    let result = super::generator::generate(None);
    if let Some(result) = result {
        println!("{}", result.value);
    }
}
