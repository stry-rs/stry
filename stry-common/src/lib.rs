#![warn(rust_2018_idioms)]

pub mod utils;

pub mod backend;
pub mod dataloader;
pub mod loader;
pub mod members;
pub mod models;

pub mod config;
pub mod error;
pub mod http;
// pub mod layered;
pub mod uri;

pub mod prelude {
    pub use crate::{members, utils::Member};

    pub use fenn::{
        iter::IntoIteratorExt, HashMapExt, IntoOption, IntoResult, OptionExt, Peep, PeepOption,
        PeepResult, StringExt, Wrap,
    };
    pub use stry_macros::box_async;

    pub use anyhow::{anyhow as err, bail, ensure, Context, Error};
    pub use async_trait::async_trait;
    pub use serde::{Deserialize, Serialize};
    pub use std::convert::TryFrom;
    pub use time::{OffsetDateTime, UtcOffset};
    pub use validator::Validate;

    pub use tracing::{
        self, debug, debug_span, error, error_span, field, info, info_span, instrument, span,
        trace, trace_span, warn, warn_span, Instrument, Span,
    };
}

pub mod futures {
    pub mod utils {
        pub use futures_util::*;
    }
}
