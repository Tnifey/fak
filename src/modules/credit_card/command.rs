use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, help = "Vendor of the credit card")]
    pub vendor: Option<String>,
}

pub fn handle(args: Arguments, count: u16, pretty: bool) {
    for _ in 0..count {
        let result = super::generator::generate(None);
        if let Some(result) = result {
            result.print(pretty);
        }
    }
}
