mod extractors;
mod layers;
mod provider;
mod v1;

mod error;
mod utils;

use axum::Router;

pub fn routes() -> Router {
    Router::new().nest("/v1", v1::router())
}
