# Hieroglyph

A API definition language and documentation tool inspired by gRPC and OpenAPI.

## Examples

Hieroglyph:
```
Connection <T> :: type {
    edges: [Edge <T>]
    page_info: PageInfo
}
```

Rust:
```rust
pub struct Connection<T> {
    pub edges: Vec<Edge<T>>,
    pub page_info: PageInfo,
}
```
