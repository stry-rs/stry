mod chapter;
mod story;

use stry_common::{
    backend::{ArcBackend, UserEntity},
    error::ErrorResponse,
    models::{
        core::{Account, User, UserRegisterForm},
        New,
    },
    prelude::{err, Validate as _},
};

use axum::{
    extract::{ContentLengthLimit, Extension, Json},
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use tower::limit::ConcurrencyLimitLayer;

use crate::error::Error;

pub fn router() -> Router {
    Router::new()
        // These have a limiter on them due to the global router allowing someone to flood these handlers causing the server to run out of memory
        .route(
            "/register",
            post(Handler::layer(register, ConcurrencyLimitLayer::new(32))),
        )
        .route(
            "/session",
            post(Handler::layer(session, ConcurrencyLimitLayer::new(128))),
        )
        //
        .route("/search", post(empty))
        //
        .route("/chapters", post(chapter::create))
        .route("/chapters/:id", get(chapter::get))
        .route("/stories", get(story::all).post(story::create))
        .route("/stories/:id", get(story::get))
}

async fn register(
    Extension(data): Extension<ArcBackend>,
    ContentLengthLimit(Json(form)): ContentLengthLimit<Json<UserRegisterForm>, { 1024 * 5000 }>,
) -> Result<impl IntoResponse, Error> {
    if let Err(err) = form.validate() {
        // TODO(txuritan): turn the `err` into a standard format rather than being based off the structure itself
        return Ok((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: err })).into_response());
    }

    let account =
        tokio::task::spawn_blocking(move || Account::new(form.username, form.email, form.password))
            .await
            .map_err(|err| err!(err))??;

    UserEntity::create(&data, New::from(User::new(account))).await?;

    Ok((StatusCode::CREATED, Json(serde_json::json!({}))).into_response())
}

async fn session(Extension(data): Extension<ArcBackend>) -> Result<impl IntoResponse, Error> {
    Ok((StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({}))).into_response())
}

async fn empty() {}
