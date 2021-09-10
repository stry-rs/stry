use std::convert::TryFrom;

use stry_common::{
    backend::{boxed::BoxedBackend, BackendEntry},
    models::{story::Story, Id},
    prelude::*,
};
use syndrome::{Params, Request, Response, SyndromeBuilder};

use crate::{
    handle,
    utils::{self, Query},
    Api, Data,
};

pub struct ApiStory;

impl ApiStory {
    async fn get(data: Data, _req: Request, params: Params) -> Result<Response, Error> {
        handle(move || async move {
            let id = params.get("id").context("missing `id` from the path")?;

            let id = Id::try_from(id.as_str())?;

            BackendEntry::<Story>::get(&**data, id).await
        })
        .await
    }

    async fn all(data: Data, req: Request, _params: Params) -> Result<Response, Error> {
        handle(move || async move {
            let query = if let Some(query) = req.uri().query() {
                serde_urlencoded::from_str::<Query>(query)?
            } else {
                Query::default()
            };

            BackendEntry::<Story>::all(&**data, query.cursor, query.limit).await
        })
        .await
    }

    async fn create(data: Data, req: Request, _params: Params) -> Result<Response, Error> {
        if let Some(res) = utils::guard::content_type(&req, "application/json").await {
            return Ok(res);
        }

        handle(move || async move {
            // let bytes = hyper::body::to_bytes(req.into_body()).await?;

            Ok("not implemented")
        })
        .await
    }
}

impl Api for ApiStory {
    fn configure(&self, router: &mut SyndromeBuilder<BoxedBackend>) {
        router.get("/story/:id", Self::get);
        router.get("/story", Self::all);
        router.post("/story", Self::create);
    }
}
