//! A very simple layered configuration wrapper.

use std::{env, fmt};

/// A config value source.
pub trait Source: fmt::Debug {
    fn get(&self, key: &str) -> Option<String>;
}

/// A wrapper trait for settings config values.
pub trait Initialize: Sized {
    fn init(config: &Anulap<'_>) -> Option<Self>;
}

/// A simple layered config.
#[derive(Debug, Default)]
pub struct Anulap<'s> {
    sources: Vec<Box<dyn Source + 's>>,
}

impl<'s> Anulap<'s> {
    /// Create new with no sources.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a source to the layer.
    ///
    /// # Note
    ///
    /// The earlier a source is added the higher it is on the source list.
    pub fn with<S>(mut self, source: S) -> Self
    where
        S: Source + 's,
    {
        self.sources.push(Box::new(source));

        self
    }

    /// A shorthand for `<* as Initialize>::init(config)`.
    pub fn init<I>(&self) -> Option<I>
    where
        I: Initialize,
    {
        I::init(self)
    }

    /// Retrieves the specified value returning the first one found.
    pub fn get(&self, key: &str) -> Option<String> {
        self.loop_get(key)
    }

    #[inline]
    fn loop_get(&self, key: &str) -> Option<String> {
        for source in &self.sources {
            if let Some(value) = source.get(key) {
                return Some(value);
            }
        }

        None
    }
}

/// A source that pulls values from the environment.
#[derive(Debug, Default)]
pub struct EnvSource {
    prefix: Option<String>,
}

impl EnvSource {
    /// Create without any prefix.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with a specified prefix.
    pub fn prefixed<P>(prefix: P) -> Self
    where
        P: fmt::Display,
    {
        Self {
            prefix: Some(prefix.to_string()),
        }
    }
}

impl Source for EnvSource {
    fn get(&self, key: &str) -> Option<String> {
        let key = if let Some(prefix) = &self.prefix {
            format!("{}_{}", prefix, key)
        } else {
            key.to_string()
        }
        .to_uppercase();

        env::var(key).ok()
    }
}
