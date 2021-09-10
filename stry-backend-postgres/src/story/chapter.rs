use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{story::Chapter, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Chapter> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Chapter>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Option<Id>, limit: usize) -> Result<Vec<Existing<Chapter>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Chapter>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Chapter>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
