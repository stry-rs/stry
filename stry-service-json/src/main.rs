mod story;

use std::net::SocketAddr;

use hyper::Server;
use stry_backend_postgres::PostgresBackendFactory;
use stry_common::{
    backend::{boxed::BoxedBackend, BackendFactory as _},
    config::Config,
    layered::{Anulap, EnvSource},
    prelude::*,
    uri::Uri,
};
use syndrome::{Syndrome, SyndromeBuilder};

type Data = syndrome::Data<BoxedBackend>;

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

            BoxedBackend::new(backend)
        }
        schema => bail!("`{}` is not a supported database", schema),
    };

    let mut router = Syndrome::builder(backend);

    story::ApiStory.configure(&mut router);

    let router = router.finish();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = Server::bind(&addr).serve(router.service());

    info!("Server listening on {}", addr);

    if let Err(e) = server.await {
        error!("server error: {}", e);
    }

    Ok(())
}

async fn handle<F, R, T>(f: F) -> Result<syndrome::Response, Error>
where
    F: FnOnce() -> R,
    R: std::future::Future<Output = Result<T, Error>>,
    T: serde::Serialize,
{
    Ok({
        let mut res =
            syndrome::Response::new(syndrome::Body::from(serde_json::to_vec(&f().await?)?));

        res.headers_mut().insert(
            syndrome::header::CONTENT_TYPE,
            syndrome::header::HeaderValue::from_static("application/json; charset=utf-8"),
        );

        res
    })
}

trait Api {
    fn configure(&self, router: &mut SyndromeBuilder<BoxedBackend>);
}
