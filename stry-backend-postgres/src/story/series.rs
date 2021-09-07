use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{story::Series, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Series> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Series>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Series>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Series>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Series>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
