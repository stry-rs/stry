use std::{env, fmt};

pub trait Source: fmt::Debug {
    fn get(&self, key: &str) -> Option<String>;
}

pub trait Initialize: Sized {
    fn init(config: &Anulap<'_>) -> Option<Self>;
}

#[derive(Debug, Default)]
pub struct Anulap<'s> {
    sources: Vec<Box<dyn Source + 's>>,
}

impl<'s> Anulap<'s> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with<S>(mut self, source: S) -> Self
    where
        S: Source + 's,
    {
        self.sources.push(Box::new(source));

        self
    }

    pub fn init<I>(&self) -> Option<I>
    where
        I: Initialize,
    {
        I::init(&self)
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
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

#[derive(Debug, Default)]
pub struct EnvSource {
    prefix: Option<String>,
}

impl EnvSource {
    pub fn new() -> Self {
        Self::default()
    }

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
