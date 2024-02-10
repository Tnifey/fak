use clap::Subcommand;

use crate::{modules::*, types::Output};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Age(age::command::Arguments),
    Nip(nip::command::Arguments),
    Pesel (pesel::command::Arguments),
    Regon(regon::command::Arguments),
    Iban(iban::command::Arguments),
    Bic(bic::command::Arguments),
    CreditCard(credit_card::command::Arguments),
}

pub fn commands(command: Commands, count: u16, pretty: bool) {
    println!("count: {}", count);
    println!("pretty: {}", pretty);
    match command {
        Commands::Age(args) => age::command::handle(args, count, pretty),
        Commands::Pesel(args) => pesel::command::handle(args, count, pretty),
        Commands::Nip(args) => nip::command::handle(args, count, pretty),
        Commands::Regon(args) => regon::command::handle(args, count, pretty),
        Commands::Iban(args) => iban::command::handle(args, count, pretty),
        Commands::Bic(args) => bic::command::handle(args, count, pretty),
        Commands::CreditCard(args) => credit_card::command::handle(args, count, pretty),
    };
}
