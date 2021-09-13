use stry_common::{
    backend::BackendEntry,
    error::NotFound,
    futures::utils::TryStreamExt as _,
    loader::{
        core::{TagLoader, UserLoader},
        story::{OriginLoader, WarningLoader},
    },
    models::{
        story::{Story, StoryRecord, TagLevel},
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

                while let Some(record) = ids.try_next().await? {
                    let id = Id::try_from(record.id.as_str())?;

                    $vec.push($loader.load(id).await?);
                }
            }
        )+
    }};
}

macro_rules! id_level_loader {
    ( $( [ $pool:expr, $loader:expr, $vec:expr, $query:expr, $($args:tt)* ], )+ ) => {{
        $(
            {
                use stry_common::models::story::IdLevelRecord;

                let mut ids = sqlx::query_file_as!(IdLevelRecord, $query, $($args)*)
                    .fetch($pool);

                while let Some(record) = ids.try_next().await? {
                    let id = Id::try_from(record.id.as_str())?;

                    let mut entity = $loader.load(id).await?;

                    entity.level = TagLevel::try_from(record.level.as_str())?;

                    $vec.push(entity);
                }
            }
        )+
    }};
}

#[derive(Clone)]
struct Loaders {
    user: Batcher<UserLoader<PostgresBackend>>,
    tag: Batcher<TagLoader<PostgresBackend>>,

    origin: Batcher<OriginLoader<PostgresBackend>>,
    warning: Batcher<WarningLoader<PostgresBackend>>,
}

impl Loaders {
    fn new(backend: PostgresBackend) -> Self {
        Self {
            user: UserLoader::new(backend.clone()),
            tag: TagLoader::new(backend.clone()),

            origin: OriginLoader::new(backend.clone()),
            warning: WarningLoader::new(backend),
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
            [&backend.pool, loaders.tag, story.tags, "queries/story/get_story-tag.sql", id.as_str()],
        ];

        #[rustfmt::skip]
        id_level_loader![
            [&backend.pool, loaders.origin, story.origins, "queries/story/get_story-origin.sql", id.as_str()],
            [&backend.pool, loaders.warning, story.warnings, "queries/story/get_story-warning.sql", id.as_str()],
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
