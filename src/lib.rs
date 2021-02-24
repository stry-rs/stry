#![allow(unused_variables)]

pub mod blog;
pub mod core;
pub mod story;

use {
    bb8::Pool,
    bb8_postgres::{
        tokio_postgres::{
            self,
            config::{ChannelBinding, TargetSessionAttrs},
            Config, NoTls,
        },
        PostgresConnectionManager,
    },
    std::{
        error::Error,
        fmt::{self, Display, Formatter},
        num::ParseIntError,
        time::Duration,
    },
    stry_common::{
        backend::{Backend, BackendFactory},
        uri::Uri,
    },
};

pub struct PostgresBackendFactory;

#[async_trait::async_trait]
impl BackendFactory for PostgresBackendFactory {
    type Error = PostgresBackendError;
    type Backend = PostgresBackend;

    async fn create(&self, uri: Uri) -> Result<Self::Backend, Self::Error> {
        let config = {
            let mut config = Config::new();

            if let Some(username) = uri.username.as_ref() {
                config.user(username);

                if let Some(password) = uri.password.as_ref() {
                    config.password(password);
                }
            }

            for (host, port) in uri.hosts.iter().zip(uri.ports.iter()) {
                config.host(host);
                config.port(*port);
            }

            if let Some(database) = uri.database.as_ref() {
                config.dbname(database);
            }

            if let Some(options) = uri.options.as_ref() {
                for (key, value) in options.iter() {
                    match key.to_lowercase().as_str() {
                        "application_name" => {
                            config.application_name(value);
                        }
                        "connect_timeout" => {
                            let value = value.parse::<i64>()?;

                            if value > 0 {
                                config.connect_timeout(Duration::from_secs(value as u64));
                            }
                        }
                        "keepalives" => {
                            config.keepalives(value.parse::<u64>()? != 0);
                        }
                        "keepalives_idle" => {
                            let value = value.parse::<i64>()?;

                            if value > 0 {
                                config.keepalives_idle(Duration::from_secs(value as u64));
                            }
                        }
                        "target_session_attrs" => {
                            let target_session_attrs = match value.to_lowercase().as_str() {
                                "any" => TargetSessionAttrs::Any,
                                "read-write" => TargetSessionAttrs::ReadWrite,
                                _ => continue,
                            };

                            config.target_session_attrs(target_session_attrs);
                        }
                        "channel_binding" => {
                            let channel_binding = match value.to_lowercase().as_str() {
                                "disable" => ChannelBinding::Disable,
                                "prefer" => ChannelBinding::Prefer,
                                "require" => ChannelBinding::Require,
                                _ => continue,
                            };

                            config.channel_binding(channel_binding);
                        }
                        _ => continue,
                    }
                }
            }

            config
        };

        let manager = PostgresConnectionManager::new(config, NoTls);

        let pool = Pool::builder().build(manager).await?;

        Ok(PostgresBackend { pool })
    }
}

pub struct PostgresBackend {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

#[async_trait::async_trait]
impl Backend<PostgresBackendError> for PostgresBackend {
    async fn migrate(&self) -> Result<(), PostgresBackendError> {
        let mut conn = self.pool.get().await?;
        let conn_inner = &mut *conn;

        embedded::migrations::runner().run_async(conn_inner).await?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum PostgresBackendError {
    BB8 {
        err: bb8::RunError<bb8_postgres::tokio_postgres::Error>,
    },
    ConfigParse {
        err: ParseIntError,
    },
    Postgres {
        err: tokio_postgres::Error,
    },
    Refinery {
        err: refinery::Error,
    },
}

impl Display for PostgresBackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PostgresBackendError::BB8 { err } => match err {
                bb8::RunError::User(err) => write!(f, "bb8 postgres error: {}", err)?,
                bb8::RunError::TimedOut => write!(f, "bb8 timeout error")?,
            },
            PostgresBackendError::ConfigParse { err } => {
                write!(f, "postgres uri parameter value parse error: {}", err)?;
            }
            PostgresBackendError::Postgres { err } => {
                write!(f, "postgres error: {}", err)?;
            }
            PostgresBackendError::Refinery { err } => {
                write!(f, "postgres refinery error: {}", err)?
            }
        }

        Ok(())
    }
}

impl Error for PostgresBackendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            PostgresBackendError::BB8 { err } => Some(err),
            PostgresBackendError::ConfigParse { err } => Some(err),
            PostgresBackendError::Postgres { err } => Some(err),
            PostgresBackendError::Refinery { err } => Some(err),
        }
    }
}

impl From<bb8::RunError<bb8_postgres::tokio_postgres::Error>> for PostgresBackendError {
    fn from(err: bb8::RunError<bb8_postgres::tokio_postgres::Error>) -> Self {
        Self::BB8 { err }
    }
}

impl From<ParseIntError> for PostgresBackendError {
    fn from(err: ParseIntError) -> Self {
        Self::ConfigParse { err }
    }
}

impl From<tokio_postgres::Error> for PostgresBackendError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::Postgres { err }
    }
}

impl From<refinery::Error> for PostgresBackendError {
    fn from(err: refinery::Error) -> Self {
        Self::Refinery { err }
    }
}

mod embedded {
    refinery::embed_migrations!("./migrations");
}
