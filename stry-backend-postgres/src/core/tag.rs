use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{core::Tag, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Tag> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Tag>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Option<Id>, limit: usize) -> Result<Vec<Existing<Tag>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Tag>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Tag>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
