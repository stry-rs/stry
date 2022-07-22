mod handlers;

mod error;
mod templates;

use std::{net::SocketAddr, time::Duration};

use stry_backend_postgres::PostgresBackend;
use stry_common::{
    backend::ArcBackend,
    config::{self, Config},
    futures::utils::TryFutureExt as _,
    prelude::*,
    uri::Uri,
};

use axum::{error_handling::HandleErrorLayer, http::StatusCode, Extension, Router};
use tower::{BoxError, ServiceBuilder};
use tower_helmet::HelmetLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "stry_service_json=debug")
    }

    tracing_subscriber::registry()
        // .with(console_subscriber::spawn())
        .with(tracing_subscriber::fmt::layer().with_thread_ids(true))
        .init();

    let config = Config::load()
        .context("unable to initialize config")?
        .into_arc();

    if config.secret == config::DEFAULT_SECRET {
        warn!("DEFAULT SECRET KEY NOT OVERWRITTEN");
    }

    let uri = Uri::parse(&config.database).context("unable to parse database connection uri")?;

    let backend = match uri.scheme.as_str() {
        "postgres" => PostgresBackend::new(uri).map_ok(ArcBackend::new).await?,
        schema => bail!("`{}` is not a supported database", schema),
    };

    backend.migrate().await?;

    let app = Router::new().merge(handlers::routes()).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| async move {
                if error.is::<tower::timeout::error::Elapsed>() {
                    (StatusCode::REQUEST_TIMEOUT, String::new())
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, String::new())
                }
            }))
            .load_shed()
            .concurrency_limit(1024)
            .timeout(Duration::from_secs(10))
            // Content-Security-Policy = "script-src myhost.com 'unsafe-eval'"
            .layer(HelmetLayer::with_defaults())
            .layer(Extension(config.clone()))
            .layer(Extension(backend))
            .layer(TraceLayer::new_for_http().make_span_with(stry_common::http::make_span))
            .into_inner(),
    );

    let addr = SocketAddr::from((config.ip, config.port + 1));

    info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(stry_common::http::shutdown_signal())
        .await?;

    Ok(())
}
