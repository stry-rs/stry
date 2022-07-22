use stry_common::{
    backend::{ArcBackend, StoryEntity},
    config::ArcConfig,
    http::Pagination,
    models::{story::Story, Id, New},
};

use axum::{
    extract::{ContentLengthLimit, Extension, Json, Path, Query, TypedHeader},
    response::IntoResponse,
};
use biscuit::{jwa::SignatureAlgorithm, jws::Secret, ValidationOptions, JWT};
use headers::{authorization::Bearer, Authorization};

use crate::error::Error;

pub async fn get(
    Extension(data): Extension<ArcBackend>,
    Path(id): Path<Id>,
) -> Result<impl IntoResponse, Error> {
    Ok(Json(StoryEntity::get(&data, id).await?))
}

pub async fn all(
    Extension(data): Extension<ArcBackend>,
    ContentLengthLimit(Query(query)): ContentLengthLimit<Query<Pagination>, { 1024 * 5000 }>,
) -> Result<impl IntoResponse, Error> {
    Ok(Json(
        StoryEntity::all(&data, query.cursor, query.limit).await?,
    ))
}

pub async fn create(
    Extension(config): Extension<ArcConfig>,
    Extension(data): Extension<ArcBackend>,
    TypedHeader(authorization): TypedHeader<Authorization<Bearer>>,
    ContentLengthLimit(Json(story)): ContentLengthLimit<Json<New<Story>>, { 1024 * 5000 }>,
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

    Ok(Json(StoryEntity::create(&data, story).await?))
}
