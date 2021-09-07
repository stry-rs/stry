use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{core::User, Existing, Id, New},
        prelude::*,
    },
};

#[async_trait::async_trait]
impl BackendEntry<User> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<User>, Error> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<User>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<User>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<User>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
