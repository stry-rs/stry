use stry_common::{
    backend::{ArcBackend, ChapterEntity},
    config::ArcConfig,
    models::{story::Chapter, Id, New},
};

use axum::{
    extract::{ContentLengthLimit, Extension, Json, Path, TypedHeader},
    response::IntoResponse,
};
use biscuit::{jwa::SignatureAlgorithm, jws::Secret, ValidationOptions, JWT};
use headers::{authorization::Bearer, Authorization};

use crate::error::Error;

pub async fn get(
    Extension(data): Extension<ArcBackend>,
    Path(id): Path<Id>,
) -> Result<impl IntoResponse, Error> {
    Ok(Json(ChapterEntity::get(&data, id).await?))
}

pub async fn create(
    Extension(config): Extension<ArcConfig>,
    Extension(data): Extension<ArcBackend>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    ContentLengthLimit(Json(chapter)): ContentLengthLimit<Json<New<Chapter>>, { 1024 * 5000 }>,
) -> Result<impl IntoResponse, Error> {
    let token = JWT::<biscuit::Empty, biscuit::Empty>::new_encoded(authorization.0.token())
        .into_decoded(
            &Secret::bytes_from_str(&config.secret),
            SignatureAlgorithm::HS256,
        )
        .map_err(Error::from_any)?;

    token
        .validate(ValidationOptions::default())
        .map_err(Error::from_any)?;

    // TODO(txuritan): validate that the token is in the database and to retrieve the user id

    Ok(Json(ChapterEntity::create(&data, chapter).await?))
}
