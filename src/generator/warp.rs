use {
    crate::{
        models::{FnParam, FnParamKind, GuardParam, Method, Route, UrlParam, UrlParams},
        parser,
    },
    proc_macro2::TokenStream,
    syn::{FnArg, ItemFn, PatType},
};

pub fn generate<'i>(
    item: &'i ItemFn,
    method: Method,
    route: Route<'i>,
) -> Result<TokenStream, TokenStream> {
    // Convert additional function attributes to warp filters
    let attrs_stream = build_guards(&route.guard_params);

    // Create the routing filters and closure inputs
    let (closure, routing) = build_filters_and_inputs(&route.url_params, &route.fn_params)?;

    let wrapper_inputs = item
        .sig
        .inputs
        .clone()
        .into_iter()
        .filter(|input| match input {
            FnArg::Receiver(_) => false,
            FnArg::Typed(PatType { attrs, .. }) => attrs.iter().any(|a| a.path.is_ident("data")),
        })
        .map(|input| match input {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(PatType {
                attrs,
                pat,
                colon_token,
                ty,
            }) => FnArg::Typed(PatType {
                attrs: attrs.into_iter().partition(parser::is_attr).1,
                pat,
                colon_token,
                ty,
            }),
        })
        .collect::<Vec<_>>();

    // Reference the data types to clone them
    let data = wrapper_inputs.iter().map(|input| match input {
        FnArg::Receiver(_) => unreachable!(),
        FnArg::Typed(PatType { pat, .. }) => pat,
    });

    // Extract the function parameter names
    let fn_inputs = route
        .clean
        .sig
        .inputs
        .iter()
        .map(|input| match input {
            FnArg::Receiver(_) => unreachable!(),
            FnArg::Typed(PatType { pat, .. }) => pat,
        })
        .collect::<Vec<_>>();

    // Async functions use `and_then` while sync functions use `map`
    // Select and build the closure for the right one
    let map = if route.asyncness {
        quote::quote! { .and_then(|#closure| {
            #( let #data = #data.clone(); )*

            async move {
                inner(#( #fn_inputs ),*).await
            }
        }) }
    } else {
        quote::quote! { .map(|#closure| {
            #( let #data = #data.clone(); )*

            inner(#( #fn_inputs ),*)
        }) }
    };

    // Select the correct filter base
    let method_handler = match method {
        Method::Delete => quote::quote! { delete() },
        Method::Get => quote::quote! { get() },
        Method::Head => quote::quote! { head() },
        Method::Options => quote::quote! { options() },
        Method::Patch => quote::quote! { patch() },
        Method::Post => quote::quote! { post() },
        Method::Put => quote::quote! { get() },
    };

    // Automatically imports warp's Filter trait
    let filter = quote::quote! {
        warp::filters::method::#method_handler
        #attrs_stream
        #routing
        .and(warp::filters::path::end())
        #map
    };

    let vis = &item.vis;
    let name = &item.sig.ident;
    let inner = route.clean;

    Ok(quote::quote! {
        #vis fn #name( #( #wrapper_inputs ),* ) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + std::clone::Clone {
            #inner

            #[allow(unused_imports)]
            {
                use warp::Filter;

                #filter
            }
        }
    })
}

fn build_guards(guards: &[GuardParam]) -> TokenStream {
    let mut parts = Vec::new();

    for attr in guards {
        // TODO: CORS
        match attr {
            GuardParam::BodySize { key, value } => {
                if key == "max" {
                    parts.push(
                        quote::quote! { and(warp::filters::body::content_length_limit(#value)) },
                    );
                } else {
                    return syn::Error::new_spanned(
                        &key,
                        format!(r#"failed to parse attribute, invalid type `{}`, example: #[body_size(max = "1024")]"#, key),
                    ).to_compile_error();
                }
            }
            GuardParam::Header { key, value } => {
                parts.push(quote::quote! { and(warp::filters::header::exact(#key, #value)) });
            }
        }
    }

    quote::quote! { #( . #parts )* }
}

// Create the actually routing filters
fn build_filters_and_inputs(
    url_params: &UrlParams<'_>,
    fun_params: &[FnParam<'_>],
) -> Result<(TokenStream, TokenStream), TokenStream> {
    let mut closure = Vec::new();
    let mut routing = Vec::new();

    // Convert URL chunks into warp filters
    for param in &url_params.items {
        match param {
            UrlParam::Chunk { value } => {
                routing.push(quote::quote! { and(warp::filters::path::path(#value)) });
            }
            UrlParam::Param { key } => {
                // Get the URL param type
                let param = fun_params.iter().find(|param| param.key == *key);

                match param {
                    Some(FnParam { typ, .. }) => {
                        let ident = quote::format_ident!("{}", key);

                        closure.push(quote::quote! { #ident: #typ });
                        routing.push(quote::quote! { and(warp::filters::path::param::<#typ>()) });
                    }
                    None => {
                        let spanned = syn::Error::new_spanned(url_params.token, "Unable to find url param type, make sure the param has a matching function parameter");

                        return Err(spanned.to_compile_error());
                    }
                }
            }
        }
    }

    // Data filters go at the end of the filter, so I need to split them
    let (data, others): (Vec<&FnParam<'_>>, Vec<&FnParam<'_>>) = fun_params
        .iter()
        .partition(|p| matches!(p.kind, Some(FnParamKind::Data)));

    // Convert non data attributes to warp filters
    for FnParam {
        token,
        key,
        typ,
        kind,
    } in others
    {
        let stream = match kind {
            Some(FnParamKind::Data) => {
                let spanned = syn::Error::new_spanned(token, "warp-macros bug: This should not happen, invalid partitioned enum variant: non data");

                return Err(spanned.to_compile_error());
            }
            Some(FnParamKind::Form) => {
                quote::quote! { and(warp::filters::body::form::<#typ>()) }
            }
            Some(FnParamKind::Header { header }) => {
                quote::quote! { and(warp::filters::header::header::<#typ>(#header)) }
            }
            Some(FnParamKind::Json) => {
                quote::quote! { and(warp::filters::body::json::<#typ>()) }
            }
            Some(FnParamKind::Query) => {
                quote::quote! { and(warp::filters::query::query::<#typ>()) }
            }
            None => continue,
        };

        closure.push(quote::quote! { #key: #typ });
        routing.push(stream);
    }

    // Convert data attributes to warp filters
    for FnParam {
        token,
        key,
        typ,
        kind,
    } in data
    {
        let stream = match kind {
            Some(FnParamKind::Data) => {
                quote::quote! { and(warp::filters::any::any().map(move || #key.clone())) }
            }
            Some(FnParamKind::Form)
            | Some(FnParamKind::Header { .. })
            | Some(FnParamKind::Json)
            | Some(FnParamKind::Query) => {
                let spanned = syn::Error::new_spanned(token, "warp-macros bug: This should not happen, invalid partitioned enum variant: data");

                return Err(spanned.to_compile_error());
            }
            None => continue,
        };

        closure.push(quote::quote! { #key: #typ });
        routing.push(stream);
    }

    Ok((
        quote::quote! { #( #closure ),* },
        quote::quote! { #( . #routing )* },
    ))
}
