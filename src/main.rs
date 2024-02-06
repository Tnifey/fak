use clap::Parser;

#[macro_use]
mod macros;

#[path ="modules/mod.rs"]
pub mod modules;

mod commands;

#[derive(Debug, Parser)]
#[command(version, about="Generate fake data from thin air", long_about=None)]
#[command(arg_required_else_help = true)]
struct App {
    pub name: Option<String>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: commands::Commands,
}

fn main() {
    let app = App::parse();
    commands::commands(app.command);
}
