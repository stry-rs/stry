use std::convert::TryFrom;

use stry_common::{
    backend::BackendEntry,
    models::{story::Story, Id},
    prelude::*,
};
use syndrome::{Params, Request, Response};

use crate::{handle, Data};

pub(crate) async fn get(data: Data, _req: Request, params: Params) -> Result<Response, Error> {
    handle(move || async move {
        let id = params.get("id").context("missing `id` from the path")?;

        let id = Id::try_from(id.as_str())?;

        BackendEntry::<Story>::get(&**data, id).await
    })
    .await
}
