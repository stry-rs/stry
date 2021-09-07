use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{core::Comment, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Comment> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Comment>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Comment>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Comment>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Comment>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
