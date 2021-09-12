pub struct Params<'u> {
    index: usize,
    array: [::std::mem::MaybeUninit<(&'static str, &'u str)>; 1],
}
impl<'u> Params<'u> {
    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&'u str> {
        fn inner<'u>(params: &Params<'u>, key: &str) -> Option<&'u str> {
            for entry in &params.array[0..params.index] {
                let (k, v) = unsafe { entry.assume_init() };
                if k == key {
                    return Some(v);
                }
            }
            None
        }
        inner(self, key.as_ref())
    }
}
pub struct Data<D> {
    inner: ::std::sync::Arc<D>,
}
impl<D> Data<D> {
    pub fn new(data: D) -> Self {
        Self {
            inner: ::std::sync::Arc::new(data),
        }
    }
}
impl<D> Clone for Data<D> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
impl<D> std::ops::Deref for Data<D> {
    type Target = ::std::sync::Arc<D>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
pub type Response = ::hyper::Response<::hyper::Body>;
pub type Request = ::hyper::Request<::hyper::Body>;
pub type Return<'u> = std::boxed::Box<
    dyn Fn(
            Data<::stry_common::backend::boxed::BoxedBackend>,
            Request,
            Params<'u>,
        ) -> ::std::pin::Pin<
            std::boxed::Box<
                dyn ::std::future::Future<
                        Output = ::std::result::Result<Response, ::stry_common::prelude::Error>,
                    > + Send
                    + 'u,
            >,
        > + Send
        + Sync
        + 'u,
>;
#[allow(dead_code, unreachable_patterns, unused_variables, clippy::manual_map)]
pub fn find<'u>(method: &::hyper::Method, url: &'u str) -> Option<(Params<'u>, Return<'u>)> {
    impl<'u> Params<'u> {
        fn new() -> Self {
            Self {
                index: 0,
                array: unsafe { ::std::mem::MaybeUninit::uninit().assume_init() },
            }
        }

        fn insert(&mut self, key: &'static str, value: &'u str) {
            {
                let entry = &mut self.array[self.index];
                *entry = ::std::mem::MaybeUninit::new((key, value));
            }
            self.index += 1;
        }
    }
    let mut parts = url.split('/').filter(|p| !p.is_empty());
    let mut params: Params<'u> = Params::new();
    match *method {
        ::hyper::Method::POST => match parts.next() {
            Some("story") => Some((
                params,
                Box::new(|d, r, p| crate::story::ApiStory::create(d, r, p)),
            )),
            _ => None,
        },
        ::hyper::Method::GET => match parts.next() {
            Some("story") => match parts.next() {
                Some(r#id) => {
                    params.insert("id", r#id);
                    Some((
                        params,
                        Box::new(|d, r, p| crate::story::ApiStory::get(d, r, p)),
                    ))
                }
                None => Some((
                    params,
                    Box::new(|d, r, p| crate::story::ApiStory::all(d, r, p)),
                )),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}
