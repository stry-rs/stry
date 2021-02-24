use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::{BackendEntry, Existing, Id, New},
        models::Chapter,
    },
};

#[async_trait::async_trait]
impl BackendEntry<Chapter, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Chapter>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Chapter>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Chapter>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Chapter>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
