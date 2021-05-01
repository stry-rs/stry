use {
    crate::models::{FnParam, FnParamKind, GuardParam, Route, Triplet, UrlParam, UrlParams},
    proc_macro2::TokenStream,
    syn::{
        punctuated::Punctuated, Attribute, FnArg, Generics, Ident, ItemFn, LitStr, Pat, PatIdent,
        PatType, Signature, Type, TypePath,
    },
};

pub fn parse<'i>(
    path: &'i LitStr,
    path_value: &'i str,
    item: &'i ItemFn,
) -> Result<Route<'i>, TokenStream> {
    let asyncness = item.sig.asyncness.is_some();

    let guard_params = parse_guards(item)?;

    let url_params = parse_url(path, &path_value);
    let fn_params = parse_params(&item.sig.inputs)?;

    let clean = clean_fn(item)?;

    Ok(Route {
        asyncness,
        url_params,
        fn_params,
        guard_params,
        clean,
    })
}

fn parse_guards(item: &ItemFn) -> Result<Vec<GuardParam>, TokenStream> {
    let mut guards = Vec::new();

    for attr in &item.attrs {
        let attr_path = &attr.path;

        if attr_path.is_ident("header") {
            // Attempt to parse the attribute's body
            let Triplet { key, value, .. }: Triplet<LitStr, syn::Token![:], LitStr> = match attr
                .parse_args()
            {
                Ok(value) => value,
                Err(err) => {
                    let mut spanned = syn::Error::new_spanned(
                        &attr.tokens,
                        r#"[stry-attrouter] failed to parse attribute, example: #[header("Content-Type": "application/json")]"#,
                    );

                    spanned.combine(err);

                    return Err(spanned.to_compile_error());
                }
            };

            guards.push(GuardParam::Header { key, value });

            continue;
        }

        if attr_path.is_ident("body_size") {
            // Attempt to parse the attribute's body
            let Triplet { key, value, .. }: Triplet<Ident, syn::Token![=], LitStr> = match attr
                .parse_args()
            {
                Ok(value) => value,
                Err(err) => {
                    let mut spanned = syn::Error::new_spanned(
                        &attr.tokens,
                        r#"[stry-attrouter] failed to parse attribute, example: #[body_size(max = "1024")]"#,
                    );

                    spanned.combine(err);

                    return Err(spanned.to_compile_error());
                }
            };

            if key == "max" {
                guards.push(GuardParam::BodySize { key, value });
            } else {
                return Err(syn::Error::new_spanned(
                    &attr.tokens,
                    format!(r#"[stry-attrouter] failed to parse attribute, invalid type `{}`, example: #[body_size(max = "1024")]"#, key),
                ).to_compile_error());
            }

            continue;
        }

        // TODO: CORS
    }

    Ok(guards)
}

// Convert the route into chunks
// Empty will be handled as `/`
fn parse_url<'u>(token: &'u LitStr, url: &'u str) -> UrlParams<'u> {
    let items = if url.contains('{') {
        // Handles routes with arguments
        url.match_indices('{')
            .zip(url.match_indices('}'))
            .fold(
                (0, Vec::with_capacity(16)),
                |(last, mut parts), ((start, _), (end, _))| {
                    // Converts any route data between the last and the current argument
                    parts.extend(
                        (&url[last..start])
                            .split('/')
                            .filter(|p| !p.is_empty())
                            .map(|value| UrlParam::Chunk { value }),
                    );

                    // Grabs the name of the current argument
                    parts.push(UrlParam::Param {
                        key: &url[(start + 1)..end],
                    });

                    (end + 1, parts)
                },
            )
            .1
    } else {
        // Handles routes without arguments
        url.split('/')
            .filter(|p| !p.is_empty())
            .map(|value| UrlParam::Chunk { value })
            .collect()
    };

    UrlParams { token, items }
}

fn parse_params<'p>(
    params: &'p Punctuated<FnArg, syn::Token![,]>,
) -> Result<Vec<FnParam<'p>>, TokenStream> {
    let mut mapped = Vec::with_capacity(params.len());

    for param in params {
        match param {
            FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                // Extract needed type data
                let key = match &**pat {
                    Pat::Ident(PatIdent { ident, .. }) => ident,
                    pat => {
                        return Err(syn::Error::new_spanned(
                            pat,
                            "[stry-attrouter] Unsupported function argument pattern type",
                        )
                        .to_compile_error());
                    }
                };

                let typ = match &**ty {
                    Type::Path(TypePath { path, .. }) => path,
                    typ => {
                        return Err(syn::Error::new_spanned(
                            typ,
                            "[stry-attrouter] Unsupported function argument type",
                        )
                        .to_compile_error());
                    }
                };

                let mut kind = None;

                // Check for supported attributes
                for attr in attrs {
                    let attr_path = &attr.path;

                    if attr_path.is_ident("data") {
                        kind = Some(FnParamKind::Data);

                        break;
                    }

                    if attr_path.is_ident("form") {
                        kind = Some(FnParamKind::Form);

                        break;
                    }

                    if attr_path.is_ident("header") {
                        let header = attr.parse_args().map_err(|err| err.to_compile_error())?;

                        kind = Some(FnParamKind::Header { header });

                        break;
                    }

                    if attr_path.is_ident("query") {
                        kind = Some(FnParamKind::Query);

                        break;
                    }

                    if attr_path.is_ident("json") {
                        kind = Some(FnParamKind::Json);

                        break;
                    }
                }

                mapped.push(FnParam {
                    token: param,

                    key,
                    typ,

                    kind,
                });
            }
            FnArg::Receiver(_) => {
                return Err(syn::Error::new_spanned(
                    param,
                    "[stry-attrouter] Routing macro does not allow for self referencing functions",
                )
                .to_compile_error());
            }
        }
    }

    Ok(mapped)
}

pub fn is_attr(attr: &Attribute) -> bool {
    for name in &["data", "form", "header", "json", "query"] {
        if attr.path.is_ident(name) {
            return true;
        }
    }

    false
}

fn clean_fn(item: &ItemFn) -> Result<ItemFn, TokenStream> {
    let block = &item.block;

    let Signature {
        asyncness,
        unsafety,
        generics,
        output,
        inputs,
        ..
    } = &item.sig;

    // The custom attributes need to be removed be I rebuild the function
    let new_inputs: Punctuated<FnArg, syn::Token![,]> = clean_inputs(inputs)?;

    let Generics {
        params,
        where_clause,
        ..
    } = generics;

    let cleaned = quote::quote! {
        #[inline(always)]
        #unsafety #asyncness fn inner #params ( #new_inputs ) #output #where_clause #block
    };

    let item = syn::parse2::<ItemFn>(cleaned).map_err(|err| err.to_compile_error())?;

    Ok(item)
}

// Remove any custom attributes from function inputs
fn clean_inputs(
    inputs: &Punctuated<FnArg, syn::Token![,]>,
) -> Result<Punctuated<FnArg, syn::Token![,]>, TokenStream> {
    let mut cleaned: Punctuated<FnArg, syn::Token![,]> = inputs.clone();

    for mut pair in cleaned.pairs_mut() {
        let value = pair.value_mut();

        match value {
            FnArg::Receiver(_) => {
                return Err(syn::Error::new_spanned(
                    value,
                    "[stry-attrouter] Routing macro does not allow for self referencing functions",
                )
                .to_compile_error())
            }
            FnArg::Typed(PatType { attrs, .. }) => {
                *attrs = attrs.clone().into_iter().partition(is_attr).1;
            }
        }
    }

    Ok(cleaned)
}
