use syn::{FnArg, Ident, ItemFn, LitStr, Path};

pub enum Method {
    Delete,
    Get,
    Head,
    Options,
    Patch,
    Post,
    Put,
}

pub struct Triplet<K, S, V> {
    pub key: K,
    pub _sep: S,
    pub value: V,
}

impl<K, S, V> syn::parse::Parse for Triplet<K, S, V>
where
    K: syn::parse::Parse,
    S: syn::parse::Parse,
    V: syn::parse::Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            _sep: input.parse()?,
            value: input.parse()?,
        })
    }
}

pub struct UrlParams<'u> {
    pub token: &'u LitStr,
    pub items: Vec<UrlParam<'u>>,
}

pub enum UrlParam<'u> {
    Chunk { value: &'u str },
    Param { key: &'u str },
}

pub enum GuardParam {
    BodySize { key: Ident, value: LitStr },
    Header { key: LitStr, value: LitStr },
}

pub struct FnParam<'p> {
    pub token: &'p FnArg,

    pub key: &'p Ident,
    pub typ: &'p Path,

    pub kind: Option<FnParamKind>,
}

pub enum FnParamKind {
    Data,
    Form,
    Header { header: LitStr },
    Json,
    Query,
}

pub struct Route<'i> {
    pub asyncness: bool,
    pub url_params: UrlParams<'i>,
    pub fn_params: Vec<FnParam<'i>>,
    pub guard_params: Vec<GuardParam>,
    pub clean: ItemFn,
}
