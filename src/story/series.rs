use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{models::Series, backend::{BackendEntry, Id, New, Existing}},
};

#[async_trait::async_trait]
impl BackendEntry<Series, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Series>, PostgresBackendError> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<Series>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Series>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Series>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
