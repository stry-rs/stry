use std::convert::TryFrom;

use stry_common::{
    backend::{boxed::BoxedBackend, BackendEntry},
    models::{story::Story, Id},
    prelude::*,
};
use syndrome::{Method, Params, Request, Response, SyndromeBuilder};

use crate::{handle, Api, Data};

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
}

impl Api for ApiStory {
    fn configure(&self, router: &mut SyndromeBuilder<BoxedBackend>) {
        router.insert(Method::GET, "/api/story/:id", Self::get);
    }
}
