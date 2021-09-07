#![warn(rust_2018_idioms)]

pub mod utils;

pub mod backend;
pub mod config;
pub mod layered;
pub mod models;
pub mod uri;

pub mod prelude {
    pub use crate::utils::{
        iter::IntoIteratorExt as _, HashMapExt as _, IntoOption as _, IntoResult as _,
        OptionExt as _, Peep as _, PeepOption as _, PeepResult as _, StringExt as _, Wrap as _,
    };
    pub use anyhow::{bail, ensure, Context as _, Error};
}
