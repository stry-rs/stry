use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{wiki::Page, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Page> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Page>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Option<Id>, limit: usize) -> Result<Vec<Existing<Page>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Page>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Page>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
