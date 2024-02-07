use clap::Parser;

#[macro_use]
mod utils;
mod commands;
#[path = "modules/mod.rs"]
pub mod modules;
pub mod types;

#[derive(Debug, Parser)]
#[command(about="Generate fake data from thin air", long_about=None, version=None, author=None)]
#[command(arg_required_else_help = true)]
struct App {
    pub name: Option<String>,

    #[command(subcommand)]
    pub command: commands::Commands,
}

fn main() {
    let app = App::parse();
    commands::commands(app.command);
}
