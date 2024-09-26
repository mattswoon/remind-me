use chrono::{DateTime, Local, Duration};
use std::str::FromStr;
use crate::error::{
    Error,
    WhenParseError,
};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Reminder {
    pub what: String,
    pub when: DateTime<Local>,
    pub state: ReminderState,
}

impl Reminder {
    pub fn new(what: String, when: DateTime<Local>) -> Self {
        Reminder {
            what,
            when,
            state: ReminderState::Active,
        }
    }

    pub fn is_overdue(&self, now: DateTime<Local>) -> bool {
        (now > self.when) & (self.state == ReminderState::Active)
    }

    pub fn time_remaining_str(&self, now: DateTime<Local>) -> String {
        let td = self.when - now;
        let hours = td.num_hours();
        let minutes = td.num_minutes() % 60;
        format!("{}:{}", hours, minutes)
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ReminderState {
    Active = 0,
    Dismissed = 1,
}

impl ReminderState {
    pub fn as_i32(&self) -> i32 {
        *self as i32 
    }

    pub fn from_i32(i: i32) -> Result<Self, Error> {
        match i {
            0 => Ok(ReminderState::Active),
            1 => Ok(ReminderState::Dismissed),
            _ => Err(Error::UnknownReminderState),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Id<A> {
    pub id: i64,
    pub value: A,
}

impl<A> Id<A> {
    pub fn map<B, F: Fn(A) -> B>(self, f: F) -> Id<B> {
        Id {
            id: self.id,
            value: f(self.value),
        }
    }

    pub fn value(&self) -> &'_ A {
        &self.value 
    }

    pub fn id(&self) -> i64 {
        self.id 
    }

    pub fn into_value(self) -> A {
        self.value 
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum When {
    InWeeks(u32),
    InDays(u32),
    InHours(u32),
    InMinutes(u32),
}

impl When {
    pub fn as_datetime(&self) -> DateTime<Local> {
        match self {
            When::InWeeks(x) => Local::now() + Duration::new((x * 60 * 60 * 24 * 7).into(), 0).unwrap(),
            When::InDays(x) => Local::now() + Duration::new((x * 60 * 60 * 24).into(), 0).unwrap(),
            When::InHours(x) => Local::now() + Duration::new((x * 60 * 60).into(), 0).unwrap(),
            When::InMinutes(x) => Local::now() + Duration::new((x * 60).into(), 0).unwrap(),
        }
    }
}

impl FromStr for When {
    type Err = WhenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = Regex::new(r"in (?<num>\d+) (?<unit>(day)|(week)|(hour)|(min)|(minute))s?").unwrap();
        let caps = r.captures(s).ok_or(WhenParseError::NoCaptures(s.to_string()))?;
        let num = caps.name("num")
            .ok_or(WhenParseError::NoNumber(s.to_string()))?
            .as_str()
            .parse::<u32>()?;
        let unit = caps.name("unit").ok_or(WhenParseError::NoUnit(s.to_string()))?
            .as_str();
        match unit {
            "day" => Ok(When::InDays(num)),
            "week" => Ok(When::InWeeks(num)),
            "hour" => Ok(When::InHours(num)),
            "min" | "minute" => Ok(When::InMinutes(num)),
            _ => Err(WhenParseError::NoUnit(s.to_string())),
        }
    }
}
