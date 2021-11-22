use std::sync::Arc;

use crate::{
    backend::{Backend, BackendEntry},
    models::{blog, core, story, wiki, Existing, Id, New},
    prelude::*,
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

impl Clone for ArcBackend {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
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
    async fn migrate(&self) -> Result<(), Error> {
        self.inner.migrate().await
    }

    async fn register(&self, form: core::UserRegisterForm) -> Result<(), Error> {
        self.inner.register(form).await
    }
}

macro_rules! impl_entry {
    ($entry:ty) => {
        #[crate::prelude::async_trait]
        impl BackendEntry<$entry> for ArcBackend {
            async fn get(&self, id: Id) -> Result<Existing<$entry>, Error> {
                BackendEntry::<$entry>::get(&*self.inner, id).await
            }

            async fn all(&self, cursor: Option<Id>, limit: usize) -> Result<Vec<Existing<$entry>>, Error> {
                BackendEntry::<$entry>::all(&*self.inner, cursor, limit).await
            }

            async fn create(&self, data: New<$entry>) -> Result<Id, Error> {
                BackendEntry::<$entry>::create(&*self.inner, data).await
            }

            async fn update(&self, data: Existing<$entry>) -> Result<(), Error> {
                BackendEntry::<$entry>::update(&*self.inner, data).await
            }

            async fn remove(&self, id: Id) -> Result<(), Error> {
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
