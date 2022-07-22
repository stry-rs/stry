mod index;
mod resources;

use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::get))
        .nest("/assets", resources::routes())
}
