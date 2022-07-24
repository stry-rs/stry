use std::{collections::HashMap, sync::Arc};

use fluent::FluentResource;
use fluent_langneg::NegotiationStrategy;
use intl_memoizer::concurrent::IntlLangMemoizer;
use unic_langid::{langid, LanguageIdentifier};

type FluentBundle = fluent::bundle::FluentBundle<FluentResource, IntlLangMemoizer>;

pub struct FluentResources {
    inner: Arc<Inner>,
}

struct Inner {
    default: LanguageIdentifier,
    bundles: HashMap<LanguageIdentifier, Arc<FluentBundle>>,
    languages: Vec<LanguageIdentifier>,
    strategy: NegotiationStrategy,
}

impl FluentResources {
    #[must_use]
    pub fn builder() -> FluentResourcesBuilder {
        FluentResourcesBuilder {
            default: langid!("en-US"),
            resources: vec![],
            strategy: NegotiationStrategy::Filtering,
        }
    }
}

pub struct FluentResourcesBuilder {
    default: LanguageIdentifier,
    resources: Vec<(String, FluentBundle)>,
    strategy: NegotiationStrategy,
}

impl FluentResourcesBuilder {
    #[must_use]
    pub fn add_bundle<L>(mut self, language: L, bundle: FluentBundle) -> Self
    where
        L: Into<String>,
    {
        self.resources.push((language.into(), bundle));
        self
    }

    #[must_use]
    pub fn default_language(mut self, language: LanguageIdentifier) -> Self {
        self.default = language;
        self
    }

    #[must_use]
    pub fn negotiation_strategy(mut self, strategy: NegotiationStrategy) -> Self {
        self.strategy = strategy;
        self
    }
}
