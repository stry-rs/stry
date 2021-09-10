use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        models::{story::Character, Existing, Id, New},
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Character> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Character>, Error> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Option<Id>,
        limit: usize,
    ) -> Result<Vec<Existing<Character>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Character>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Character>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
