use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{core::Part, Existing, Id, New},
        prelude::*,
    },
};

#[async_trait::async_trait]
impl BackendEntry<Part> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Part>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Part>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Part>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Part>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
