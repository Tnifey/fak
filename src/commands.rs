use clap::Subcommand;

use crate::modules::*;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Age(age::command::Arguments),
    Nip(nip::command::Arguments),
    Pesel (pesel::command::Arguments),
    Regon(regon::command::Arguments),
    // Iban(iban::command::Arguments),
}

pub fn commands(command: Commands) {
    match command {
        Commands::Age(args) => age::command::handle(args),
        Commands::Pesel(args) => pesel::command::handle(args),
        Commands::Nip(args) => nip::command::handle(args),
        Commands::Regon(args) => regon::command::handle(args),
        // Commands::Iban(args) => iban::command::handle(args),
    }
}
