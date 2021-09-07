use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{story::Pairing, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Pairing> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Pairing>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Pairing>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Pairing>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Pairing>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
