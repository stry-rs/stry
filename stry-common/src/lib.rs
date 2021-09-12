#![warn(rust_2018_idioms)]

pub mod utils;

pub mod backend;
pub mod config;
pub mod error;
pub mod layered;
pub mod models;
pub mod uri;

pub mod prelude {
    pub use crate::utils::{
        iter::IntoIteratorExt as _, HashMapExt as _, IntoOption as _, IntoResult as _,
        OptionExt as _, Peep as _, PeepOption as _, PeepResult as _, StringExt as _, Wrap as _,
    };

    pub use stry_macros::box_async;

    pub use anyhow::{bail, ensure, Context as _, Error};
    pub use async_trait::async_trait;
    pub use chrono::{DateTime, Utc};
    pub use serde::{Deserialize, Serialize};
    pub use std::convert::TryFrom as _;

    pub use tracing::{
        debug, debug_span, error, error_span, field, info, info_span, instrument, span, trace,
        trace_span, warn, warn_span, Instrument as _, Span,
    };
}

pub mod futures {
    pub mod utils {
        pub use futures_util::*;
    }
}
