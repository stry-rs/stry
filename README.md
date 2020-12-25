# (stry) attrouter

Attrouter is an attribute based router for various Rust web servers.

## Web Servers
  - [ ] Tide
  - [x] Warp

## Examples

Add `stry-attrouter`, `tokio`, and `warp` to your dependencies:

```toml
stry-attrouter = { version = "0.1", default-features = false, features = [ "with-warp" ] }
tokio = { version = "0.2", features = ["full"] }
warp = "0.2"
```

And in your `main.rs`:

```rust
// GET /hello/warp => 200 OK with body "Hello, warp!"
#[stry_attrouter::get("/hello/{name}")]
fn hello(name: String) -> impl warp::Reply {
    format!("Hello, {}!", name)
}

#[tokio::main]
async fn main() {
    warp::serve(hello())
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```
