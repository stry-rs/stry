//! Types for the implementation of database backends.

use {
    crate::{
        models::{
            blog::Post,
            core::{Comment, Part, User},
            story::{Chapter, Character, Origin, Pairing, Series, Story, Tag, Warning},
            wiki::{Category, Page},
            Existing, Id, New,
        },
        uri::Uri,
    },
    std::error::Error,
};

/// A backend 'factory' that acts as a constructor for backends.
#[cfg(feature = "with-backend")]
#[async_trait::async_trait]
pub trait BackendFactory {
    type Error: Error;
    type Backend: Backend<Self::Error>;

    /// Essentially a `new` function.
    async fn create(&self, config: Uri) -> Result<Self::Backend, Self::Error>;
}

/// A supported database backend that depends on a series of entries.
///
/// Requires that the backend also implements [`BackendEntry`] for these types
/// (sharing the same error type):
///
///   - Core Types
///     - [`Comment`]
///     - [`Part`]
///     - [`User`]
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
///   - Wiki Types
///     - [`Category`]
///     - [`Page`]
#[cfg(feature = "with-backend")]
#[rustfmt::skip]
#[async_trait::async_trait]
pub trait Backend<Err>:
    // Core
    BackendEntry<User, Err>
    + BackendEntry<Comment, Err>
    + BackendEntry<Part, Err>
    // Blog
    + BackendEntry<Post, Err>
    // Story
    + BackendEntry<Chapter, Err>
    + BackendEntry<Origin, Err>
    + BackendEntry<Warning, Err>
    + BackendEntry<Pairing, Err>
    + BackendEntry<Character, Err>
    + BackendEntry<Tag, Err>
    + BackendEntry<Story, Err>
    + BackendEntry<Series, Err>
    // Wiki
    + BackendEntry<Category, Err>
    + BackendEntry<Page, Err>
where
    Err: Error,
{
    /// Run any missing migration on the database backend.
    async fn migrate(&self) -> Result<(), Err>;
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
