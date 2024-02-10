use clap::*;

#[derive(Debug, Clone, Parser)]
#[command(about = "Generate a random polish PESEL number")]
pub struct Arguments {
    #[arg(short, long, help = "Year of birth")]
    year: Option<u16>,

    #[arg(short, long, help = "Month of birth")]
    month: Option<u16>,

    #[arg(short, long, help = "Day of birth")]
    day: Option<u16>,

    #[arg(short, long, help = "Male or female")]
    gender: Option<String>,
}

pub fn handle(args: Arguments) -> crate::types::Output {
    super::generator::generate(super::generator::Input {
        year: args.year,
        month: args.month,
        day: args.day,
        gender: args.gender.clone(),
    })
}
