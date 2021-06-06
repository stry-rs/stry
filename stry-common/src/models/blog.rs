//! Entities for the blog 'module', for both user and site blog posts.

use crate::models::{
    core::{Comment, Part},
    Existing,
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Post {
    /// The parts that make up this blog post.
    pub parts: Vec<Existing<Part>>,

    /// Comment/replies to the main post itself.
    pub comments: Vec<Existing<Comment>>,
}
