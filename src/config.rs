use {
    miau::{builder::ConfigurationBuilder, provider::EnvironmentProvider},
    std::{collections::HashMap, env, marker::PhantomData, str::FromStr},
};

#[cfg(feature = "with-io")]
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

pub struct ConfigResolver<'c> {
    #[cfg(feature = "with-source-clap")]
    args: Option<clap::ArgMatches<'c>>,

    env: (bool, Option<String>),

    #[cfg(feature = "with-source-ron")]
    ron: Option<String>,

    _l: PhantomData<&'c ()>,
}

impl<'c> ConfigResolver<'c> {
    #[cfg(feature = "with-source-clap")]
    pub fn with_args(mut self, args: clap::ArgMatches<'c>) -> Self {
        self.args = Some(args);

        self
    }

    pub fn with_env(mut self) -> Self {
        self.env.0 = true;

        self
    }

    pub fn with_env_prefix<P>(mut self, prefix: P) -> Self
    where
        P: ToString,
    {
        self.env = (true, Some(prefix.to_string()));

        self
    }

    #[cfg(feature = "with-source-ron")]
    pub fn with_ron<P>(mut self, path: P) -> Self
    where
        P: ToString,
    {
        self.ron = Some(path.to_string());

        self
    }

    pub fn resolve(self) -> Result<Config, Error> {
        let mut builder = ConfigurationBuilder::default();

        #[cfg(feature = "with-source-clap")]
        if let Some(args) = self.args {
            builder.add_provider(ClapProvider::new(args));
        }

        match self.env {
            (true, None) => {
                builder.add_provider(EnvironmentProvider::new());
            }
            (true, Some(prefix)) => {
                builder.add_provider(EnvironmentProvider::with_prefix(prefix));
            }
            (_, _) => {}
        }

        #[cfg(feature = "with-source-ron")]
        if let Some(path) = self.ron {
            builder.add(miau::source::FileSource::from_path(path), Ron);
        }

        todo!()
    }
}

impl<'c> Default for ConfigResolver<'c> {
    fn default() -> Self {
        Self {
            #[cfg(feature = "with-source-clap")]
            args: None,

            env: (false, None),

            #[cfg(feature = "with-source-ron")]
            ron: None,

            _l: PhantomData,
        }
    }
}

#[cfg(feature = "with-source-ron")]
struct Ron;

#[cfg(feature = "with-source-ron")]
impl miau::format::Format for Ron {
    fn transform(
        &self,
        input: Vec<u8>,
    ) -> Result<miau::configuration::ConfigurationTree, miau::error::ConfigurationError> {
        ron::de::from_bytes::<miau::configuration::ConfigurationTree>(&input)
            .map_err(|e| miau::error::ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "ron".into()
    }
}

#[cfg(feature = "with-source-clap")]
struct ClapProvider<'c> {
    args: clap::ArgMatches<'c>,
}

#[cfg(feature = "with-source-clap")]
impl<'c> ClapProvider<'c> {
    fn new(args: clap::ArgMatches<'c>) -> Self {
        Self { args }
    }
}

#[cfg(feature = "with-source-clap")]
impl<'c> miau::provider::Provider for ClapProvider<'c> {
    fn collect(
        &self,
    ) -> Result<miau::configuration::Configuration, miau::error::ConfigurationError> {
        todo!()
    }

    fn describe(&self) -> miau::configuration::ConfigurationInfo {
        miau::configuration::ConfigurationInfo::new("args", "clap")
    }
}

pub trait Source {
    fn exists<S>(&self, key: S) -> bool
    where
        S: AsRef<str>;

    fn get_string<S>(&self, key: S) -> Option<String>
    where
        S: AsRef<str>;
}

#[cfg(feature = "source-clap")]
impl<'m> Source for clap::ArgMatches<'m> {
    fn exists<S>(&self, key: S) -> bool
    where
        S: AsRef<str>,
    {
        self.is_present(key)
    }

    fn get_string<S>(&self, key: S) -> Option<String>
    where
        S: AsRef<str>,
    {
        self.value_of(key).map(String::from)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "with-io")]
    #[error("Unable to open or read file")]
    IO {
        #[from]
        source: io::Error,
    },
    #[cfg(feature = "with-io")]
    #[error("Unable deserialize config")]
    Ron {
        #[from]
        source: ron::Error,
    },
    #[error("Frontend can only be `both`, `api` or `user`, found '{value}'")]
    InvalidFrontendValue { value: String },
    #[error("Log level can only be `error`, `warn`, `info`, `debug` or `trace`, found '{value}'")]
    InvalidLogLevel { value: String },
    #[error("Worker count can only be a multiple of 4 (up to 32), found '{value}'")]
    InvalidWorkerCountValue { value: String },
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub ip: [u8; 4],
    pub port: u16,
    pub tls: Tls,
    pub frontend: Frontend,
    pub workers: FourCount,
    pub database: Database,
    pub executor: Executor,
    pub logging: Logging,
}

impl Config {
    #[cfg(feature = "with-io")]
    pub fn load_from_file<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let cfg_path = path.as_ref();

        let config = if cfg_path.exists() {
            let file = fs::OpenOptions::new().read(true).open(cfg_path)?;
            let mut reader = io::BufReader::new(file);

            let mut contents = String::new();

            reader.read_to_string(&mut contents)?;

            ron::de::from_str(&contents)?
        } else {
            Config::default()
        };

        Ok(config)
    }

    pub fn with_source<S>(&mut self, source: &S)
    where
        S: Source,
    {
        if let Some(new) = env::var("STRY_SERVER_IP")
            .ok()
            .or_else(|| source.get_string("server-ip"))
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
        {
            self.ip = new;
        }

        if let Some(new) = env::var("STRY_SERVER_PORT")
            .ok()
            .or_else(|| source.get_string("server-port"))
            .and_then(|value| {
                let port = value.parse().ok()?;

                Some(port)
            })
        {
            self.port = new;
        }

        if let Some(new) = env::var("STRY_TLS").ok().and_then(|value| {
            let cert = env::var("STRY_TLS_CERT").ok()?;
            let key = env::var("STRY_TLS_KEY").ok()?;

            match &*value.to_lowercase() {
                "file" => Some(Tls::File { cert, key }),
                "text" => Some(Tls::Text { cert, key }),
                _ => None,
            }
        }) {
            self.tls = new;
        }

        if let Some(new) = env::var("STRY_FRONTEND").ok().and_then(|value| {
            let frontend = Frontend::from_str(&value).ok()?;

            Some(frontend)
        }) {
            self.frontend = new;
        }

        if let Some(new) = env::var("STRY_WORKERS").ok().and_then(|value| {
            let workers = FourCount::from_str(&value).ok()?;

            Some(workers)
        }) {
            self.workers = new;
        }

        self.database.with_source(source);
        self.executor.with_source(source);
        self.logging.with_source(source);
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ip: [0, 0, 0, 0],
            port: 8901,
            tls: Tls::None,
            frontend: Frontend::Both,
            workers: FourCount::Four,
            database: Database::default(),
            executor: Executor::default(),
            logging: Logging::default(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub enum Tls {
    File { cert: String, key: String },
    Text { cert: String, key: String },
    None,
}

impl Default for Tls {
    fn default() -> Self {
        Tls::None
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Frontend {
    Both,
    Api,
    User,
}

impl Frontend {
    pub fn as_bool(self) -> (bool, bool) {
        match self {
            Frontend::Both => (true, true),
            Frontend::Api => (true, false),
            Frontend::User => (false, true),
        }
    }
}

impl FromStr for Frontend {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "both" => Ok(Frontend::Both),
            "api" => Ok(Frontend::Api),
            "user" => Ok(Frontend::User),
            _ => Err(Error::InvalidFrontendValue {
                value: s.to_string(),
            }),
        }
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum FourCount {
    Four,
    Eight,
    Twelve,
    Sixteen,
    Twenty,
    TwentyFour,
    TwentyEight,
    ThirtyTwo,
}

impl FourCount {
    pub fn as_count(self) -> usize {
        match self {
            FourCount::Four => 4,
            FourCount::Eight => 8,
            FourCount::Twelve => 12,
            FourCount::Sixteen => 16,
            FourCount::Twenty => 20,
            FourCount::TwentyFour => 24,
            FourCount::TwentyEight => 28,
            FourCount::ThirtyTwo => 32,
        }
    }
}

impl FromStr for FourCount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "4" => Ok(FourCount::Four),
            "8" => Ok(FourCount::Eight),
            "12" => Ok(FourCount::Twelve),
            "16" => Ok(FourCount::Sixteen),
            "20" => Ok(FourCount::Twenty),
            "24" => Ok(FourCount::TwentyFour),
            "28" => Ok(FourCount::TwentyEight),
            "32" => Ok(FourCount::ThirtyTwo),
            _ => Err(Error::InvalidWorkerCountValue {
                value: s.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Database {
    #[serde(rename = "type")]
    pub typ: BackendType,
    pub storage: StorageType,
}

impl Database {
    pub fn with_source<S>(&mut self, source: &S)
    where
        S: Source,
    {
        if let Some(new) = env::var("STRY_BACKEND_TYPE")
            .ok()
            .or_else(|| source.get_string("backend-type"))
            .and_then(|value| match &*value.to_lowercase() {
                "postgres" => Some(BackendType::Postgres),
                "sqlite" => Some(BackendType::Sqlite),
                _ => None,
            })
        {
            self.typ = new;
        }

        match self.typ {
            BackendType::Postgres => {
                if let Some(new) = env::var("STRY_BACKEND_HOST")
                    .ok()
                    .map(|host| {
                        let port = env::var("STRY_BACKEND_PORT").ok();
                        let database = env::var("STRY_BACKEND_DATABASE").ok();
                        let username = env::var("STRY_BACKEND_USERNAME").ok();
                        let password = env::var("STRY_BACKEND_PASSWORD").ok();

                        (username, password, host, port, database)
                    })
                    .or_else(|| {
                        source.get_string("backend-host").map(|host| {
                            let port = source.get_string("backend-port");
                            let database = source.get_string("backend-database");
                            let username = source.get_string("backend-username");
                            let password = source.get_string("backend-password");

                            (username, password, host, port, database)
                        })
                    })
                    .map(|(username, password, host, port, database)| {
                        StorageType::Parts {
                            username,
                            password,
                            host,
                            port,
                            database,
                            // TODO: maybe serde will help with this
                            params: None,
                        }
                    })
                {
                    self.storage = new;
                }
            }
            BackendType::Sqlite => {
                if let Some(new) = env::var("STRY_BACKEND_FILE")
                    .ok()
                    .or_else(|| source.get_string("backend-file"))
                    .map(|location| StorageType::File { location })
                {
                    self.storage = new;
                }
            }
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self {
            typ: BackendType::Sqlite,
            storage: StorageType::File {
                location: String::from("stry.db"),
            },
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum StorageType {
    File {
        location: String,
    },
    Parts {
        username: Option<String>,
        password: Option<String>,
        host: String,
        port: Option<String>,
        database: Option<String>,
        params: Option<HashMap<String, String>>,
    },
}

impl StorageType {
    pub fn is_file(&self) -> bool {
        matches!(self, StorageType::File { .. })
    }

    pub fn is_parts(&self) -> bool {
        matches!(self, StorageType::Parts { .. })
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum BackendType {
    Postgres,
    Sqlite,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Executor {
    pub core_threads: Option<usize>,
    pub max_threads: Option<usize>,
}

impl Executor {
    pub fn with_source<S>(&mut self, _source: &S)
    where
        S: Source,
    {
        if let Some(new) = env::var("STRY_EXECUTOR_CORE_THREADS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
        {
            self.core_threads = Some(new);
        }

        if let Some(new) = env::var("STRY_EXECUTOR_MAX_THREADS")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
        {
            self.max_threads = Some(new);
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self {
            core_threads: None,
            max_threads: None,
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(default)]
pub struct Logging {
    pub ansi: bool,
    pub flame: Option<String>,
    pub level: LogLevel,
    pub out: LoggingOutput,
    pub thread_ids: bool,
    pub thread_names: bool,
}

impl Logging {
    pub fn with_source<S>(&mut self, source: &S)
    where
        S: Source,
    {
        if let Some(new) = env::var("STRY_LOGGING_ANSI")
            .ok()
            .map(|value| !matches!(&*value, "0"))
        {
            self.ansi = new;
        }

        if let Some(new) = env::var("STRY_LOGGING_FLAME")
            .ok()
            .or_else(|| source.get_string("tracing-flame"))
        {
            self.flame = Some(new);
        }

        if let Some(new) = env::var("STRY_LOGGING_LEVEL")
            .ok()
            .or_else(|| source.get_string("tracing-level"))
            .and_then(|value| LogLevel::from_str(&value).ok())
        {
            self.level = new;
        }

        if let Some(new) = env::var("STRY_LOGGING_OUTPUT")
            .ok()
            .map(|output| {
                let json = env::var("STRY_LOGGING_JSON")
                    .map(|value| !matches!(&*value, "0"))
                    .ok();

                let directory = env::var("STRY_LOGGING_DIRECTORY").ok();
                let prefix = env::var("STRY_LOGGING_PREFIX").ok();

                (output, directory, json, prefix)
            })
            .map(|(output, mut directory, json, mut prefix)| {
                if directory.is_none() && source.exists("tracing-directory") {
                    directory = source.get_string("tracing-directory");
                }

                if prefix.is_none() && source.exists("tracing-prefix") {
                    prefix = source.get_string("tracing-prefix");
                }

                (output, directory, json, prefix)
            })
            .and_then(
                |(output, directory, json, prefix)| match &*output.to_lowercase() {
                    "both" => Some(LoggingOutput::Both {
                        directory: directory?,
                        json: json?,
                        prefix: prefix?,
                    }),
                    "file" => Some(LoggingOutput::File {
                        directory: directory?,
                        json: json?,
                        prefix: prefix?,
                    }),
                    "stdout" => Some(LoggingOutput::StdOut { json: json? }),
                    _ => None,
                },
            )
        {
            self.out = new;
        }

        if let Some(new) = env::var("STRY_LOGGING_THREAD_IDS")
            .ok()
            .map(|value| !matches!(&*value, "0"))
        {
            self.thread_ids = new;
        }

        if let Some(new) = env::var("STRY_LOGGING_THREAD_NAMES")
            .ok()
            .map(|value| !matches!(&*value, "0"))
        {
            self.thread_names = new;
        }
    }
}

impl Default for Logging {
    fn default() -> Self {
        Self {
            ansi: true,
            flame: None,
            level: LogLevel::Debug,
            out: LoggingOutput::StdOut { json: false },
            thread_ids: true,
            thread_names: true,
        }
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl FromStr for LogLevel {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower = s.to_lowercase();

        match lower.as_str() {
            "error" => Ok(LogLevel::Error),
            "warn" => Ok(LogLevel::Warn),
            "info" => Ok(LogLevel::Info),
            "debug" => Ok(LogLevel::Debug),
            "trace" => Ok(LogLevel::Trace),
            _ => Err(Error::InvalidLogLevel {
                value: s.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize)]
pub enum LoggingOutput {
    Both {
        directory: String,
        json: bool,
        prefix: String,
    },
    File {
        directory: String,
        json: bool,
        prefix: String,
    },
    StdOut {
        json: bool,
    },
}
