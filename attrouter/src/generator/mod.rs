mod warp;

use {
    crate::models::{Method, Route},
    proc_macro2::TokenStream,
    syn::ItemFn,
};

pub fn generate<'i>(
    item: &'i ItemFn,
    method: Method,
    route: Route<'i>,
) -> Result<TokenStream, TokenStream> {
    if cfg!(feature = "with-warp") {
        warp::generate(item, method, route)
    } else if cfg!(feature = "with-tide") {
        todo!()
    } else {
        Err(syn::Error::new_spanned(
            item,
            "Either feature `with-tide` or `with-warp` must be enabled for this crate.",
        )
        .to_compile_error())
    }
}
