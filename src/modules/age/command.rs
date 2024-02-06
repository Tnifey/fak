use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long)]
    pub r#type: Option<String>,
}

pub fn handle(args: Arguments) {
    let r#type = args.r#type;
    let input = super::generator::Input {
        r#type,
    };
    let result = super::generator::generate(Some(input));
    if let Some(result) = result {
        println!("{}", result.age)
    }
}
