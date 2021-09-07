use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{story::Story, Existing, Id, New},
        prelude::*,
    },
};

#[async_trait::async_trait]
impl BackendEntry<Story> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Story>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Story>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Story>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Story>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
