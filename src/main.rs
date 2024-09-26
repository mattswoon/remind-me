use clap::{Parser, Subcommand};
use remind_me::error::Error;
use remind_me::data::{
    When,
    Reminder,
};
use remind_me::api::Store;
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Summary,
    Add { 
        what: String, 
        #[arg(value_parser=When::from_str)]
        when: When 
    },
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.action {
        Action::Summary => Ok(()),
        Action::Add { what, when } => {
            let store = Store::init()?;
            println!("Ok, I'll remind you \"{}\" at {}", &what, when.as_datetime().format("%H:%M on %Y-%m-%d"));
            store.insert_reminder(&Reminder::new(what, when.as_datetime()))?;
            Ok(())
        }
    }
}
