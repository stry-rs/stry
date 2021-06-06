use syndrome::SyndromeBuilder;

pub fn routes(router: &mut SyndromeBuilder) {
    router.insert(syndrome::Method::POST, "/graphql", index);
}

clockwork::route! {
    pub async fn index[state: Index / INDEX](req: syndrome::Request) -> anyhow::Result<syndrome::Response> = if {
        index_inner(req).await
    } else {
        let mut res = syndrome::Response::new(syndrome::Body::empty());

        *res.status_mut() = syndrome::StatusCode::SERVICE_UNAVAILABLE;

        Ok(res)
    }
}

#[inline]
async fn index_inner(req: syndrome::Request) -> anyhow::Result<syndrome::Response> {
    todo!()
}
