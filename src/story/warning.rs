use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::{BackendEntry},
        models::{Existing, Id, New, story::Warning},
    },
};

#[async_trait::async_trait]
impl BackendEntry<Warning, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Warning>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Warning>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Warning>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Warning>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
