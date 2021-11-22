use std::task::{Context, Poll};

use axum::{body::HttpBody, http::Request};
use headers::Header;
use tower::{Layer, Service};

pub struct EnsureHeaderLayer<T: Header + Clone>(pub T);

impl<T: Header + Clone, S> Layer<S> for EnsureHeaderLayer<T> {
    type Service = EnsureHeaderService<T, S>;

    fn layer(&self, inner: S) -> Self::Service {
        EnsureHeaderService { header: self.0.clone(), service: inner, }
    }
}

pub struct EnsureHeaderService<T: Header + Clone, S>{
    header: T,
    service: S,
}

impl<T: Header + Clone, S, RequestBody> Service<Request<RequestBody>> for EnsureHeaderService<T, S>
where
    S: Service<Request<RequestBody>>,
    RequestBody: HttpBody,
{
    type Response = S::Response;

    type Error = S::Error;

    type Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Request<RequestBody>) -> Self::Future {
        todo!()
    }
}
