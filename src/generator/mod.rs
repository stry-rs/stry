mod warp;

use {
    crate::models::{Method, Route},
    proc_macro2::TokenStream,
    syn::ItemFn,
};

#[cfg(not(any(feature = "with-tide", feature = "with-warp")))]
pub fn generate<'i>(
    item: &'i ItemFn,
    method: Method,
    route: Route<'i>,
) -> Result<TokenStream, TokenStream> {
    Err(syn::Error::new_spanned(item, "Either feature `with-tide` or `with-warp` must be enabled for this crate.").to_compile_error())
}

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
