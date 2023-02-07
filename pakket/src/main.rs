use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Install package(s)
    #[clap(alias = "i")]
    Install,
    /// Update package(s)
    #[clap(alias = "u")]
    Update,
}

fn main() {
    let args = Args::parse();
    match args.command {
        Some(Command::Install) => todo!("install"),
        Some(Command::Update) | None => todo!("update"),
    };
}
