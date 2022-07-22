use std::sync::Arc;

use crate::{
    models::{
        blog::Post,
        core::{Comment, Part, Tag, User},
        story::{Chapter, Character, Origin, Pairing, Series, Story, Warning},
        wiki::Page,
        Existing, Id, New,
    },
    prelude::*,
};

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
#[rustfmt::skip]
#[crate::prelude::async_trait]
pub trait Backend:
    // Core
    UserEntity
    + CommentEntity
    + PartEntity
    + TagEntity
    // Story
    + ChapterEntity
    + OriginEntity
    + WarningEntity
    + PairingEntity
    + CharacterEntity
    + StoryEntity
    + SeriesEntity
{
    /// Run any missing migration on the database backend.
    async fn migrate(&self) -> Result<(), Error>;
}

pub struct ArcBackend {
    inner: Arc<dyn Backend + Send + Sync + 'static>,
}

impl ArcBackend {
    pub fn new<B>(backend: B) -> Self
    where
        B: Backend + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(backend),
        }
    }
}

impl Clone for ArcBackend {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl std::ops::Deref for ArcBackend {
    type Target = Arc<dyn Backend + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// A macro that takes the input trait statement and
/// automatically created the [`ArcBackend`] implementation wrapper.
macro_rules! def {
    (
        $( #[$attrs:meta] )*
        pub trait $name:ident {
            $(
                $( #[$fn_attrs:meta] )*
                async fn $fn_name:ident(
                    &self, $( $fn_param_name:ident : $fn_param_typ:ty ),*
                ) -> $fn_ret:ty;
            )*
        }
    ) => {
        $( #[$attrs] )*
        #[crate::prelude::async_trait]
        pub trait $name {
            $(
                $( #[$fn_attrs] )*
                async fn $fn_name(
                    &self, $( $fn_param_name : $fn_param_typ ),*
                ) -> $fn_ret;
            )*
        }

        #[crate::prelude::async_trait]
        impl $name for ArcBackend {
            $(
                async fn $fn_name(
                    &self, $( $fn_param_name : $fn_param_typ ),*
                ) -> $fn_ret {
                    $name::$fn_name(&*self.inner, $( $fn_param_name ),*).await
                }
            )*
        }
    };
}

def! {
    pub trait UserEntity {
        async fn get(&self, id: Id) -> Result<Existing<User>, Error>;
        async fn create(&self, data: New<User>) -> Result<Id, Error>;
    }
}

def! {
    pub trait CommentEntity {}
}

def! {
    pub trait PartEntity {}
}

def! {
    pub trait TagEntity {
        async fn get(&self, id: Id) -> Result<Existing<Tag>, Error>;
        async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Tag>>, Error>;
    }
}

def! {
    pub trait ChapterEntity {
        async fn get(&self, id: Id) -> Result<Existing<Chapter>, Error>;
        async fn create(&self, data: New<Chapter>) -> Result<Id, Error>;
    }
}

def! {
    pub trait OriginEntity {
        async fn get(&self, id: Id) -> Result<Existing<Origin>, Error>;
        async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Origin>>, Error>;
    }
}

def! {
    pub trait WarningEntity {
        async fn get(&self, id: Id) -> Result<Existing<Warning>, Error>;
        async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Warning>>, Error>;
    }
}

def! {
    pub trait PairingEntity {
        async fn get(&self, id: Id) -> Result<Existing<Pairing>, Error>;
        async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Pairing>>, Error>;
    }
}

def! {
    pub trait CharacterEntity {
        async fn get(&self, id: Id) -> Result<Existing<Character>, Error>;
        async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Character>>, Error>;
    }
}

def! {
    pub trait StoryEntity {
        async fn get(&self, id: Id) -> Result<Existing<Story>, Error>;
        async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Story>>, Error>;
        async fn create(&self, data: New<Story>) -> Result<Id, Error>;
    }
}

def! {
    pub trait SeriesEntity {
        async fn get(&self, id: Id) -> Result<Existing<Series>, Error>;
        async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Series>>, Error>;
    }
}

// NOTE: old entry trait
// pub trait BackendEntry<Entry> {
//     /// Get an entity of type with a specific id.
//     async fn get(&self, id: Id) -> Result<Existing<Entry>, Error>;
//     /// Get all entities of type using offset cursor
//     async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Entry>>, Error>;

//     /// Create a new entity of type.
//     async fn create(&self, data: New<Entry>) -> Result<Id, Error>;
//     /// Update an entity of type's data.
//     async fn update(&self, data: Existing<Entry>) -> Result<(), Error>;
//     /// Remove a entity of type.
//     async fn remove(&self, id: Id) -> Result<(), Error>;
// }
