use stry_common::{backend::arc::ArcBackend, models::core::UserRegisterForm};

use axum::{
    extract::{ContentLengthLimit, Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::error::Error;

mod chapter;
mod story;

pub fn router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/session", post(empty))
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
    // TODO(txuritan): convert any errors to an actually informative response

    data.register(form).await?;

    Ok(StatusCode::CREATED)
}

async fn empty() {}
