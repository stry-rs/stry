mod generator;

mod models;
mod parser;

use crate::models::Method;

macro_rules! handler {
    ($( $name:ident => $method:expr, )*) => {
        $(
            #[proc_macro_attribute]
            pub fn $name(
                attr: proc_macro::TokenStream,
                body: proc_macro::TokenStream,
            ) -> proc_macro::TokenStream {
                // $crate::warp::route(
                //     $method,
                //     syn::parse_macro_input!(attr as syn::LitStr),
                //     syn::parse_macro_input!(body as syn::ItemFn),
                // )
                let attr = syn::parse_macro_input!(attr as syn::LitStr);
                let body = syn::parse_macro_input!(body as syn::ItemFn);
                let path = attr.value();

                let route = match $crate::parser::parse(&attr, &path, &body) {
                    Ok(route) => route,
                    Err(stream) => return proc_macro::TokenStream::from(stream),
                };

                match $crate::generator::generate(&body, $method, route) {
                    Ok(stream) => proc_macro::TokenStream::from(stream),
                    Err(stream) => proc_macro::TokenStream::from(stream),
                }
            }
        )*
    };
}

handler! {
    delete => Method::Delete,
    get => Method::Get,
    head => Method::Head,
    options => Method::Options,
    patch => Method::Patch,
    post => Method::Post,
    put => Method::Put,
}
