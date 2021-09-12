use {
    crate::PostgresBackend,
    stry_common::{
        backend::BackendEntry,
        error::NotFound,
        futures::utils::TryStreamExt as _,
        models::{
            core::{User, UserRecord},
            story::{Story, StoryRecord},
            Existing, Id, New,
        },
        prelude::*,
    },
};

#[stry_common::prelude::async_trait]
impl BackendEntry<Story> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Story>, Error> {
        let record = sqlx::query_file_as!(StoryRecord, "queries/story/get_story.sql", id.as_str())
            .fetch_optional(&self.pool)
            .await?;

        match record {
            Some(record) => {
                let mut story =
                    Story::new(record.name, record.summary, record.rating, record.state);

                {
                    let mut story_authors = sqlx::query_file_as!(
                        UserRecord,
                        "queries/story/get_story-authors.sql",
                        id.as_str()
                    )
                    .fetch(&self.pool);

                    while let Some(author) = story_authors.try_next().await? {
                        story.authors.push(Existing::new(
                            Id::try_from(author.id.as_str())?,
                            User::new_simple(author.name),
                            author.created,
                            author.updated,
                        ));
                    }
                }

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
