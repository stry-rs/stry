//! Init config information, everything else is handled though the frontend.

use crate::layered::{Anulap, Initialize};

#[cfg(any(feature = "with-source-clap", feature = "with-source-on"))]
use crate::layered::Source;

/// A config layer source that pulls values from a clap `ArgMatches`.
#[cfg(feature = "with-source-clap")]
#[derive(Debug)]
pub struct ClapSource<'a> {
    matches: clap::ArgMatches<'a>,
}

#[cfg(feature = "with-source-clap")]
impl<'a> ClapSource<'a> {
    /// Create a new clap source.
    pub fn new(matches: clap::ArgMatches<'a>) -> Self {
        Self { matches }
    }
}

#[cfg(feature = "with-source-clap")]
impl<'a> Source for ClapSource<'a> {
    fn get(&self, key: &str) -> Option<String> {
        self.matches.value_of(key).map(String::from)
    }
}

/// A config layer source based on a RON file.
#[cfg(feature = "with-source-ron")]
#[derive(Debug)]
pub struct RonSource {
    value: ron::Value,
}

#[cfg(feature = "with-source-ron")]
impl RonSource {
    /// Create a new source using the data from a file.
    pub fn from_file<P>(path: P) -> Result<Self, RonSourceError>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::OpenOptions::new().read(true).open(path)?;
        let mut reader = std::io::BufReader::new(file);

        let value = ron::de::from_reader(&mut reader)?;

        Ok(Self { value })
    }
}

#[cfg(feature = "with-source-ron")]
impl Source for RonSource {
    fn get(&self, key: &str) -> Option<String> {
        match &self.value {
            ron::Value::Map(map) => map
                .iter()
                .find(|(k, _)| match k {
                    ron::Value::String(k) => k == key,
                    _ => false,
                })
                .and_then(|(_, value)| -> Option<String> {
                    match value {
                        ron::Value::Bool(boolean) => Some(boolean.to_string()),
                        ron::Value::Number(number) => match number {
                            ron::Number::Integer(integer) => Some(integer.to_string()),
                            ron::Number::Float(float) => Some(float.get().to_string()),
                        },
                        ron::Value::String(string) => Some(string.clone()),
                        _ => None,
                    }
                }),
            _ => None,
        }
    }
}

/// Errors that could occur when created a [`RonSource`].
#[cfg(feature = "with-source-ron")]
#[derive(Debug)]
pub enum RonSourceError {
    IO { source: std::io::Error },
    Ron { source: ron::Error },
}

#[cfg(feature = "with-source-ron")]
impl std::fmt::Display for RonSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RonSourceError::IO { source } => write!(f, "ron source, io error: {}", source),
            RonSourceError::Ron { source } => {
                write!(f, "ron source, ron deserialize error: {}", source)
            }
        }
    }
}

#[cfg(feature = "with-source-ron")]
impl std::error::Error for RonSourceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RonSourceError::IO { source } => Some(source),
            RonSourceError::Ron { source } => Some(source),
        }
    }
}

#[cfg(feature = "with-source-ron")]
impl From<std::io::Error> for RonSourceError {
    fn from(source: std::io::Error) -> Self {
        Self::IO { source }
    }
}

#[cfg(feature = "with-source-ron")]
impl From<ron::Error> for RonSourceError {
    fn from(source: ron::Error) -> Self {
        Self::Ron { source }
    }
}

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
        })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: [0, 0, 0, 0],
            port: 8901,
            database: String::from("postgres://stry:stry@localhost:5432/stry"),
        }
    }
}
