//! Site wide and user wiki, allowing for history and notes on anything needed.

use {
    crate::models::{core::{Part, Tag}, Existing},
    chrono::{DateTime, Utc},
};

/// A wiki page and its related data.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Page {
    pub name: String,

    pub parts: Vec<Existing<Part>>,

    pub tags: Vec<Existing<Tag>>,
}

/// A point in history for a page.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PageRevision {
    pub name: String,

    pub parts: Vec<Existing<Part>>,

    pub tags: Vec<Existing<Tag>>,

    pub modified: DateTime<Utc>,
}
