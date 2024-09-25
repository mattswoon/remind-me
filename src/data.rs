use chrono::{DateTime, Local};

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

    pub fn is_overdue(&self, time: DateTime<Local>) -> bool {
        (self.when > time) & (self.state == ReminderState::Active)
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
