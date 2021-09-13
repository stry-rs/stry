macro_rules! impl_loader {
    ($( $name:ident => $value:path , )*) => {
        $(
            pub struct $name<B: crate::backend::Backend + Send + Sync + 'static> {
                backend: B,
            }

            impl<B: crate::backend::Backend + Send + Sync + 'static> $name<B> {
                pub fn new(backend: B) -> crate::prelude::dataloader::Batcher<$name<B>> {
                    crate::prelude::dataloader::Batcher::new(Self { backend }).build()
                }
            }

            #[crate::prelude::async_trait]
            impl<B: crate::backend::Backend + Send + Sync + 'static> crate::prelude::dataloader::Fetcher for $name<B> {
                type Key = crate::models::Id;
                type Value = crate::models::Existing<$value>;
                type Error = crate::prelude::Error;

                async fn fetch(
                    &self,
                    keys: &[Self::Key],
                    values: &mut crate::prelude::dataloader::Cache<'_, Self::Key, Self::Value>,
                ) -> Result<(), Self::Error> {
                    for id in keys {
                        let value = crate::backend::BackendEntry::<$value>::get(&self.backend, *id).await?;

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
        UserLoader => crate::models::core::User,
        TagLoader => crate::models::core::Tag,
    }
}

pub mod story {
    #[rustfmt::skip]
    impl_loader! {
        OriginLoader => crate::models::story::Origin,
        WarningLoader => crate::models::story::Warning,
        PairingLoader => crate::models::story::Pairing,
        CharacterLoader => crate::models::story::Character,
    }
}
