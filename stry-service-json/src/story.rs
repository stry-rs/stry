use std::convert::TryFrom;

use stry_common::{
    backend::BackendEntry,
    models::{story::Story, Id},
    prelude::*,
};

use crate::{
    router::{Params, Request, Response},
    utils::{self, handle, Query},
    Data,
};

pub struct ApiStory;

#[stry_common::prelude::box_async]
impl ApiStory {
    pub async fn get(data: Data, _req: Request, params: Params<'_>) -> Result<Response, Error> {
        handle(move || async move {
            let id = params.get("id").context("missing `id` from the path")?;

            let id = Id::try_from(id)?;

            BackendEntry::<Story>::get(&**data, id).await
        })
        .await
    }

    pub async fn all(data: Data, req: Request, _params: Params<'_>) -> Result<Response, Error> {
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

    pub async fn create(data: Data, req: Request, _params: Params<'_>) -> Result<Response, Error> {
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

// impl Api for ApiStory {
//     fn configure(&self, router: &mut SyndromeBuilder<BoxedBackend>) {
//         router.get("/story/:id", Self::get);
//         router.get("/story", Self::all);
//         router.post("/story", Self::create);
//     }
// }
