mod handlers;
mod templates;

mod error;

use axum::Router;

pub fn routes() -> Router {
    Router::new().merge(handlers::routes())
}
