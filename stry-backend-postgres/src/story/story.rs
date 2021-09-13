use stry_common::{
    backend::BackendEntry,
    error::NotFound,
    futures::utils::TryStreamExt as _,
    loader::core::{TagLoader, UserLoader},
    models::{
        story::{Story, StoryRecord},
        Existing, Id, New,
    },
    prelude::{dataloader::Batcher, *},
};

use crate::PostgresBackend;

macro_rules! id_loader {
    ( $( [ $pool:expr, $loader:expr, $vec:expr, $query:expr, $($args:tt)* ], )+ ) => {{
        $(
            {
                use stry_common::models::IdRecord;

                let mut ids = sqlx::query_file_as!(IdRecord, $query, $($args)*)
                    .fetch($pool);

                while let Some(IdRecord { id }) = ids.try_next().await? {
                    let id = Id::try_from(id.as_str())?;

                    $vec.push($loader.load(id).await?);
                }
            }
        )+
    }};
}

#[derive(Clone)]
struct Loaders {
    user: Batcher<UserLoader<PostgresBackend>>,
    tag: Batcher<TagLoader<PostgresBackend>>,
}

impl Loaders {
    fn new(backend: PostgresBackend) -> Self {
        Self {
            user: UserLoader::new(backend.clone()),
            tag: TagLoader::new(backend),
        }
    }
}

async fn story_get(
    backend: PostgresBackend,
    loaders: Loaders,
    id: Id,
) -> Result<Existing<Story>, Error> {
    let record = sqlx::query_file_as!(StoryRecord, "queries/story/get_story.sql", id.as_str())
        .fetch_optional(&backend.pool)
        .await?;

    if let Some(record) = record {
        let mut story = Story::new(record.name, record.summary, record.rating, record.state);

        #[rustfmt::skip]
        id_loader![
            [&backend.pool, loaders.user, story.authors, "queries/story/get_story-user.sql", id.as_str(), "author"],
            [&backend.pool, loaders.user, story.commissioners, "queries/story/get_story-user.sql", id.as_str(), "commissioner"],
            [&backend.pool, loaders.user, story.dedicatees, "queries/story/get_story-user.sql", id.as_str(), "dedicated"],
        ];

        #[rustfmt::skip]
        id_loader![
            [&backend.pool, loaders.tag, story.tags, "queries/story/get_story-tag.sql", id.as_str()],
        ];

        Ok(Existing::new(id, story, record.created, record.updated))
    } else {
        Err(NotFound.into())
    }
}

#[stry_common::prelude::async_trait]
impl BackendEntry<Story> for PostgresBackend {
    async fn get(&self, id: Id) -> Result<Existing<Story>, Error> {
        let loaders = Loaders::new(Clone::clone(self));

        story_get(Clone::clone(self), loaders, id).await
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
