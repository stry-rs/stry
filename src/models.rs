use {chrono::{DateTime, Utc}, crate::backend::Id};

// TODO: flesh out story types, comments and their paragraph form
// TODO: change some items to allow references to themselves instead of whole data (ie chapters and series)

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

    pub user: User,
    pub content: String,
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

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Series {
    pub id: Id,

    pub name: String,
    pub summary: String,

    pub state: State,

    pub stories: Vec<Story>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Story {
    pub id: Id,

    pub name: String,
    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub chapters: i32,
    pub words: i32,

    pub authors: Vec<User>,
    pub commissioners: Vec<User>,
    pub dedicatees: Vec<User>,

    pub origins: Vec<Origin>,
    pub warnings: Vec<Warning>,
    pub pairings: Vec<Pairing>,
    pub characters: Vec<Character>,
    pub tags: Vec<Tag>,

    pub comments: Vec<Comment>,

    pub series: Option<Series>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Character {
    pub id: Id,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Origin {
    pub id: Id,

    pub name: String,

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

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub id: Id,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Warning {
    pub id: Id,

    pub name: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
//#endregion
