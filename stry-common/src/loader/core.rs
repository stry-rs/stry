use ultra_batch::{Batcher, Cache, Fetcher};

use crate::{
    backend::{Backend, BackendEntry},
    models::{
        core::{Tag, User},
        Existing, Id,
    },
};

pub struct UserLoader<B: Backend + Send + Sync + 'static> {
    backend: B,
}

impl<B: Backend + Send + Sync + 'static> UserLoader<B> {
    pub fn new(backend: B) -> Batcher<UserLoader<B>> {
        Batcher::new(Self { backend }).build()
    }
}

#[crate::prelude::async_trait]
impl<B: Backend + Send + Sync + 'static> Fetcher for UserLoader<B> {
    type Key = Id;
    type Value = Existing<User>;
    type Error = crate::prelude::Error;

    async fn fetch(
        &self,
        keys: &[Self::Key],
        values: &mut Cache<'_, Self::Key, Self::Value>,
    ) -> Result<(), Self::Error> {
        for id in keys {
            let value = BackendEntry::<User>::get(&self.backend, *id).await?;

            values.insert(*id, value);
        }

        Ok(())
    }
}

pub struct TagLoader<B: Backend + Send + Sync + 'static> {
    backend: B,
}

impl<B: Backend + Send + Sync + 'static> TagLoader<B> {
    pub fn new(backend: B) -> Batcher<TagLoader<B>> {
        Batcher::new(Self { backend }).build()
    }
}

#[crate::prelude::async_trait]
impl<B: Backend + Send + Sync + 'static> Fetcher for TagLoader<B> {
    type Key = Id;
    type Value = Existing<Tag>;
    type Error = crate::prelude::Error;

    async fn fetch(
        &self,
        keys: &[Self::Key],
        values: &mut Cache<'_, Self::Key, Self::Value>,
    ) -> Result<(), Self::Error> {
        for id in keys {
            let value = BackendEntry::<Tag>::get(&self.backend, *id).await?;

            values.insert(*id, value);
        }

        Ok(())
    }
}
