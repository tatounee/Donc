
use std::fmt;


pub enum Error {
    NoClanTagProvided,
    InexistentClan,
    NoApiTokenProvided,
    Io(std::io::Error),
    Dotenv(dotenv::Error),
    Reqwest(reqwest::Error),
    Csv(csv::Error)
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoClanTagProvided => f.write_str("No clan tag provided"),
            Self::InexistentClan => f.write_str("Inexistent clan"),
            Self::NoApiTokenProvided => f.write_str("No coc's api token provided"),
            Self::Io(e) => e.fmt(f),
            Self::Dotenv(e) => e.fmt(f),
            Self::Reqwest(e) => {
                if e.is_decode() {
                    Self::InexistentClan.fmt(f)
                } else {
                    e.fmt(f)
                }
            },
            Self::Csv(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<dotenv::Error> for Error {
    fn from(e: dotenv::Error) -> Self {
        Self::Dotenv(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Self::Csv(e)
    }
}