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

    #[arg(
        long,
        help = "Number of items to generate",
        default_value = "1",
        global = true
    )]
    pub count: u16,

    #[arg(
        long,
        help = "Pretty print the output",
        default_value = "false",
        global = true
    )]
    pub pretty: bool,
}

fn main() {
    let app = App::parse();
    let App { count, pretty, .. } = app;
    commands::commands(app.command, count, pretty);
}
