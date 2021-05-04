use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::BackendEntry,
        models::{story::Origin, Existing, Id, New},
    },
};

#[async_trait::async_trait]
impl BackendEntry<Origin, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Origin>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Origin>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Origin>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Origin>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
