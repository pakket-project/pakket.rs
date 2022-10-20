use clap::Args;

#[derive(Debug, Args)]
pub struct Install {
    pub path: String,
}

impl Install {
    pub fn execute(args: Install) {
        println!("Installing {}", args.path);
    }
}
