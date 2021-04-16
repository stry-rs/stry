#![allow(unused_variables)]

pub mod blog;
pub mod core;
pub mod story;
pub mod wiki;

use {
    sqlx::{migrate::Migrator, postgres::PgConnectOptions, Pool, Postgres},
    std::{
        error::Error,
        fmt::{self, Display, Formatter},
        num::ParseIntError,
    },
    stry_common::{
        backend::{Backend, BackendFactory},
        uri::Uri,
    },
};

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct PostgresBackendFactory;

#[async_trait::async_trait]
impl BackendFactory for PostgresBackendFactory {
    type Error = PostgresBackendError;
    type Backend = PostgresBackend;

    async fn create(&self, uri: Uri) -> Result<Self::Backend, Self::Error> {
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

        Ok(PostgresBackend { pool })
    }
}

pub struct PostgresBackend {
    pool: Pool<Postgres>,
}

#[async_trait::async_trait]
impl Backend<PostgresBackendError> for PostgresBackend {
    async fn migrate(&self) -> Result<(), PostgresBackendError> {
        MIGRATOR.run(&self.pool).await?;

        Ok(())
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum PostgresBackendError {
    ConfigParse { err: ParseIntError },
    SQLx { err: sqlx::Error },
    SQLxMigration { err: sqlx::migrate::MigrateError },
}

impl Display for PostgresBackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PostgresBackendError::ConfigParse { err } => {
                write!(f, "postgres uri parameter value parse error: {}", err)?;
            }
            PostgresBackendError::SQLx { err } => {
                write!(f, "sqlx error: {}", err)?;
            }
            PostgresBackendError::SQLxMigration { err } => {
                write!(f, "sqlx migration error: {}", err)?;
            }
        }

        Ok(())
    }
}

impl Error for PostgresBackendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PostgresBackendError::ConfigParse { err } => Some(err),
            PostgresBackendError::SQLx { err } => Some(err),
            PostgresBackendError::SQLxMigration { err } => Some(err),
        }
    }
}

impl From<ParseIntError> for PostgresBackendError {
    fn from(err: ParseIntError) -> Self {
        Self::ConfigParse { err }
    }
}

impl From<sqlx::Error> for PostgresBackendError {
    fn from(err: sqlx::Error) -> Self {
        Self::SQLx { err }
    }
}

impl From<sqlx::migrate::MigrateError> for PostgresBackendError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        Self::SQLxMigration { err }
    }
}
