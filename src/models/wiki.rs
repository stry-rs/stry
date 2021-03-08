use crate::models::{Existing, core::Part};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Page {
    pub name: String,

    pub parts: Vec<Existing<Part>>,

    pub categories: Vec<Existing<Category>>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Category {
    pub content: String,

    pub description: String,
}
