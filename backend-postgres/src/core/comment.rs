use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::BackendEntry,
        models::{core::Comment, Existing, Id, New},
    },
};

#[async_trait::async_trait]
impl BackendEntry<Comment, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Comment>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Comment>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Comment>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Comment>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
