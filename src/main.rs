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
use nom::{
    Parser as NomParser,
    IResult,
    multi::{
        many_till,
    },
    character::complete::{
        anychar,
        space1,
        space0,
        digit1,
    },
    combinator::{
        peek,
        recognize,
        map,
        map_res,
    },
    sequence::{
        separated_pair,
    },
    bytes::complete::{
        tag,
    },
    branch::{
        alt,
    },
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
    AddSmart {
        #[clap(trailing_var_arg=true)]
        args: Vec<String>,
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
                        Cell::new(&textwrap::fill(reminder.value.what.as_str(), 50)).fg(Color::Red),
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
        Action::AddSmart { args } => {
            let store = Store::init()?;
            let (what, when) = interpret(args.join(" "))?;
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

fn interpret(s: String) -> Result<(String, When), Error> {
    let (_, (what, when)) = parse_add(s.as_str()).map_err(|e| e.map(|e2| e2.cloned()))?;
    Ok((what, when))
}

fn parse_add(input: &str) -> IResult<&str, (String, When)> {
    let mut parser = separated_pair(
        map(take_until_parser(_in), |s: &str| s.to_string()),
        _in,
        _when
    );
    parser.parse(input)
}

pub fn take_until_parser<'a, P>(parser: P) -> impl NomParser<&'a str, Output=&'a str, Error=<P as NomParser<&'a str>>::Error>
where
    P: NomParser<&'a str>,
{
    recognize(many_till(anychar, peek(parser)))
}

pub fn _in(input: &str) -> IResult<&str, &str> {
    let mut parser = recognize((space1,tag("in"),space1));
    parser.parse(input)
}

pub fn _when(input: &str) -> IResult<&str, When> {
    let mut parser = map_res(
        (
            space0,
            map_res(digit1, |s: &str| s.parse::<u32>()),
            space0,
            alt((
                tag("hours"),
                tag("hour"),
                tag("days"),
                tag("day"),
                tag("minutes"),
                tag("minute"),
                tag("mins"),
                tag("min"),
                tag("weeks"),
                tag("week"),
            ))
        ),
        |(_, num, _, time)| match time {
            "hours" | "hour" => Ok(When::InHours(num)),
            "day" | "days" => Ok(When::InDays(num)),
            "mins" | "minutes" | "min" | "minute" => Ok(When::InMinutes(num)),
            "week" | "weeks" => Ok(When::InWeeks(num)),
            _ => Err("unknown time period"),
        }
    );
    parser.parse(input)
}
