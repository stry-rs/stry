use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::BackendEntry,
        models::{story::Tag, Existing, Id, New},
    },
};

#[async_trait::async_trait]
impl BackendEntry<Tag, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Tag>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Tag>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Tag>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Tag>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
