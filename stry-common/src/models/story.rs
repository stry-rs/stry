//! Entities for the story 'module', everything tags unique to a story or
//! series is here.

use crate::{
    models::{
        core::{Comment, Part, Tag, User},
        Either, Existing, Id,
    },
    prelude::{members, Member, OffsetDateTime, TryFrom},
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Story {
    /// The title of the story, this is not unique as stories are tracked
    /// with its `Id`.
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
    #[serde(flatten)]
    pub chapters: Option<Either<Vec<Existing<Chapter>>, Vec<Id>>>,

    /// THe sum of all of the chapter's word counts.
    ///
    /// # Database Implementation
    ///
    /// Do not actually store the value in the database, let it be counted at run time.
    pub words: i32,

    pub comments: Vec<Existing<Comment>>,
}

impl Story {
    pub fn new(name: String, summary: String, rating: Rating, state: State) -> Self {
        Self {
            name,
            summary,

            rating,
            state,

            authors: Vec::new(),
            commissioners: Vec::new(),
            dedicatees: Vec::new(),

            origins: Vec::new(),
            warnings: Vec::new(),
            pairings: Vec::new(),
            characters: Vec::new(),
            tags: Vec::new(),

            series: None,

            chapters: None,
            words: 0,

            comments: Vec::new(),
        }
    }
}

impl Member for Story {
    type F = members::Authors;
    type T = Vec<Existing<User>>;

    fn get(&self) -> &Self::T {
        &self.authors
    }
}

/// A type used for database responses, maps to a row in the stories table.
pub struct StoryRecord {
    pub name: String,
    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub created: OffsetDateTime,
    pub updated: OffsetDateTime,
}

/// A variant of [`StoryRecord`] that includes the story id in [`String`] form.
pub struct StoryRecordId {
    pub id: String,

    pub name: String,
    pub summary: String,

    pub rating: Rating,
    pub state: State,

    pub created: OffsetDateTime,
    pub updated: OffsetDateTime,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Chapter {
    /// The title of the chapter.
    ///
    /// # Variant
    ///
    /// To display use the default title (`Chapter {number}`) if is [`None`].
    pub name: Option<String>,

    /// Marks the chapter was published allowing non authors to view it.
    ///
    /// # Note
    ///
    /// Even when `true` a chapter can still be edited.
    pub published: bool,

    /// The parts the make up the beginning author's note.
    ///
    /// # Note
    ///
    /// This does not contribute to word count.
    pub prefix: Vec<Existing<Part>>,

    /// THe parts of the actual chapter itself.
    pub main: Vec<Existing<Part>>,

    /// The parts the make up the ending author's note.
    ///
    /// # Note
    ///
    /// This does not contribute to word count.
    pub suffix: Vec<Existing<Part>>,

    /// Comments on the chapter itself not its parts.
    pub comments: Vec<Existing<Comment>>,

    /// The sum of all the [`main`] parts word count.
    ///
    /// # Database Implementation
    ///
    /// Do not actually store the value in the database, let it be counted at run time.
    ///
    /// [`main`]: #structfield.main
    pub words: i64,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Series {
    /// The title of the series, this is not unique as series are tracked
    /// with its `Id`.
    pub name: String,

    pub summary: String,

    pub state: State,

    /// # Variant
    ///
    /// Is `Left` when its used directly and is `Left` when its used indirectly (ie in another entity).
    #[serde(flatten)]
    pub stories: Either<Vec<Existing<Story>>, Vec<Id>>,
}

/// The type of tag, used for generic rendering.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum TagKind {
    Warning,
    Pairing,
    Character,
    General,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum TagLevel {
    Major,
    Minor,
}

impl TryFrom<&str> for TagLevel {
    type Error = crate::prelude::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "major" => Ok(TagLevel::Major),
            "minor" => Ok(TagLevel::Minor),
            value => crate::prelude::bail!("`{}` is not a valid tag level", value),
        }
    }
}

/// A generic trait for accepting tags from stories.
pub trait StoryTag {
    fn level(&self) -> TagLevel;
    fn kind(&self) -> TagKind;
}

impl<T: StoryTag> StoryTag for Existing<T> {
    fn level(&self) -> TagLevel {
        self.inner.level()
    }

    fn kind(&self) -> TagKind {
        self.inner.kind()
    }
}

impl StoryTag for Tag {
    fn level(&self) -> TagLevel {
        TagLevel::Major
    }

    fn kind(&self) -> TagKind {
        TagKind::General
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Origin {
    pub content: String,

    pub description: String,

    pub level: TagLevel,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Warning {
    pub content: String,

    pub description: String,

    pub level: TagLevel,
}

impl StoryTag for Warning {
    fn level(&self) -> TagLevel {
        self.level
    }

    fn kind(&self) -> TagKind {
        TagKind::Warning
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Character {
    pub content: String,

    pub description: String,

    pub level: TagLevel,
}

impl StoryTag for Character {
    fn level(&self) -> TagLevel {
        self.level
    }

    fn kind(&self) -> TagKind {
        TagKind::Character
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Pairing {
    pub hash: String,

    pub relationship: Relationship,

    pub characters: Vec<Existing<Character>>,

    pub level: TagLevel,
}

impl StoryTag for Pairing {
    fn level(&self) -> TagLevel {
        self.level
    }

    fn kind(&self) -> TagKind {
        TagKind::Pairing
    }
}

pub struct IdLevelRecord {
    pub id: String,
    pub level: String,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Relationship {
    Family,
    Friends,
    Romantic,
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "story_rating", rename_all = "snake_case"))]
pub enum Rating {
    Explicit,
    Mature,
    Teen,
    General,
}

/// The story's state.
///
/// # Note
///
/// Stories will automatically become [`State::Abandoned`] if they are left
/// without modification for some time.
///
/// To get this time check the `story-auto-abandon` setting key.
#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[cfg_attr(feature = "sqlx", sqlx(type_name = "story_state", rename_all = "snake_case"))]
pub enum State {
    Completed,
    InProgress,
    Hiatus,
    Abandoned,
}
