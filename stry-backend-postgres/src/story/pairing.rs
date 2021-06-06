use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::BackendEntry,
        models::{story::Pairing, Existing, Id, New},
    },
};

#[async_trait::async_trait]
impl BackendEntry<Pairing, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Pairing>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Pairing>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Pairing>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Pairing>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
