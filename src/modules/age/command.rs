use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long, help = "Type of age to generate", default_value = "all")]
    pub r#type: Option<String>,
}

pub fn handle(args: Arguments) -> Option<crate::types::Output> {
    super::generator::generate(super::generator::Input {
        r#type: args.r#type,
    })
}
