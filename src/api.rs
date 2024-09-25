use directories::ProjectDirs;
use std::path::{
    Path,
    PathBuf,
};
use rusqlite::Connection;
use crate::error::Error;
use crate::data::{
    Reminder,
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
            "INSERT INTO reminders (what, when, state) VALUES (?, ?, ?)",
            (&reminder.what, &reminder.when, reminder.state.as_i32())
        )?;
        // TODO: return inserted
        Ok(())
    }
}

fn init_db<P: AsRef<Path>>(path: P) -> Result<(), Error> {
    let conn = Connection::open(path)?;
    conn.execute(
        "CREATE TABLE reminders (id INTEGER PRIMARY KEY, what TEXT, when TEXT, state INTEGER)",
        ()
    )?;
    Ok(())
}
