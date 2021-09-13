//! Types for the implementation of database backends.

pub mod arc;

use crate::{
    models::{
        blog::Post,
        core::{Comment, Part, Tag, User},
        story::{Chapter, Character, Origin, Pairing, Series, Story, Warning},
        wiki::Page,
        Existing, Id, New,
    },
    prelude::*,
    uri::Uri,
};

/// A backend 'factory' that acts as a constructor for backends.
#[crate::prelude::async_trait]
pub trait BackendFactory {
    type Backend: Backend;

    /// Essentially a `new` function.
    async fn create(&self, config: Uri) -> Result<Self::Backend, Error>;
}

/// A supported database backend that depends on a series of entries.
///
/// Requires that the backend also implements [`BackendEntry`] for these types
/// (sharing the same error type):
///
///   - Core Types
///     - [`Comment`]
///     - [`Part`]
///     - [`Tag`]
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
///     - [`Warning`]
///   - Wiki Types
///     - [`Page`]
///     - [`PageRevision`]
#[rustfmt::skip]
#[crate::prelude::async_trait]
pub trait Backend:
    // Core
    BackendEntry<User>
    + BackendEntry<Comment>
    + BackendEntry<Part>
    + BackendEntry<Tag>
    // Blog
    + BackendEntry<Post>
    // Story
    + BackendEntry<Chapter>
    + BackendEntry<Origin>
    + BackendEntry<Warning>
    + BackendEntry<Pairing>
    + BackendEntry<Character>
    + BackendEntry<Story>
    + BackendEntry<Series>
    // Wiki
    + BackendEntry<Page>
{
    /// Run any missing migration on the database backend.
    async fn migrate(&self) -> Result<(), Error>;
}

/// A database entry, or something that can be stored and retrieved from a database.
#[crate::prelude::async_trait]
pub trait BackendEntry<Entry> {
    /// Get an entity of type with a specific id.
    async fn get(&self, id: Id) -> Result<Existing<Entry>, Error>;
    /// Get all entities of type using offset cursor
    async fn all(&self, cursor: Option<Id>, limit: usize) -> Result<Vec<Existing<Entry>>, Error>;

    /// Create a new entity of type.
    async fn create(&self, data: New<Entry>) -> Result<Id, Error>;
    /// Update an entity of type's data.
    async fn update(&self, data: Existing<Entry>) -> Result<(), Error>;
    /// Remove a entity of type.
    async fn remove(&self, id: Id) -> Result<(), Error>;
}
