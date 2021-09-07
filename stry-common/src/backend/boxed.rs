use crate::{
    backend::{Backend, BackendEntry},
    models::{blog, core, story, wiki, Existing, Id, New},
};

pub struct BoxedBackend {
    inner: Box<dyn Backend + Send + Sync + 'static>,
}

impl BoxedBackend {
    pub fn new<B>(backend: B) -> Self
    where
        B: Backend + Send + Sync + 'static,
    {
        Self {
            inner: Box::new(backend),
        }
    }
}

impl std::ops::Deref for BoxedBackend {
    type Target = Box<dyn Backend + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

macro_rules! impl_entry {
    ($entry:ty) => {
        #[async_trait::async_trait]
        impl BackendEntry<$entry> for BoxedBackend {
            async fn get(&self, id: Id) -> anyhow::Result<Existing<$entry>> {
                BackendEntry::<$entry>::get(&*self, id).await
            }

            async fn all(&self, cursor: Id, limit: usize) -> anyhow::Result<Vec<Existing<$entry>>> {
                BackendEntry::<$entry>::all(&*self, cursor, limit).await
            }

            async fn create(&self, data: New<$entry>) -> anyhow::Result<Id> {
                BackendEntry::<$entry>::create(&*self, data).await
            }

            async fn update(&self, data: Existing<$entry>) -> anyhow::Result<()> {
                BackendEntry::<$entry>::update(&*self, data).await
            }

            async fn remove(&self, id: Id) -> anyhow::Result<()> {
                BackendEntry::<$entry>::remove(&*self, id).await
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
