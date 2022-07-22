use stry_common::{
    backend::{ArcBackend, StoryEntity},
    http::Pagination,
    models::{story::Story, Existing},
    prelude::*,
};

use askama::Template;
use axum::{
    extract::{ContentLengthLimit, Query},
    response::{Html, IntoResponse},
    Extension,
};

use crate::{
    error::Error,
    templates::{HeaderSegments, SEGMENTS},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Page {
    segments: HeaderSegments,
    selected: String,
    stories: Vec<Existing<Story>>,
}

#[instrument(skip(data, query), err)]
pub async fn get(
    Extension(data): Extension<ArcBackend>,
    ContentLengthLimit(Query(query)): ContentLengthLimit<Query<Pagination>, { 1024 * 5000 }>,
) -> Result<impl IntoResponse, Error> {
    let stories = StoryEntity::all(&data, query.cursor, query.limit).await?;

    Ok(Html(
        Page {
            segments: SEGMENTS,
            selected: "".to_string(),
            stories,
        }
        .render()?,
    ))
}
