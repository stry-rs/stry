use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::BackendEntry,
        models::{wiki::Page, Existing, Id, New},
    },
};

#[async_trait::async_trait]
impl BackendEntry<Page, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Page>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Page>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Page>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Page>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
