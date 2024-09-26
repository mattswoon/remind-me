use clap::{Parser, Subcommand};
use remind_me::error::Error;
use remind_me::data::{
    When,
    Reminder,
};
use remind_me::api::Store;
use std::str::FromStr;
use chrono::Local;
use comfy_table::{
    Table,
    Cell,
    Color,
    presets::UTF8_FULL,
};

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
    Find {
        what: String,
    },
    Dismiss { 
        id: i64, 
    }
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.action {
        Action::Summary => {
            let store = Store::init()?;
            let mut table = Table::new();
            let now = Local::now();
            table
                .load_preset(UTF8_FULL)
                .set_header(vec!["Id", "What", "When", "Time remaining (HH:MM)"]);
            for reminder in store.list_active()? {
                if reminder.value.is_overdue(now) {
                    table.add_row(vec![
                        Cell::new(&reminder.id).fg(Color::Red),
                        Cell::new(&reminder.value.what).fg(Color::Red),
                        Cell::new(reminder.value.when.format("%H:%M %Y-%m-%d")).fg(Color::Red),
                        Cell::new(reminder.value.time_remaining_str(now)).fg(Color::Red),
                    ]);
                } else {
                    table.add_row(vec![
                        Cell::new(&reminder.id),
                        Cell::new(&reminder.value.what),
                        Cell::new(reminder.value.when.format("%H:%M %Y-%m-%d")),
                        Cell::new(reminder.value.time_remaining_str(now)),
                    ]);
                }
            }
            println!("{}", table);
            Ok(())
        },
        Action::Add { what, when } => {
            let store = Store::init()?;
            println!("Ok, I'll remind you \"{}\" at {}", &what, when.as_datetime().format("%H:%M on %Y-%m-%d"));
            store.insert_reminder(&Reminder::new(what, when.as_datetime()))?;
            Ok(())
        },
        Action::Find { what } => {
            let store = Store::init()?;
            let mut table = Table::new();
            table 
                .load_preset(UTF8_FULL)
                .set_header(vec!["Id", "What", "When"]);
            let now = Local::now();
            for reminder in store.find_by_what(&what)? {
                if reminder.value.is_overdue(now) {
                    table.add_row(vec![
                        Cell::new(&reminder.id).fg(Color::Red),
                        Cell::new(&reminder.value.what).fg(Color::Red),
                        Cell::new(reminder.value.when.format("%H:%M %Y-%m-%d")).fg(Color::Red),
                    ]);
                } else {
                    table.add_row(vec![
                        Cell::new(&reminder.id),
                        Cell::new(&reminder.value.what),
                        Cell::new(reminder.value.when.format("%H:%M %Y-%m-%d")),
                    ]);
                }
            }
            println!("{}", table);
            Ok(())
        },
        Action::Dismiss { id } => {
            let store = Store::init()?;
            store.dismiss_by_id(id)?;
            Ok(())
        }
    }
}
