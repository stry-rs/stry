#![warn(rust_2018_idioms)]

pub mod utils;

pub mod loader;
pub mod models;

pub mod backend;
pub mod config;
pub mod error;
pub mod layered;
pub mod uri;

pub mod prelude {
    pub use crate::utils::{
        iter::IntoIteratorExt, HashMapExt, IntoOption, IntoResult,
        OptionExt, Peep, PeepOption, PeepResult, StringExt, Wrap,
    };

    pub use stry_macros::box_async;

    pub use anyhow::{bail, ensure, Context, Error};
    pub use async_trait::async_trait;
    pub use chrono::{DateTime, Utc};
    pub use serde::{Deserialize, Serialize};
    pub use std::convert::TryFrom;

    pub use tracing::{
        debug, debug_span, error, error_span, field, info, info_span, instrument, span, trace,
        trace_span, warn, warn_span, Instrument, Span,
    };

    pub use ultra_batch as dataloader;
}

pub mod futures {
    pub mod utils {
        pub use futures_util::*;
    }
}
