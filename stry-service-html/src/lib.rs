mod handlers;

mod error;
mod templates;

use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(handlers::routes())
}
