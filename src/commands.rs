use clap::Subcommand;

use crate::modules::*;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Nip(nip::command::Arguments),
    Pesel(pesel::command::Arguments),
    Regon(regon::command::Arguments),
    Iban(iban::command::Arguments),
    Bic(bic::command::Arguments),
    CreditCard(credit_card::command::Arguments),
    IdentityCard(identity_card::command::Arguments),
    RandomInt(random_int::command::Arguments),
}

pub fn commands(command: Commands, count: u16, pretty: bool) {
    for i in 0..count {
        let result = match command {
            Commands::Pesel(ref args) => pesel::command::handle(args.clone()),
            Commands::Nip(ref args) => nip::command::handle(args.clone()),
            Commands::Regon(ref args) => regon::command::handle(args.clone()),
            Commands::Iban(ref args) => iban::command::handle(args.clone()),
            Commands::Bic(ref args) => bic::command::handle(args.clone()),
            Commands::CreditCard(ref args) => credit_card::command::handle(args.clone()),
            Commands::IdentityCard(ref args) => identity_card::command::handle(args.clone()),
            Commands::RandomInt(ref args) => random_int::command::handle(args.clone()),
        };

        result.print(pretty);
        if pretty && i < count - 1 {
            println!(" ");
        }
    }
}
