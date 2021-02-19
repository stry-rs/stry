//! Types for the configuration and implementation of database backends.

use {
    crate::models::{
        Chapter, Character, Comment, Origin, Pairing, Part, Post, Series, Story, Tag, User, Warning,
    },
    std::{collections::HashMap, error::Error, ops::Deref},
};

/// A wrapper type to indicate that a type has no backend id.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct New<T> {
    inner: T,
}

impl<T> Deref for New<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A wrapper type to indicate that a type has a backend id.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Existing<T> {
    pub id: Id,
    inner: T,
}

impl<T> Deref for Existing<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A supported database backend that depends on a series of entries.
///
/// Requires that the backend also implements [`BackendEntry`] for these types
/// (sharing the same error type):
///
///   - Core Types
///     - [`User`]
///     - [`Part`]
///     - [`Comment`]
///   - Blog Types
///     - [`Post`]
///   - Story Types
///     - [`Chapter`]
///     - [`Character`]
///     - [`Origin`]
///     - [`Pairing`]
///     - [`Series`]
///     - [`Story`]
///     - [`Tag`]
///     - [`Warning`]
#[cfg(feature = "with-backend")]
#[rustfmt::skip]
#[async_trait::async_trait]
pub trait Backend<Err>:
    BackendEntry<Chapter, Err>
    + BackendEntry<Character, Err>
    + BackendEntry<Comment, Err>
    + BackendEntry<Origin, Err>
    + BackendEntry<Pairing, Err>
    + BackendEntry<Part, Err>
    + BackendEntry<Post, Err>
    + BackendEntry<Series, Err>
    + BackendEntry<Story, Err>
    + BackendEntry<Tag, Err>
    + BackendEntry<User, Err>
    + BackendEntry<Warning, Err>
where
    Err: Error,
{
}

/// A database entry, or something that can be stored and retrieved from a database.
#[cfg(feature = "with-backend")]
#[async_trait::async_trait]
pub trait BackendEntry<Entry, Error> {
    /// Get an entity of type with a specific id.
    async fn get(&self, id: Id) -> Result<Existing<Entry>, Error>;
    /// Get all entities of type using offset cursor
    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Entry>>, Error>;

    /// Create a new entity of type.
    async fn create(&self, data: New<Entry>) -> Result<Id, Error>;
    /// Update an entity of type's data.
    async fn update(&self, data: Existing<Entry>) -> Result<(), Error>;
    /// Remove a entity of type.
    async fn remove(&self, id: Id) -> Result<(), Error>;
}

crate::newtype! {
    /// The database entry id newtype, is a `String` by default
    #[derive(serde::Deserialize, serde::Serialize)]
    Id: String
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum StorageType {
    File {
        location: String,
    },
    Parts {
        username: Option<String>,
        password: Option<String>,
        host: String,
        port: Option<String>,
        database: Option<String>,
        params: Option<HashMap<String, String>>,
    },
}

impl StorageType {
    pub fn is_file(&self) -> bool {
        matches!(self, StorageType::File { .. })
    }

    pub fn is_parts(&self) -> bool {
        matches!(self, StorageType::Parts { .. })
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum BackendType {
    Postgres,
    Sqlite,
}
