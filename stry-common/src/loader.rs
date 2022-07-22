macro_rules! impl_loader {
    ($( $name:ident => ($entity:path, $value:path) , )*) => {
        $(
            pub struct $name<B: crate::backend::Backend + Send + Sync + 'static> {
                backend: B,
            }

            impl<B: crate::backend::Backend + Send + Sync + 'static> $name<B> {
                pub fn new(backend: B) -> crate::dataloader::Batcher<$name<B>> {
                    crate::dataloader::Batcher::builder(Self { backend }).build()
                }
            }

            #[crate::prelude::async_trait]
            impl<B: crate::backend::Backend + Send + Sync + 'static> crate::dataloader::Fetcher for $name<B> {
                type Key = crate::models::Id;
                type Value = crate::models::Existing<$value>;
                type Error = crate::prelude::Error;

                async fn fetch(
                    &self,
                    keys: &[Self::Key],
                    values: &mut crate::dataloader::Cache<'_, Self::Key, Self::Value>,
                ) -> Result<(), Self::Error> {
                    for id in keys {
                        let value = <B as $entity>::get(&self.backend, *id).await?;

                        values.insert(*id, value);
                    }

                    Ok(())
                }
            }
        )*
    };
}

pub mod core {
    #[rustfmt::skip]
    impl_loader! {
        UserLoader => (crate::backend::UserEntity ,crate::models::core::User),
        TagLoader => (crate::backend::TagEntity ,crate::models::core::Tag),
    }
}

pub mod story {
    use crate::{backend::Backend, dataloader::Batcher};

    use super::core::{TagLoader, UserLoader};

    #[rustfmt::skip]
    impl_loader! {
        OriginLoader => (crate::backend::OriginEntity ,crate::models::story::Origin),
        WarningLoader => (crate::backend::WarningEntity ,crate::models::story::Warning),
        PairingLoader => (crate::backend::PairingEntity ,crate::models::story::Pairing),
        CharacterLoader => (crate::backend::CharacterEntity ,crate::models::story::Character),
    }

    #[derive(Clone)]
    pub struct StoryLoaders<B: Backend + Clone + Send + Sync + 'static> {
        pub user: Batcher<UserLoader<B>>,
        pub tag: Batcher<TagLoader<B>>,

        pub origin: Batcher<OriginLoader<B>>,
        pub warning: Batcher<WarningLoader<B>>,
    }

    impl<B: Backend + Clone + Send + Sync + 'static> StoryLoaders<B> {
        pub fn new(backend: B) -> Self {
            Self {
                user: UserLoader::new(backend.clone()),
                tag: TagLoader::new(backend.clone()),

                origin: OriginLoader::new(backend.clone()),
                warning: WarningLoader::new(backend),
            }
        }
    }
}
