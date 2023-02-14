use std::{fs, io};

use clap::{Parser, Subcommand};
use pakket_core::{config::Config, paths::Path};

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

fn main() -> io::Result<()> {
    let args = Args::parse();

    let config: Config =
        toml::from_str(&fs::read_to_string(Path::Config.to_path_buf()).unwrap_or_default())
            .unwrap_or_default();

    match args.command {
        Some(Command::Install) => todo!("install"),
        Some(Command::Update) | None => todo!("update"),
    };
}
