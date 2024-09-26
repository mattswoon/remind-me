use directories::ProjectDirs;
use std::path::{
    Path,
    PathBuf,
};
use rusqlite::Connection;
use crate::error::Error;
use crate::data::{
    Reminder,
    ReminderState,
    Id,
};


#[derive(Debug, Clone)]
pub struct Store {
    path: PathBuf
}

impl Store {
    pub fn init() -> Result<Store, Error> {
        let path = ProjectDirs::from("", "", "remind-me")
            .ok_or(Error::NoValidHomeDirectory)?
            .data_local_dir()
            .join("db.sqlite");
        if !path.exists() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            init_db(&path)?;
        }
        Ok(Store { path })
    }

    pub fn connection(&self) -> Result<Connection, Error> {
        Ok(Connection::open(&self.path)?)
    }

    pub fn insert_reminder(&self, reminder: &Reminder) -> Result<(), Error> {
        let conn = self.connection()?;
        conn.execute(
            "INSERT INTO reminders (what, when_, state) VALUES (?, ?, ?)",
            (&reminder.what, &reminder.when, reminder.state.as_i32())
        )?;
        // TODO: return inserted
        Ok(())
    }

    pub fn list_active(&self) -> Result<Vec<Id<Reminder>>, Error> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare("SELECT * FROM reminders WHERE state = ?")?;
        let records = stmt.query_and_then(
            [ReminderState::Active.as_i32()],
            |row| {
                let id = row.get(0)?;
                let what = row.get(1)?;
                let when = row.get(2)?;
                let state = row.get(3).map_err(Into::into).and_then(ReminderState::from_i32)?;
                Ok::<_, Error>(Id { id, value: Reminder { what, when, state }})
            }
        )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(records)
    }

    pub fn find_by_what(&self, pattern: &str) -> Result<Vec<Id<Reminder>>, Error> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare("SELECT * FROM reminders WHERE state = ? and what LIKE ?")?;
        let records = stmt.query_and_then(
            (ReminderState::Active.as_i32(), format!("%{}%", pattern)),
            |row| {
                let id = row.get(0)?;
                let what = row.get(1)?;
                let when = row.get(2)?;
                let state = row.get(3).map_err(Into::into).and_then(ReminderState::from_i32)?;
                Ok::<_, Error>(Id { id, value: Reminder { what, when, state }})
            }
        )?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(records)
    }

    pub fn dismiss_by_id(&self, id: i64) -> Result<(), Error> {
        let conn = self.connection()?;
        let mut stmt = conn.prepare("UPDATE reminders SET state = ? WHERE id = ?")?;
        stmt.execute((ReminderState::Dismissed.as_i32(), id))?;
        Ok(())
    }
}

fn init_db<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE reminders (id INTEGER PRIMARY KEY, what TEXT, when_ TEXT, state INTEGER);",
        ()
    )?;
    Ok(())
}
