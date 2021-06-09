use {
    hyper::{service::Service, Body, Request, Response},
    std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    },
};

pub struct AnalyticsService {}

impl AnalyticsService {
    async fn handle(&self) {}
}

impl Service<Request<Body>> for AnalyticsService {
    type Response = Response<Body>;

    type Error = hyper::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        todo!()
    }
}
