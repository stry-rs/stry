use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{models::User, backend::{BackendEntry, Id, New, Existing}},
};

#[async_trait::async_trait]
impl BackendEntry<User, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<User>, PostgresBackendError> {
        todo!()
    }

    async fn all(&self, cursor: Id, limit: usize) -> Result<Vec<Existing<User>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<User>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<User>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
