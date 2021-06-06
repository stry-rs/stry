use {hyper::Server, std::net::SocketAddr, syndrome::Syndrome};

#[tokio::main]
async fn main() {
    let mut router = Syndrome::builder();

    service_graphql::routes(&mut router);

    let router = router.finish();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let server = Server::bind(&addr).serve(router.service());

    if let Err(e) = server.await {
        tracing::error!("server error: {}", e);
    }
}
