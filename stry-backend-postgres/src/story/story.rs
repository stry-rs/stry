use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::BackendEntry,
        models::{story::Story, Existing, Id, New},
    },
};

#[async_trait::async_trait]
impl BackendEntry<Story, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Story>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Story>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Story>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Story>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
