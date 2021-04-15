//! Site wide and user wiki, allowing for history and notes on anything needed.

use {crate::models::{Existing, core::Part}, chrono::{DateTime, Utc}};

/// A wiki page and its related data.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Page {
    pub name: String,

    pub parts: Vec<Existing<Part>>,

    pub categories: Vec<Existing<Category>>,
}

/// A point in history for a page.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PageRevision {
    pub name: String,

    pub parts: Vec<Existing<Part>>,

    pub categories: Vec<Existing<Category>>,

    pub modified: DateTime<Utc>,
}

/// A category that can be used to separate pages into groups.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Category {
    pub content: String,

    pub description: String,
}
