//! Init config information, everything else is handled though the frontend.

use std::sync::Arc;

use twelf::Layer;

/// The default secret key.
///
/// Used to check if the server was supplied with a new key.
pub static DEFAULT_SECRET: &str = "&E)H@McQfTjWnZr4u7w!z%C*F-JaNdRg";

pub type ArcConfig = Arc<Config>;

/// The application init configuration.
#[twelf::config]
#[derive(Clone, Debug)]
pub struct Config {
    /// Defines what ip the web server will be bound to.
    ///
    /// # Default
    ///
    /// If constructed with [`Default::default`] this value is set to `[0, 0, 0, 0]` (aka `0.0.0.0`).
    #[serde(default = "default_ip")]
    pub ip: [u8; 4],

    /// Defines what port the web server will listen to.
    ///
    /// # Default
    ///
    /// If constructed with [`Default::default`] this value is set to `8901`.
    #[serde(default = "default_port")]
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
    #[serde(default = "default_database")]
    pub database: String,

    /// The secret key used for JWT creation and verification.
    #[serde(default = "default_secret")]
    pub secret: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        Ok(Self::with_layers(&[
            Layer::Toml("stry.toml".into()),
            Layer::Env(Some("STRY_".to_string())),
        ])?)
    }

    pub fn into_arc(self) -> Arc<Self> {
        Arc::new(self)
    }
}

fn default_ip() -> [u8; 4] {
    [0, 0, 0, 0]
}

fn default_port() -> u16 {
    8901
}

fn default_database() -> String {
    String::from("postgres://stry:stry@localhost:5432/stry")
}

fn default_secret() -> String {
    String::from(DEFAULT_SECRET)
}
