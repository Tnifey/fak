use clap::Parser;

#[macro_use]
mod macros;

#[path ="generators/mod.rs"]
mod generators;
mod app;

fn main() {
    let app = app::App::parse();
    if let Some(command) = app.command {
        app::commands(command);
    } else {
        println!("Invalid command");
    }
}