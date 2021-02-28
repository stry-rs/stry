use {
    crate::models::{
        core::{Comment, Part},
    },
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Post {
    pub parts: Vec<Part>,
    pub comments: Vec<Comment>,
}
