#[derive(Debug)]
pub enum Error {
    NoValidHomeDirectory,
    Sqlite(rusqlite::Error)
}

macro_rules! from {
    ($err:ty, $ty:ty, $variant:ident) => {
        impl From<$ty> for $err {
            fn from(e: $ty) -> $err {
                <$err>::$variant(e.into())
            }
        }
    }
}

from!(Error, rusqlite::Error, Sqlite);
