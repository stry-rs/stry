use std::sync::Arc;

use crate::{
    backend::{Backend, BackendEntry},
    models::{blog, core, story, wiki, Existing, Id, New},
};

pub struct ArcBackend {
    inner: Arc<dyn Backend + Send + Sync + 'static>,
}

impl ArcBackend {
    pub fn new<B>(backend: B) -> Self
    where
        B: Backend + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(backend),
        }
    }
}

impl std::ops::Deref for ArcBackend {
    type Target = Arc<dyn Backend + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[crate::prelude::async_trait]
impl Backend for ArcBackend {
    async fn migrate(&self) -> Result<(), anyhow::Error> {
        self.inner.migrate().await
    }
}

macro_rules! impl_entry {
    ($entry:ty) => {
        #[crate::prelude::async_trait]
        impl BackendEntry<$entry> for ArcBackend {
            async fn get(&self, id: Id) -> anyhow::Result<Existing<$entry>> {
                BackendEntry::<$entry>::get(&*self.inner, id).await
            }

            async fn all(&self, cursor: Option<Id>, limit: usize) -> anyhow::Result<Vec<Existing<$entry>>> {
                BackendEntry::<$entry>::all(&*self.inner, cursor, limit).await
            }

            async fn create(&self, data: New<$entry>) -> anyhow::Result<Id> {
                BackendEntry::<$entry>::create(&*self.inner, data).await
            }

            async fn update(&self, data: Existing<$entry>) -> anyhow::Result<()> {
                BackendEntry::<$entry>::update(&*self.inner, data).await
            }

            async fn remove(&self, id: Id) -> anyhow::Result<()> {
                BackendEntry::<$entry>::remove(&*self.inner, id).await
            }
        }
    };
    ($( $entry:ty , )*) => {
        $(
            impl_entry!($entry);
        )*
    };
}

#[rustfmt::skip]
impl_entry![
    core::Comment, core::Part, core::Tag, core::User,
    blog::Post,
    story::Chapter, story::Character, story::Origin, story::Pairing,
    story::Series, story::Story, story::Warning,
    wiki::Page,
];
