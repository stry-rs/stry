use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{story::Warning, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Warning> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Warning>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Warning>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Warning>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Warning>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
