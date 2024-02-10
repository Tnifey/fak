use clap::*;

#[derive(Debug, Clone, Parser)]
#[command(about = "Generate a random Business Identifier Code (BIC) number")]
pub struct Arguments {
    #[arg(short, long)]
    pub branch: Option<bool>,
}

pub fn handle(args: Arguments) -> Option<crate::types::Output> {
    super::generator::generate(super::generator::Input {
        branch: args.branch,
    })
}
