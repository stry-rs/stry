use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::BackendEntry,
        models::{wiki::Category, Existing, Id, New},
    },
};

#[async_trait::async_trait]
impl BackendEntry<Category, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Category>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Category>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Category>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Category>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
