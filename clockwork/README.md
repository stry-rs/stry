# Clockwork (Just runs like clockwork)

Clockwork is metrics and route manager dashboard for [hyper](https://hyper.rs/).

## Example

```rust
use clockwork::Route;

static CLOCKWORK: &[Route] = &[
    route::INDEX,
];

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn service(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    // match (req.method(), req.uri().path()) {
    //     (&Method::GET, "/") =>
    // }

    todo!()
}

mod routes {
    #[clockwork::route]
    pub async fn index(_: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new("Hello, World!".into()))
    }
}
```
