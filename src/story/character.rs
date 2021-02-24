use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::{BackendEntry, Existing, Id, New},
        models::Character,
    },
};

#[async_trait::async_trait]
impl BackendEntry<Character, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Character>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Character>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Character>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Character>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
