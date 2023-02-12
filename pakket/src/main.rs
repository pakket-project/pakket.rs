mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about = "pakket is a package manager for macOS")]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Install package(s)
    #[clap(alias = "i")]
    Install,
    /// Update packages
    #[clap(alias = "u")]
    Update,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Some(Command::Install) => todo!("install"),
        Some(Command::Update) | None => commands::update(),
    };
}
