#![allow(unused_variables)]

pub mod core;
pub mod blog;
pub mod story;

use {
    bb8::Pool,
    bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager},
    stry_common::backend::Backend,
    std::{error::Error, fmt::{self, Display, Formatter}},
};

pub struct PostgresBackend {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl Backend<PostgresBackendError> for PostgresBackend {}

#[derive(Debug)]
pub enum PostgresBackendError {}

impl Display for PostgresBackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Error for PostgresBackendError {}
