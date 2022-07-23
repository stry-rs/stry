use stry_common::{
    backend::{ArcBackend, StoryEntity},
    http::Pagination,
    prelude::*,
};

use axum::{
    extract::{ContentLengthLimit, Query},
    response::{Html, IntoResponse},
    Extension,
};
use windswept::Render as _;

use crate::error::Error;

#[instrument(skip(data, query), err)]
pub async fn get(
    Extension(data): Extension<ArcBackend>,
    ContentLengthLimit(Query(query)): ContentLengthLimit<Query<Pagination>, { 1024 * 5000 }>,
) -> Result<impl IntoResponse, Error> {
    let stories = StoryEntity::all(&data, query.cursor, query.limit).await?;

    Ok(Html(crate::templates::page::index(&stories).render()?))
}
