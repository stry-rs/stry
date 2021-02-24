use {
    crate::{PostgresBackend, PostgresBackendError},
    stry_common::{
        backend::{BackendEntry, Existing, Id, New},
        models::Part,
    },
};

#[async_trait::async_trait]
impl BackendEntry<Part, PostgresBackendError> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Part>, PostgresBackendError> {
        todo!()
    }

    async fn all(
        &self,
        cursor: Id,
        limit: usize,
    ) -> Result<Vec<Existing<Part>>, PostgresBackendError> {
        todo!()
    }

    async fn create(&self, data: New<Part>) -> Result<Id, PostgresBackendError> {
        todo!()
    }

    async fn update(&self, data: Existing<Part>) -> Result<(), PostgresBackendError> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), PostgresBackendError> {
        todo!()
    }
}
