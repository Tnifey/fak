use clap::{Parser, Subcommand};

#[macro_use]
mod macros;

mod generate_pesel;
mod generate_email;

#[derive(Debug, Parser)]
#[command(version, about="Generate pesel or email", long_about=None)]
struct App {
    name: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
#[command(about = "Generate pesel")]
#[command(arg_required_else_help = true)]
enum Commands {
    Pesel {
        #[arg(short, long, default_value = "3")]
        count: Option<u16>,

        #[arg(short, long)]
        year: Option<u16>,

        #[arg(short, long)]
        month: Option<u16>,

        #[arg(short, long)]
        day: Option<u16>,

        #[arg(short, long)]
        sex: Option<String>,

        #[arg(short, long)]
        raw: Option<bool>,
    },
    Email {
        #[arg(short, long)]
        count: Option<u16>,

        #[arg(short, long)]
        pre: Option<String>,
    },
}

fn main() {
    let app = App::parse();
    if let Some(command) = app.command {
        match command {
            Commands::Pesel {
                count,
                year,
                month,
                day,
                sex,
                raw,
            } => {
                let raw = raw.unwrap_or(false);
                let count = count.unwrap_or(3);
                let input = generate_pesel::PeselInput {
                    year,
                    month,
                    day,
                    sex: generate_pesel::PeselInput::sex_from_option_string(sex),
                };
                for _ in 0..count {
                    let result = generate_pesel::generate_pesel(input.clone());
                    if let Some(result) = result {
                        match raw {
                            true => println!("{}", result.pesel),
                            _ => println!("→ {} {}  →  {}", result.sex, result.date, result.pesel),
                        }
                    }
                }
            }
            Commands::Email { count, pre } => {
                let count = count.unwrap_or(3);
                if let Some(pre) = pre {
                    for _ in 0..count {
                        let result = generate_email::generate_pre_email(&pre);
                        println!("{}", result);
                    }
                }
            },
        }
    }
}

