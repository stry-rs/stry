use {
    futures_util::{
        future::{self, Either, Map, Ready, UnwrapOrElse},
        FutureExt as _, TryFutureExt as _,
    },
    hyper::{server::conn::AddrStream, service::Service},
    std::{
        borrow::Cow,
        collections::HashMap,
        convert::Infallible,
        future::Future,
        net::SocketAddr,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
    },
    tracing::{instrument::Instrumented, Instrument as _},
};

pub use hyper::{Body, Method, StatusCode};

pub type Response = hyper::Response<Body>;
pub type Request = hyper::Request<Body>;

type BoxedFuture<Output> = Pin<Box<dyn Future<Output = Output> + Send + Sync + 'static>>;

type HandlerResponse = BoxedFuture<anyhow::Result<Response>>;

pub trait Handler: Send + Sync {
    fn handle(&self, req: Request) -> HandlerResponse;
}

impl<F, R> Handler for F
where
    F: Fn(Request) -> R + Send + Sync + 'static,
    R: Future<Output = anyhow::Result<Response>> + Send + Sync + 'static,
{
    fn handle(&self, req: Request) -> HandlerResponse {
        Box::pin(self(req))
    }
}

#[derive(Clone)]
pub struct Syndrome {
    #[allow(clippy::type_complexity)]
    internal: Arc<HashMap<Method, HashMap<Cow<'static, str>, Box<dyn Handler>>>>,
}

impl Syndrome {
    pub fn builder() -> SyndromeBuilder {
        SyndromeBuilder {
            internal: HashMap::new(),
        }
    }

    pub fn service(self) -> MakeSyndromeService {
        MakeSyndromeService { map: self }
    }
}

pub struct SyndromeBuilder {
    internal: HashMap<Method, HashMap<Cow<'static, str>, Box<dyn Handler>>>,
}

impl SyndromeBuilder {
    pub fn insert<F, R>(&mut self, method: Method, route: &'static str, handler: F)
    where
        F: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = anyhow::Result<Response>> + Send + Sync + 'static,
    {
        self.internal
            .entry(method)
            .or_insert_with(HashMap::new)
            .insert(Cow::from(route), Box::new(handler));
    }

    pub fn finish(self) -> Syndrome {
        Syndrome {
            internal: Arc::new(self.internal),
        }
    }
}

pub struct MakeSyndromeService {
    map: Syndrome,
}

impl Service<&AddrStream> for MakeSyndromeService {
    type Response = SyndromeService;

    type Error = Infallible;

    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, conn: &AddrStream) -> Self::Future {
        future::ok(SyndromeService {
            map: self.map.clone(),
            addr: conn.remote_addr(),
        })
    }
}

type ServiceFnErrorMap = fn(anyhow::Error) -> Response;
type ServiceFnMap = fn(Response) -> Result<Response, Infallible>;

type ServiceUnwrap = UnwrapOrElse<HandlerResponse, ServiceFnErrorMap>;
type ServiceMap = Map<ServiceUnwrap, ServiceFnMap>;

type ServiceLeftFuture = Instrumented<ServiceMap>;
type ServiceRightFuture = Ready<Result<Response, Infallible>>;

pub struct SyndromeService {
    map: Syndrome,
    addr: SocketAddr,
}

impl Service<Request> for SyndromeService {
    type Response = Response;

    type Error = Infallible;

    type Future = Either<ServiceLeftFuture, ServiceRightFuture>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let map = self.map.clone();
        let addr = self.addr;

        let span = tracing::info_span!(
            "request",
            remote.addr = %addr,
            method = %req.method(),
            path = %req.uri().path(),
            version = ?req.version(),
            referer = tracing::field::Empty,
        );

        let headers = req.headers();

        if let Some(Ok(referer)) = headers
            .get(hyper::header::REFERER)
            .map(|value| value.to_str())
        {
            span.record("referer", &tracing::field::display(referer));
        }

        fn service_map_err(error: anyhow::Error) -> Response {
            for err in error.chain() {
                tracing::error!("{:?}", err);
            }

            let mut res = Response::new(Body::empty());

            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

            res
        }

        #[allow(clippy::unnecessary_wraps)]
        fn service_map(res: Response) -> Result<Response, Infallible> {
            Ok(res)
        }

        let handler = map
            .internal
            .get(req.method())
            .and_then(|map| map.get(req.uri().path()));

        if let Some(handler) = handler {
            let handle = handler.handle(req);
            let unwrapped: ServiceUnwrap = handle.unwrap_or_else(service_map_err);
            let mapped: ServiceMap = unwrapped.map(service_map);
            let ret: ServiceLeftFuture = mapped.instrument(span);

            Either::Left(ret)
        } else {
            let mut res = Response::new(Body::empty());

            *res.status_mut() = StatusCode::NOT_FOUND;

            Either::Right(future::ok(res))
        }
    }
}
