use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{story::Origin, Existing, Id, New},
        prelude::*,
    },
};

#[async_trait::async_trait]
impl BackendEntry<Origin> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Origin>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Origin>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Origin>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Origin>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
