#![warn(rust_2018_idioms)]

use std::fmt;

pub mod utils;

pub mod anulap;
pub mod backend;
pub mod config;
pub mod error;
pub mod models;
pub mod uri;

#[cfg(feature = "with-nanoid")]
pub use crate::utils::nanoid::nanoid;

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum LibraryDetails {
    Curl {
        // TODO: add feature members
        number: &'static str,
        version: String,
    },
    OpenSSL {
        version: &'static str,
    },
    SQLite {
        version: &'static str,
    },
}

impl fmt::Display for LibraryDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibraryDetails::Curl { number, version } => {
                writeln!(f, "cURL {} ({})", version, number)
            }
            LibraryDetails::OpenSSL { version } => writeln!(f, "{}", version),
            LibraryDetails::SQLite { version } => writeln!(f, "SQLite {}", version),
        }
    }
}
