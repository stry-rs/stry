use crate::models::{
    core::{Comment, Part},
    Existing,
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Post {
    pub parts: Vec<Existing<Part>>,
    pub comments: Vec<Existing<Comment>>,
}
