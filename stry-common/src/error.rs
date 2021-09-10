use std::{error::Error, fmt};

#[derive(Debug)]
pub struct NotFound;

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not found")
    }
}

impl Error for NotFound {}
