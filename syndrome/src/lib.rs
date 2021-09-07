use {
    chrono::Utc,
    futures_util::{
        future::{self, Either, Inspect, Map, Ready, UnwrapOrElse},
        FutureExt as _, TryFutureExt as _,
    },
    hyper::{server::conn::AddrStream, service::Service},
    path_tree::PathTree,
    rand::{rngs::OsRng, RngCore as _},
    std::{
        collections::HashMap,
        convert::Infallible,
        future::Future,
        io::{Cursor, Write as _},
        net::SocketAddr,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll},
        time::{self, SystemTime},
    },
    tracing::{instrument::Instrumented, Instrument as _},
};

pub use hyper::{header, http, Body, Method, Server, StatusCode};

pub type Response = hyper::Response<Body>;
pub type Request = hyper::Request<Body>;

type BoxedFuture<Output> = Pin<Box<dyn Future<Output = Output> + Send + 'static>>;

type HandlerResponse = BoxedFuture<anyhow::Result<Response>>;

pub trait Handler<D>: Send + Sync {
    fn handle(&self, data: Data<D>, req: Request, params: Params) -> HandlerResponse;
}

impl<D, F, R> Handler<D> for F
where
    F: Fn(Data<D>, Request, Params) -> R + Send + Sync + 'static,
    R: Future<Output = anyhow::Result<Response>> + Send + 'static,
{
    fn handle(&self, data: Data<D>, req: Request, params: Params) -> HandlerResponse {
        Box::pin(self(data, req, params))
    }
}

pub struct Data<D> {
    inner: Arc<D>,
}

impl<D> Clone for Data<D> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<D> std::ops::Deref for Data<D> {
    type Target = Arc<D>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// TODO: figure out a way to get this to use lifetimes, i keep getting `E0308` `one type is more general than the other`
// expected associated type `<for<'p> fn(hyper::Request<syndrome::Body>, &'p [(&'p str, &'p str)]) -> impl for<'p> std::future::Future {story::get} as std::ops::FnOnce<(hyper::Request<syndrome::Body>, &[(&str, &str)])>>::Output`
//    found associated type `<for<'p> fn(hyper::Request<syndrome::Body>, &'p [(&'p str, &'p str)]) -> impl for<'p> std::future::Future {story::get} as std::ops::FnOnce<(hyper::Request<syndrome::Body>, &'p [(&'p str, &'p str)])>>::Output`
pub struct Params(Vec<(String, String)>);

impl Params {
    pub fn get(&self, key: &str) -> Option<&String> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

pub struct Syndrome<D> {
    data: Data<D>,
    #[allow(clippy::type_complexity)]
    internal: Arc<HashMap<Method, PathTree<Box<dyn Handler<D>>>>>,
}

impl<D> Syndrome<D> {
    pub fn builder(data: D) -> SyndromeBuilder<D> {
        SyndromeBuilder {
            data: Data {
                inner: Arc::new(data),
            },
            internal: HashMap::new(),
        }
    }

    pub fn service(self) -> MakeSyndromeService<D> {
        MakeSyndromeService { map: self }
    }
}

impl<D> Clone for Syndrome<D> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            internal: self.internal.clone(),
        }
    }
}

pub struct SyndromeBuilder<D> {
    data: Data<D>,
    internal: HashMap<Method, PathTree<Box<dyn Handler<D>>>>,
}

impl<D> SyndromeBuilder<D> {
    pub fn insert<H>(&mut self, method: Method, route: &'static str, handler: H)
    where
        H: Handler<D> + 'static,
    {
        self.internal
            .entry(method)
            .or_insert_with(PathTree::new)
            .insert(route, Box::new(handler));
    }

    pub fn finish(self) -> Syndrome<D> {
        Syndrome {
            data: self.data,
            internal: Arc::new(self.internal),
        }
    }
}

pub struct MakeSyndromeService<D> {
    map: Syndrome<D>,
}

impl<D> Service<&AddrStream> for MakeSyndromeService<D> {
    type Response = SyndromeService<D>;

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

type ServiceInspect = Inspect<HandlerResponse, fn(&anyhow::Result<Response>)>;
type ServiceUnwrap = UnwrapOrElse<ServiceInspect, ServiceFnErrorMap>;
type ServiceMap = Map<ServiceUnwrap, ServiceFnMap>;

type ServiceLeftFuture = Instrumented<ServiceMap>;
type ServiceRightFuture = Ready<Result<Response, Infallible>>;

pub struct SyndromeService<D> {
    map: Syndrome<D>,
    addr: SocketAddr,
}

impl<D> Service<Request> for SyndromeService<D> {
    type Response = Response;

    type Error = Infallible;

    type Future = Either<ServiceLeftFuture, ServiceRightFuture>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let now = Utc::now().time();

        let map = self.map.clone();

        let span = tracing::info_span!(
            "request",
            remote.addr = %self.addr,
            method = %req.method(),
            path = %req.uri().path(),
            version = ?req.version(),
            accept = tracing::field::Empty,
            referer = tracing::field::Empty,
            user_agent = tracing::field::Empty,
            id = %random_id(),
        );

        let headers = req.headers();

        fn set_record(
            span: &tracing::Span,
            headers: &http::HeaderMap,
            field: &str,
            header: http::header::HeaderName,
        ) {
            if let Some(Ok(referer)) = headers.get(header).map(|value| value.to_str()) {
                span.record(field, &tracing::field::display(referer));
            }
        }

        set_record(&span, headers, "accept", header::ACCEPT);
        set_record(&span, headers, "referer", header::REFERER);
        set_record(&span, headers, "user_agent", header::USER_AGENT);

        tracing::debug!(parent: &span, "received request");

        fn service_map_err(_: anyhow::Error) -> Response {
            let mut res = Response::new(Body::empty());

            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

            res
        }

        #[allow(clippy::unnecessary_wraps)]
        fn service_map(res: Response) -> Result<Response, Infallible> {
            Ok(res)
        }

        fn inspector(ret: &anyhow::Result<Response>) {
            match ret {
                Ok(res) => {
                    tracing::info!(
                        status = %res.status().as_u16(),
                        "finished processing with success",
                    )
                }
                Err(err) => {
                    tracing::error!(
                        status = %StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        error = ?err,
                        "unable to process request",
                    );
                }
            }
        }

        let uri_path = req.uri().path().to_string();

        let handler = map
            .internal
            .get(req.method())
            .and_then(|map| map.find(&uri_path));

        if let Some((handler, params)) = handler {
            tracing::info!(parent: &span, "processing request");

            let params = params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();

            let handle = handler.handle(self.map.data.clone(), req, Params(params));
            let inspect: ServiceInspect = handle.inspect(inspector);
            let unwrapped: ServiceUnwrap = inspect.unwrap_or_else(service_map_err);
            let mapped: ServiceMap = unwrapped.map(service_map);
            let ret: ServiceLeftFuture = mapped.instrument(span);

            Either::Left(ret)
        } else {
            // TODO logging for this branch
            let mut res = Response::new(Body::empty());

            *res.status_mut() = StatusCode::NOT_FOUND;

            Either::Right(future::ok(res))
        }
    }
}

fn random_id() -> String {
    let length = 32;

    let byte_length: usize = (length / 4) * 3;

    let mut raw: Vec<u8> = vec![0; byte_length];

    {
        let now = SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();

        let secs: u64 = now.as_secs();
        let nano_secs: u32 = now.subsec_nanos();

        let mut cursor = Cursor::new(&mut *raw);

        cursor.write_all(&nano_secs.to_le_bytes()).unwrap();
        cursor.write_all(&secs.to_le_bytes()).unwrap();
    }

    OsRng.fill_bytes(&mut raw[12..byte_length]);

    base64::encode_config(&raw, base64::STANDARD)
}
