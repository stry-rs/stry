#![allow(unused_variables)]

pub mod blog;
pub mod core;
pub mod story;
pub mod wiki;

use {
    sqlx::{migrate::Migrator, postgres::PgConnectOptions, Pool, Postgres},
    stry_common::{
        backend::{Backend, BackendFactory},
        prelude::*,
        uri::Uri,
    },
};

static MIGRATOR: Migrator = sqlx::migrate!();

pub struct PostgresBackendFactory;

#[async_trait::async_trait]
impl BackendFactory for PostgresBackendFactory {
    type Backend = PostgresBackend;

    async fn create(&self, uri: Uri) -> Result<Self::Backend, Error> {
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
impl Backend for PostgresBackend {
    async fn migrate(&self) -> Result<(), Error> {
        MIGRATOR.run(&self.pool).await?;

        Ok(())
    }
}
