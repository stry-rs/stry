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

    pub authors: Vec<Existing<User>>,
    pub commissioners: Vec<Existing<User>>,
    pub dedicatees: Vec<Existing<User>>,

    pub origins: Vec<Existing<Origin>>,
    pub warnings: Vec<Existing<Warning>>,
    pub pairings: Vec<Existing<Pairing>>,
    pub characters: Vec<Existing<Character>>,
    pub tags: Vec<Existing<Tag>>,

    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub series: Option<Existing<Series>>,

    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub chapters: Option<Vec<Existing<Chapter>>>,
    pub words: i32,

    pub comments: Vec<Existing<Comment>>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub name: String,

    pub prefix: Vec<Existing<Part>>,
    pub main: Vec<Existing<Part>>,
    pub suffix: Vec<Existing<Part>>,

    pub comments: Vec<Existing<Comment>>,

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
    // TODO: switch to an enum for more possible states
    pub platonic: bool,

    pub characters: Vec<Existing<Character>>,
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
