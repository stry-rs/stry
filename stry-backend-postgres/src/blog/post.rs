use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{blog::Post, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Post> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Post>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Post>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Post>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Post>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
