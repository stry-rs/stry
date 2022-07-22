#![allow(unused_variables)]

use stry_common::{
    backend::{
        Backend, ChapterEntity, CharacterEntity, CommentEntity, OriginEntity, PairingEntity,
        PartEntity, SeriesEntity, StoryEntity, TagEntity, UserEntity, WarningEntity,
    },
    error::NotFound,
    futures::utils::TryStreamExt as _,
    loader::story::StoryLoaders,
    models::{
        core::{Tag, User},
        story::{
            Chapter, Character, Origin, Pairing, Series, Story, StoryRecord, StoryRecordId,
            TagLevel, Warning,
        },
        Existing, Id, New,
    },
    prelude::*,
    uri::Uri,
};

use sqlx::{migrate::Migrator, postgres::PgConnectOptions, Pool, Postgres};

static MIGRATOR: Migrator = sqlx::migrate!();

macro_rules! id_loader {
    ( $( [ $pool:expr, $loader:expr, $record_id:ident, $vec:expr, $query:expr, $($args:tt)* ], )+ ) => {{
        $(
            {
                async {
                    use stry_common::models::IdRecord;

                    let mut ids = sqlx::query_file_as!(IdRecord, $query, $($args)*)
                        .fetch($pool);

                    while let Some(record) = ids.try_next().await? {
                        let id = Id::try_from(record.id.as_str())?;

                        $vec.push($loader.load(id).await?);
                    }

                    Ok::<(), Error>(())
                }.instrument(trace_span!("story entities", id = ?$record_id, query_file = ?$query)).await?;
            }
        )+
    }};
}

macro_rules! id_level_loader {
    ( $( [ $pool:expr, $loader:expr, $record_id:ident, $vec:expr, $query:expr, $($args:tt)* ], )+ ) => {{
        $(
            {
                async {
                    use stry_common::models::story::IdLevelRecord;

                    let mut ids = sqlx::query_file_as!(IdLevelRecord, $query, $($args)*)
                        .fetch($pool);

                    while let Some(record) = ids.try_next().await? {
                        let id = Id::try_from(record.id.as_str())?;

                        let mut entity = $loader.load(id).await?;

                        entity.level = TagLevel::try_from(record.level.as_str())?;

                        $vec.push(entity);
                    }

                    Ok::<(), Error>(())
                }.instrument(trace_span!("story entities", id = ?$record_id, query_file = ?$query)).await?;
            }
        )+
    }};
}

#[derive(Clone)]
pub struct PostgresBackend {
    pool: Pool<Postgres>,
}

impl PostgresBackend {
    pub async fn new(uri: Uri) -> Result<Self, Error> {
        let config = {
            let mut config = PgConnectOptions::new();

            if let Some(username) = uri.username.as_ref() {
                config = config.username(username);

                if let Some(password) = uri.password.as_ref() {
                    config = config.password(password);
                }
            }

            for (host, port) in uri.hosts.iter().zip(uri.ports.iter()) {
                config = config.host(host);
                config = config.port(*port);
            }

            if let Some(database) = uri.database.as_ref() {
                config = config.database(database);
            }

            if let Some(options) = uri.options.as_ref() {
                for (key, value) in options.iter() {
                    match key.to_lowercase().as_str() {
                        "application_name" => {
                            config = config.application_name(value);
                        }
                        _ => continue,
                    }
                }
            }

            config
        };

        let pool = Pool::connect_with(config).await?;

        Ok(Self { pool })
    }
}

#[stry_common::prelude::async_trait]
impl Backend for PostgresBackend {
    async fn migrate(&self) -> Result<(), Error> {
        MIGRATOR.run(&self.pool).await?;

        Ok(())
    }
}

#[async_trait]
impl UserEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<User>, Error> {
        todo!()
    }

    #[instrument(skip(self, data), err)]
    async fn create(&self, data: New<User>) -> Result<Id, Error> {
        todo!()
    }
}

#[async_trait]
impl CommentEntity for PostgresBackend {}

#[async_trait]
impl PartEntity for PostgresBackend {}

#[async_trait]
impl TagEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<Tag>, Error> {
        todo!()
    }

    #[instrument(skip(self, cursor, limit), err)]
    async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Tag>>, Error> {
        todo!()
    }
}

#[async_trait]
impl ChapterEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<Chapter>, Error> {
        todo!()
    }

    #[instrument(skip(self, data), err)]
    async fn create(&self, data: New<Chapter>) -> Result<Id, Error> {
        todo!()
    }
}

#[async_trait]
impl OriginEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<Origin>, Error> {
        todo!()
    }

    #[instrument(skip(self, cursor, limit), err)]
    async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Origin>>, Error> {
        todo!()
    }
}

#[async_trait]
impl WarningEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<Warning>, Error> {
        todo!()
    }

    #[instrument(skip(self, cursor, limit), err)]
    async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Warning>>, Error> {
        todo!()
    }
}

#[async_trait]
impl PairingEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<Pairing>, Error> {
        todo!()
    }

    #[instrument(skip(self, cursor, limit), err)]
    async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Pairing>>, Error> {
        todo!()
    }
}

#[async_trait]
impl CharacterEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<Character>, Error> {
        todo!()
    }

    #[instrument(skip(self, cursor, limit), err)]
    async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Character>>, Error> {
        todo!()
    }
}

#[async_trait]
impl StoryEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<Story>, Error> {
        let loaders = StoryLoaders::new(Clone::clone(self));

        let record_id = id.as_str();

        let record = sqlx::query_file_as!(StoryRecord, "queries/story/get_story.sql", id.as_str())
            .fetch_optional(&self.pool)
            .instrument(trace_span!("fetch story with id", id = ?record_id))
            .await?;

        if let Some(record) = record {
            let mut story = Story::new(record.name, record.summary, record.rating, record.state);

            async {
                #[rustfmt::skip]
                id_loader![
                    [&self.pool, loaders.user, record_id, story.authors, "queries/story/get_story-user.sql", record_id, "author"],
                    [&self.pool, loaders.user, record_id, story.commissioners, "queries/story/get_story-user.sql", record_id, "commissioner"],
                    [&self.pool, loaders.user, record_id, story.dedicatees, "queries/story/get_story-user.sql", record_id, "dedicated"],
                    [&self.pool, loaders.tag, record_id, story.tags, "queries/story/get_story-tag.sql", record_id],
                ];

                #[rustfmt::skip]
                id_level_loader![
                    [&self.pool, loaders.origin, record_id, story.origins, "queries/story/get_story-origin.sql", record_id],
                    [&self.pool, loaders.warning, record_id, story.warnings, "queries/story/get_story-warning.sql", record_id],
                ];

                Ok::<(), Error>(())
            }.instrument(trace_span!("story load all entities", id = ?record_id)).await?;

            Ok(Existing::new(id, story, record.created, record.updated))
        } else {
            Err(NotFound.into())
        }
    }

    #[instrument(skip(self, cursor, limit), err)]
    async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Story>>, Error> {
        let loaders = StoryLoaders::new(Clone::clone(self));

        let records = if let Some(cursor) = cursor {
            sqlx::query_file_as!(
                StoryRecordId,
                "queries/story/all_stories--cursor.sql",
                cursor.as_str(),
                limit
            )
            .fetch_all(&self.pool)
            .instrument(trace_span!("fetch stories with cursor"))
            .await?
        } else {
            sqlx::query_file_as!(StoryRecordId, "queries/story/all_stories.sql", limit)
                .fetch_all(&self.pool)
                .instrument(trace_span!("fetch stories without cursor"))
                .await?
        };

        let mut stories = Vec::with_capacity(records.len());

        for record in records {
            let mut story = Story::new(record.name, record.summary, record.rating, record.state);

            let id = record.id.as_str();

            async {
                #[rustfmt::skip]
                id_loader![
                    [&self.pool, loaders.user, id, story.authors, "queries/story/get_story-user.sql", id, "author"],
                    [&self.pool, loaders.user, id, story.commissioners, "queries/story/get_story-user.sql", id, "commissioner"],
                    [&self.pool, loaders.user, id, story.dedicatees, "queries/story/get_story-user.sql", id, "dedicated"],
                    [&self.pool, loaders.tag, id, story.tags, "queries/story/get_story-tag.sql", id],
                ];

                #[rustfmt::skip]
                id_level_loader![
                    [&self.pool, loaders.origin, id, story.origins, "queries/story/get_story-origin.sql", id],
                    [&self.pool, loaders.warning, id, story.warnings, "queries/story/get_story-warning.sql", id],
                ];

                Ok::<(), Error>(())
            }.instrument(trace_span!("story entities", id = ?record.id)).await?;

            stories.push(Existing::new(
                Id::try_from(record.id.as_str())?,
                story,
                record.created,
                record.updated,
            ));
        }

        Ok(stories)
    }

    #[instrument(skip(self, data), err)]
    async fn create(&self, data: New<Story>) -> Result<Id, Error> {
        todo!()
    }
}

#[async_trait]
impl SeriesEntity for PostgresBackend {
    #[instrument(skip(self, id), err)]
    async fn get(&self, id: Id) -> Result<Existing<Series>, Error> {
        todo!()
    }

    #[instrument(skip(self, cursor, limit), err)]
    async fn all(&self, cursor: Option<Id>, limit: i64) -> Result<Vec<Existing<Series>>, Error> {
        todo!()
    }
}
