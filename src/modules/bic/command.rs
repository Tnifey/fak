use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Arguments {
    #[arg(short, long)]
    pub pretty: Option<bool>,

    #[arg(short, long)]
    pub branch: Option<bool>,
}

pub fn handle(args: Arguments) {
    let pretty = args.pretty.unwrap_or(false);
    let input = super::generator::Input {
        branch: args.branch,
    };
    let result = super::generator::generate(Some(input));
    if let Some(result) = result {
        match pretty {
            true => println!("{}", super::generator::format_pretty(&result)),
            false => println!("{}", result.value),
        }
    }
}
