mod extractors;
mod layers;
mod provider;
mod v1;

mod error;
mod utils;

use std::{
    io::{Cursor, Write},
    net::SocketAddr,
    time::{self, Duration, SystemTime},
};

use stry_backend_postgres::PostgresBackendFactory;
use stry_common::{
    backend::{arc::ArcBackend, BackendFactory as _},
    config::{Config, DEFAULT_SECRET},
    futures::utils::TryFutureExt as _,
    layered::{Anulap, EnvSource},
    prelude::*,
    uri::Uri,
};

use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    http::{Request, StatusCode},
    AddExtensionLayer, Router,
};
use rand::{rngs::OsRng, RngCore as _};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "stry_service_json=debug")
    }

    tracing_subscriber::fmt().with_thread_ids(true).init();

    let anulap = Anulap::new().with(EnvSource::new());

    let config = anulap
        .init::<Config>()
        .context("unable to initialize config")?;
    let config = config.into_arc();

    if config.secret == DEFAULT_SECRET {
        warn!("DEFAULT SECRET KEY NOT OVERWRITTEN");
    }

    let uri = Uri::parse(&config.database).context("unable to parse database connection uri")?;

    let backend = match uri.scheme.as_str() {
        "postgres" => {
            PostgresBackendFactory
                .create(uri)
                .map_ok(ArcBackend::new)
                .await?
        }
        schema => bail!("`{}` is not a supported database", schema),
    };

    backend.migrate().await?;

    let app = Router::new().nest("/v1", v1::router()).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| {
                if error.is::<tower::timeout::error::Elapsed>() {
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                }
            }))
            .load_shed()
            .concurrency_limit(1024)
            .timeout(Duration::from_secs(10))
            .layer(AddExtensionLayer::new(config.clone()))
            .layer(AddExtensionLayer::new(backend))
            .layer(TraceLayer::new_for_http().make_span_with(make_span))
            .into_inner(),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3002));

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn make_span(req: &Request<Body>) -> Span {
    let span = debug_span!(
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
        headers: &axum::http::HeaderMap,
        field: &str,
        header: axum::http::header::HeaderName,
    ) {
        if let Some(Ok(referer)) = headers.get(header).map(|value| value.to_str()) {
            span.record(field, &field::display(referer));
        }
    }

    set_record(&span, headers, "accept", axum::http::header::ACCEPT);
    set_record(&span, headers, "referer", axum::http::header::REFERER);
    set_record(&span, headers, "user_agent", axum::http::header::USER_AGENT);

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

#[cfg(unix)]
async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;

    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;

        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    info!("signal received, starting graceful shutdown");
}

#[cfg(windows)]
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");

    info!("signal received, starting graceful shutdown");
}
