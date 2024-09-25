use clap::{Parser, Subcommand};
use remind_me::error::Error;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Summary,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.action {
        Action::Summary => Ok(())
    }
}
