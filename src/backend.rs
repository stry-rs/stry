use {
    crate::models,
    std::{collections::HashMap, error::Error},
};

/// A supported database backend that depends on a series of entries.
#[cfg(feature = "with-backend")]
#[async_trait::async_trait]
pub trait Backend<Err>: BackendEntry<Entry = models::Tag, Error = Err>
where
    Err: Error,
{}

/// A database entry, or something that can be stored and retrieved from a database.
#[cfg(feature = "with-backend")]
#[async_trait::async_trait]
pub trait BackendEntry {
    type Entry;
    type Error: Error;

    async fn insert(&self, data: Self::Entry) -> Result<Self::Entry, Self::Error>;
    async fn select(&self, id: Id) -> Result<Self::Entry, Self::Error>;
    async fn update(&self, data: Self::Entry) -> Result<Self::Entry, Self::Error>;
    async fn delete(&self, id: Id) -> Result<Self::Entry, Self::Error>;
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
