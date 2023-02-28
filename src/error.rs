use core::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Tera(tera::Error),
    Toml(toml::de::Error),
    Notify(notify::Error),
    Favia(String),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Error::Tera(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

impl From<walkdir::Error> for Error {
    fn from(err: walkdir::Error) -> Self {
        Error::Io(err.into_io_error().unwrap())
    }
}

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Self {
        Error::Notify(err)
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => match err.kind() {
                io::ErrorKind::NotFound => write!(f, "does not exist: {err}")?,
                _ => todo!(),
            },
            Error::Tera(_) => todo!(),
            Error::Toml(_) => todo!(),
            Error::Notify(err) => write!(f, "{err}")?,
            Error::Favia(err) => write!(f, "{err}")?,
        }

        Ok(())
    }
}
