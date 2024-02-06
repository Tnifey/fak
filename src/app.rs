use clap::{Parser, Subcommand};
use crate::generators;

#[derive(Debug, Parser)]
#[command(version, about="Generate pesel or email", long_about=None)]
pub struct App {
    pub name: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
#[command(about = "Generate pesel")]
#[command(arg_required_else_help = true)]
pub enum Commands {
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
    Nip {
        #[arg(short, long, default_value = "3")]
        count: Option<u16>,
    },
    Regon {
        #[arg(short, long, default_value = "3")]
        count: Option<u16>,
    },
    Iban {
        #[arg(short, long, default_value = "3")]
        count: Option<u16>,

        #[arg(long)]
        cc: Option<String>,
    },
    Age {
        #[arg(short, long)]
        r#type: Option<String>,
    },
    Email {
        #[arg(short, long)]
        count: Option<u16>,

        #[arg(short, long)]
        pre: Option<String>,
    },
}

pub fn commands(command: Commands) {
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
            let input = generators::generate_pesel::PeselInput {
                year,
                month,
                day,
                sex: generators::generate_pesel::PeselInput::sex_from_option_string(sex),
            };
            for _ in 0..count {
                let result = generators::generate_pesel::generate_pesel(input.clone());
                if let Some(result) = result {
                    match raw {
                        true => println!("{}", result.pesel),
                        _ => println!("→ {} {}  →  {}", result.sex, result.date, result.pesel),
                    }
                }
            }
        }
        Commands::Nip { count } => {
            let count = count.unwrap_or(3);
            for _ in 0..count {
                let result = generators::generate_nip::generate_nip(generators::generate_nip::NipInput {});
                if let Some(result) = result {
                    println!("{}", result.nip)
                }
            }
        }
        Commands::Regon { count } => {
            let count = count.unwrap_or(3);
            for _ in 0..count {
                let result = generators::generate_regon::generate_regon(generators::generate_regon::RegonInput {});
                if let Some(result) = result {
                    println!("{}", result.regon)
                }
            }
        }
        Commands::Iban { count, cc } => {
            let count = count.unwrap_or(3);
            for _ in 0..count {
                let result = generators::generate_iban::generate_iban(generators::generate_iban::IBANInput {
                    country_code: cc.clone(),
                });
                if let Some(result) = result {
                    println!("{}", result.iban)
                }
            }
        }
        Commands::Email { count, pre } => {
            let count = count.unwrap_or(3);
            if let Some(pre) = pre {
                for _ in 0..count {
                    let result = generators::generate_email::generate_pre_email(&pre);
                    println!("{}", result);
                }
            }
        },
        Commands::Age { r#type } => {
            let input = generators::generate_age::AgeInput {
                r#type,
            };
            let result = generators::generate_age::generate_age(Some(input));
            if let Some(result) = result {
                println!("{}", result.age)
            }
        }
    }
}