use {
    crate::models::{
        core::{Comment, Part},
        Id,
    },
    chrono::{DateTime, Utc},
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Post {
    pub id: Id,

    pub parts: Vec<Part>,
    pub comments: Vec<Comment>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
