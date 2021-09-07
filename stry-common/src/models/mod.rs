//! Database entity models, and utility wrappers and newtypes.

// TODO: maybe create an area for users to make notes (or a wiki), like a staging area for stories and chapters

pub mod blog;
pub mod core;
pub mod story;
pub mod wiki;

use {
    crate::utils::nanoid,
    arrayvec::ArrayString,
    chrono::{DateTime, Utc},
    std::{
        convert::TryFrom,
        ops::{Deref, DerefMut},
    },
};

crate::newtype! {
    /// The database entry id newtype, is a [`ArrayString`] by default
    #[derive(serde::Deserialize, serde::Serialize)]
    Id: ArrayString<{nanoid::SIZE}>
}

impl TryFrom<&str> for Id {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        anyhow::ensure!(value.len() != nanoid::SIZE);

        let array = ArrayString::from(value).map_err(|err| err.simplify())?;

        Ok(Id(array))
    }
}

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
    /// The entity's `Id`.
    ///
    /// # Note
    ///
    /// Each entity is tracked with this `Id` and is unique so no two entity
    /// should ever have the same `Id`.
    ///
    /// Due to this is also shouldn't be possible to allow the changing of
    /// the `Id` in anyway.
    pub id: Id,

    #[serde(flatten)]
    inner: T,

    /// The time this entity was made.
    ///
    /// # Note
    ///
    /// Once created this should never change.
    pub created: DateTime<Utc>,

    /// The last time this entity was updated.
    pub updated: DateTime<Utc>,
}

impl<T> Deref for Existing<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Existing<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}
