mod router;
mod story;
mod utils;

use std::{
    io::{Cursor, Write},
    net::SocketAddr,
    time::{self, SystemTime},
};

use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use rand::{rngs::OsRng, RngCore};
use stry_backend_postgres::PostgresBackendFactory;
use stry_common::{
    backend::{arc::ArcBackend, BackendFactory as _},
    config::Config,
    layered::{Anulap, EnvSource},
    prelude::*,
    uri::Uri,
};

type Data = router::Data<ArcBackend>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().with_thread_ids(true).init();

    let anulap = Anulap::new().with(EnvSource::new());

    let config = anulap
        .init::<Config>()
        .context("unable to initialize config")?;

    let uri: Uri =
        Uri::parse(&config.database).context("unable to parse database connection uri")?;

    let backend = match uri.scheme.as_str() {
        "postgres" => {
            let backend = PostgresBackendFactory.create(uri).await?;

            ArcBackend::new(backend)
        }
        schema => bail!("`{}` is not a supported database", schema),
    };

    backend.migrate().await?;

    let data = router::Data::new(backend);

    let make_service = make_service_fn(|_conn| {
        let data = data.clone();

        async move {
            let data = data.clone();

            Ok::<_, std::convert::Infallible>(service_fn(move |req: router::Request| {
                let data = data.clone();

                async move {
                    let then = Utc::now().time();

                    let span = build_span(&req);

                    debug!(parent: &span, "received request");

                    let method = req.method();

                    let url = req.uri().clone();
                    let path = url.path();

                    let res = if let Some((params, handler)) = router::find(method, path) {
                        info!(parent: &span, "processing request");

                        match (handler)(data.clone(), req, params)
                            .instrument(info_span!(parent: &span, "request handler"))
                            .await
                        {
                            Ok(res) => Some(res),
                            Err(err) => {
                                error!(parent: &span, error = ?err, "error handling request");

                                Some(utils::responses::internal_server_error())
                            }
                        }
                    } else {
                        None
                    };

                    Ok::<_, std::convert::Infallible>({
                        let res = res.unwrap_or_else(utils::responses::not_found);

                        let now = Utc::now().time();

                        let time = then.signed_duration_since(now);

                        info!(
                            parent: &span,
                            http.status = %res.status(),
                            time.seconds = time.num_seconds(),
                            time.milliseconds = time.num_milliseconds(),
                            "finished request"
                        );

                        res
                    })
                }
            }))
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));

    let server = Server::bind(&addr).serve(make_service);

    info!("Server listening on {}", addr);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}

fn build_span(req: &router::Request) -> Span {
    let span = info_span!(
        "request",
        method = %req.method(),
        path = %req.uri().path(),
        version = ?req.version(),
        accept = field::Empty,
        referer = field::Empty,
        user_agent = field::Empty,
        id = %random_id(),
    );

    let headers = req.headers();

    fn set_record(
        span: &Span,
        headers: &hyper::http::HeaderMap,
        field: &str,
        header: hyper::http::header::HeaderName,
    ) {
        if let Some(Ok(referer)) = headers.get(header).map(|value| value.to_str()) {
            span.record(field, &field::display(referer));
        }
    }

    set_record(&span, headers, "accept", hyper::header::ACCEPT);
    set_record(&span, headers, "referer", hyper::header::REFERER);
    set_record(&span, headers, "user_agent", hyper::header::USER_AGENT);

    span
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
