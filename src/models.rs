use {
    crate::backend::Id,
    chrono::{DateTime, Utc},
    either::Either,
};

//#region [rgba(186,225,255,0.05)] core types
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub key: String,
    pub value: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct User {
    pub id: Id,

    pub name: String,
    pub bio: String,

    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub stories: Option<Vec<Story>>,
    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub posts: Option<Vec<Post>>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Part {
    pub id: Id,

    pub kind: PartKind,
    pub comments: Vec<Comment>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum PartKind {
    Image { url: String, alt: Option<String>, },
    Text { content: String, },
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Comment {
    pub id: Id,

    pub author: User,
    pub main: Vec<Part>,
    pub children: Vec<Comment>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
//#endregion

//#region [rgba(255,179,186,0.05)] blog types
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
//#endregion

//#region[rgba(186,255,201,0.05)] story types
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Story {
    pub id: Id,

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

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    pub id: Id,

    pub name: String,

    pub prefix: Vec<Part>,
    pub main: Vec<Part>,
    pub suffix: Vec<Part>,

    pub comments: Vec<Comment>,

    pub words: i64,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Series {
    pub id: Id,

    pub name: String,
    pub summary: String,

    pub state: State,

    /// # Variant
    ///
    /// Is `Left` when its used directly and is `Left` when its used indirectly (ie in another entity).
    pub stories: Either<Vec<Story>, Vec<Id>>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Character {
    pub id: Id,

    pub content: String,
    pub description: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Origin {
    pub id: Id,

    pub content: String,
    pub description: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Pairing {
    pub id: Id,

    pub hash: String,
    pub platonic: bool,

    pub characters: Vec<Character>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub id: Id,

    pub content: String,
    pub description: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Warning {
    pub id: Id,

    pub content: String,
    pub description: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
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
//#endregion
