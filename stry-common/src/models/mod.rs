//! Database entity models, and utility wrappers and newtypes.

// TODO: maybe create an area for users to make notes (or a wiki), like a staging area for stories and chapters

pub mod blog;
pub mod core;
pub mod story;
pub mod wiki;

use std::{
    convert::TryFrom,
    ops::{Deref, DerefMut},
};

use arrayvec::ArrayString;

use crate::{prelude::OffsetDateTime, utils::nanoid};

fenn::newtype! {
    /// The database entry id newtype, is a [`ArrayString`] by default
    #[derive(serde::Deserialize, serde::Serialize)]
    Id: ArrayString<{nanoid::ID_SIZE}>
}

impl Id {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Converts a [`str`] into an [`Id`] ignoring any errors.
    ///
    /// # Safety
    ///
    /// As this is made to handle responses from the database,
    /// there are no safety checks involved for now.
    pub unsafe fn from_str_unchecked(id: &str) -> Self {
        Self(ArrayString::from(id).unwrap_unchecked())
    }
}

impl TryFrom<&str> for Id {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            value.len() == nanoid::ID_SIZE,
            "value size is `{}`",
            value.len()
        );

        let array = ArrayString::from(value).map_err(|err| err.simplify())?;

        Ok(Self(array))
    }
}

impl TryFrom<String> for Id {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

fenn::newtype! {
    /// The user session newtype, is a [`ArrayString`] by default
    #[derive(serde::Deserialize, serde::Serialize)]
    Session: ArrayString<{nanoid::SESSION_SIZE}>
}

impl Session {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<&str> for Session {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        anyhow::ensure!(
            value.len() == nanoid::SESSION_SIZE,
            "value size is `{}`",
            value.len()
        );

        let array = ArrayString::from(value).map_err(|err| err.simplify())?;

        Ok(Self(array))
    }
}

impl TryFrom<String> for Session {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

pub struct IdRecord {
    pub id: String,
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

impl<T> From<T> for New<T> {
    fn from(t: T) -> Self {
        New { inner: t }
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
    pub created: OffsetDateTime,

    /// The last time this entity was updated.
    pub updated: OffsetDateTime,
}

impl<T> Existing<T> {
    pub fn new(id: Id, data: T, created: OffsetDateTime, updated: OffsetDateTime) -> Self {
        Self {
            id,
            inner: data,
            created,
            updated,
        }
    }
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
