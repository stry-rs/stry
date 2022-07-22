use axum::{
    body::{Bytes, Full},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

static MAIN_CSS: &str = include_str!("../../assets/main.css");
static MAIN_JS: &str = include_str!("../../assets/main.js");

static ALPINE_JS: &str = include_str!("../../assets/alpinejs-csp.js");
static ALPINE_COLLAPSE_JS: &str = include_str!("../../assets/alpinejs-collapse.js");

pub fn routes() -> Router {
    Router::new()
        .route("/main.css", get(|| async { Css(MAIN_CSS) }))
        .route("/main.js", get(|| async { Js(MAIN_JS) }))
        .route("/alpinejs.js", get(|| async { Js(ALPINE_JS) }))
        .route(
            "/alpinejs-collapse.js",
            get(|| async { Js(ALPINE_COLLAPSE_JS) }),
        )
}

pub struct Css<T: Into<Full<Bytes>>>(pub T);

impl<T: Into<Full<Bytes>>> IntoResponse for Css<T> {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/css; charset=UTF-8"),
            )],
            self.0.into(),
        )
            .into_response()
    }
}

pub struct Js<T: Into<Full<Bytes>>>(pub T);

impl<T: Into<Full<Bytes>>> IntoResponse for Js<T> {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/javascript; charset=UTF-8"),
            )],
            self.0.into(),
        )
            .into_response()
    }
}
