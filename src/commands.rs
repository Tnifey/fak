use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Age(crate::modules::age::command::Arguments),
    Pesel (crate::modules::pesel::command::Arguments),
    Nip(crate::modules::nip::command::Arguments),
    Regon(crate::modules::regon::command::Arguments),
    // Iban {
    //     #[arg(short, long, default_value = "3")]
    //     count: Option<u16>,

    //     #[arg(long)]
    //     cc: Option<String>,
    // },
}

pub fn commands(command: Commands) {
    match command {
        Commands::Age(args) => crate::modules::age::command::handle(args),
        Commands::Pesel(args) => crate::modules::pesel::command::handle(args),
        Commands::Nip(args) => crate::modules::nip::command::handle(args),
        Commands::Regon(args) => crate::modules::regon::command::handle(args),
        
    }
}