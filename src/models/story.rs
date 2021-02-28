use {
    crate::models::{
        core::{Comment, Part, User},
        Existing, Id,
    },
    either::Either,
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Story {
    pub name: String,
    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub authors: Vec<User>,
    pub commissioners: Vec<User>,
    pub dedicatees: Vec<User>,

    pub origins: Vec<Origin>,
    pub warnings: Vec<Warning>,
    pub pairings: Vec<Pairing>,
    pub characters: Vec<Character>,
    pub tags: Vec<Tag>,

    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub series: Option<Series>,

    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub chapters: Option<Vec<Chapter>>,
    pub words: i32,

    pub comments: Vec<Comment>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub name: String,

    pub prefix: Vec<Part>,
    pub main: Vec<Part>,
    pub suffix: Vec<Part>,

    pub comments: Vec<Comment>,

    pub words: i64,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Series {
    pub name: String,
    pub summary: String,

    pub state: State,

    /// # Variant
    ///
    /// Is `Left` when its used directly and is `Left` when its used indirectly (ie in another entity).
    pub stories: Either<Vec<Existing<Story>>, Vec<Id>>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Character {
    pub content: String,
    pub description: String,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Origin {
    pub content: String,
    pub description: String,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Pairing {
    pub hash: String,
    pub platonic: bool,

    pub characters: Vec<Character>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub content: String,
    pub description: String,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Warning {
    pub content: String,
    pub description: String,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Rating {
    Explicit,
    Mature,
    Teen,
    General,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum State {
    Completed,
    InProgress,
    Hiatus,
    Abandoned,
}
