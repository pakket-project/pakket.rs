mod commands;

use clap::{Parser, Subcommand};
// use std::env;

#[derive(Debug, Parser)]
#[command()]
struct PakketCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Install(commands::install::Install),
}

fn main() {
    // if env::consts::OS != "macos" {
    //     println!("Oops, you can only run Pakket on MacOS!");
    //     return;
    // }

    let args = PakketCli::parse();

    match args.command {
        Commands::Install(args) => {
            commands::install::Install::execute(args);
        }
    }
}
