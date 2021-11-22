//! Init config information, everything else is handled though the frontend.

use std::sync::Arc;

use crate::layered::{Anulap, Initialize};

/// The default secret key.
///
/// Used to check if the server was supplied with a new key.
pub static DEFAULT_SECRET: &str = "&E)H@McQfTjWnZr4u7w!z%C*F-JaNdRg";

pub type ArcConfig = Arc<Config>;

/// The application init configuration.
#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    /// Defines what ip the web server will be bound to.
    ///
    /// # Default
    ///
    /// If constructed with [`Default::default`] this value is set to `[0, 0, 0, 0]` (aka `0.0.0.0`).
    pub ip: [u8; 4],

    /// Defines what port the web server will listen to.
    ///
    /// # Default
    ///
    /// If constructed with [`Default::default`] this value is set to `8901`.
    pub port: u16,

    /// The database connection URI.
    ///
    /// Uses following format:
    ///
    /// ```not_rust
    /// scheme://[username:password@]host[:port1][,...hostN[:portN]][/[database][?options]]
    /// ```
    ///
    /// # Default
    ///
    /// If constructed with [`Default::default`] this value is set to `postgres://stry:stry@localhost:5432/stry`.
    ///
    /// # Examples
    ///
    /// Connecting to `PostgreSQL`:
    ///
    /// ```not_rust
    /// postgres://stry:stry@localhost:5432/stry
    /// ```
    ///
    /// Connecting with `SQLite`:
    ///
    /// ```not_rust
    /// sqlite://stry.db
    /// ```
    ///
    /// # Warning
    ///
    /// The parser for this is very simple and may not be able to understand
    /// every valid URI.
    pub database: String,

    /// The secret key used for JWT creation and verification.
    pub secret: String,
}

impl Config {
    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

impl Initialize for Config {
    fn init(config: &Anulap<'_>) -> Option<Self> {
        Some(Self {
            ip: config
                .get("ip")
                .and_then(|value| {
                    let mut parts = value
                        .split('.')
                        .map(str::parse)
                        .collect::<Vec<Result<u8, _>>>();

                    let four = parts.pop()?.ok()?;
                    let three = parts.pop()?.ok()?;
                    let two = parts.pop()?.ok()?;
                    let one = parts.pop()?.ok()?;

                    Some([one, two, three, four])
                })
                .unwrap_or_else(|| [0, 0, 0, 0]),
            port: config
                .get("port")
                .and_then(|value| value.parse().ok())
                .unwrap_or(8901),
            database: config
                .get("database")
                .unwrap_or_else(|| String::from("postgres://stry:stry@localhost:5432/stry")),
            secret: config
                .get("secret")
                .unwrap_or_else(|| String::from(DEFAULT_SECRET)),
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: [0, 0, 0, 0],
            port: 8901,
            database: String::from("postgres://stry:stry@localhost:5432/stry"),
            secret: String::from(DEFAULT_SECRET),
        }
    }
}
