use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        error::NotFound,
        models::{
            story::{Story, StoryRecord},
            Existing, Id, New,
        },
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Story> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Story>, Error> {
        let record = sqlx::query_as!(StoryRecord, r#"SELECT name, summary, rating as "rating: _", state as "state: _", created as "created: _", updated as "updated: _" FROM story_story WHERE id = $1"#, id.as_str())
            .fetch_optional(&self.pool)
            .await?;

        match record {
            Some(record) => {
                let story = Story::new(record.name, record.summary, record.rating, record.state);

                Ok(Existing::new(id, story, record.created, record.updated))
            }
            None => Err(NotFound.into()),
        }
    }

    async fn all(&self, cursor: Option<Id>, limit: usize) -> Result<Vec<Existing<Story>>, Error> {
        todo!()
    }

    async fn create(&self, data: New<Story>) -> Result<Id, Error> {
        todo!()
    }

    async fn update(&self, data: Existing<Story>) -> Result<(), Error> {
        todo!()
    }

    async fn remove(&self, id: Id) -> Result<(), Error> {
        todo!()
    }
}
