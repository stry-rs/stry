#[cfg(feature = "with-warp")]
mod warp;

use {
    crate::models::{Method, Route},
    proc_macro2::TokenStream,
    syn::ItemFn,
};

#[cfg(feature = "with-tide")]
pub fn generate<'i>(
    item: &'i ItemFn,
    method: Method,
    route: Route<'i>,
) -> Result<TokenStream, TokenStream> {
    todo!()
}

#[cfg(feature = "with-warp")]
pub fn generate<'i>(
    item: &'i ItemFn,
    method: Method,
    route: Route<'i>,
) -> Result<TokenStream, TokenStream> {
    warp::generate(item, method, route)
}
