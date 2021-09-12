use stry_common::models::Id;

#[rustfmt::skip]
#[derive(stry_common::prelude::Deserialize)]
pub struct Query {
    pub cursor: Option<Id>,
    #[serde(default  = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

impl Default for Query {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: 10,
        }
    }
}

pub async fn handle<F, R, T>(f: F) -> Result<crate::router::Response, stry_common::prelude::Error>
where
    F: FnOnce() -> R,
    R: std::future::Future<Output = Result<T, stry_common::prelude::Error>>,
    T: serde::Serialize,
{
    match f().await {
        Ok(entry) => Ok(self::responses::ok(serde_json::to_vec(&entry)?)),
        Err(err) if err.is::<stry_common::error::NotFound>() => Ok(self::responses::not_found()),
        Err(err) => Err(err),
    }
}

pub mod responses {
    use hyper::{
        header::{HeaderValue, CONTENT_TYPE},
        Body, StatusCode,
    };

    use crate::router::Response;

    fn error<B: Into<Body>>(status: StatusCode, body: B) -> Response {
        let mut res = Response::new(body.into());

        *res.status_mut() = status;
        res.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );

        res
    }

    pub fn ok<B: Into<Body>>(body: B) -> Response {
        error(StatusCode::OK, body)
    }

    pub fn bad_request() -> Response {
        error(
            StatusCode::BAD_REQUEST,
            r#"{ "error": { "code": 400, "status": "bad request" }, "message": "body is too large" }"#,
        )
    }

    pub fn not_found() -> Response {
        error(
            StatusCode::NOT_FOUND,
            r#"{ error": { "code": 404, "status": "not found" } }"#,
        )
    }

    pub fn internal_server_error() -> Response {
        error(
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{ error": { "code": 500, "status": "internal server error" } }"#,
        )
    }
}

pub mod guard {
    use hyper::{
        header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE},
        Body, StatusCode,
    };

    use crate::router::{Request, Response};

    const CONFIG_CONTENT_LENGTH: usize = 1024 * 4;

    pub async fn content_length(req: &Request) -> Option<Response> {
        if let Some(length) = req.headers().get(CONTENT_LENGTH) {
            let state = match length.to_str() {
                Ok(length) => match length.parse::<usize>() {
                    Ok(length) if length > CONFIG_CONTENT_LENGTH => Some((
                        StatusCode::BAD_REQUEST,
                        Body::from(
                            r#"{ "error": { "code": 400, "status": "bad request" }, "message": "body is too large" }"#,
                        ),
                    )),
                    Err(err) => Some((StatusCode::BAD_REQUEST, Body::from(""))),
                    _ => None,
                },
                Err(err) => Some((StatusCode::BAD_REQUEST, Body::from(""))),
            };

            if let Some((status, body)) = state {
                let mut res = Response::new(body);

                *res.status_mut() = status;
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json; charset=utf-8"),
                );

                return Some(res);
            }
        }

        None
    }

    pub async fn content_type(req: &Request, typ: &'static str) -> Option<Response> {
        if let Some(header) = req.headers().get(CONTENT_TYPE) {
            if header != HeaderValue::from_static(typ) {
                let mut res = Response::new(Body::from(
                    r#"{ "error": { "code": 400, "status": "bad request" }, "message": "incorrect content type" }"#,
                ));

                *res.status_mut() = StatusCode::BAD_REQUEST;
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json; charset=utf-8"),
                );

                return Some(res);
            }
        }

        None
    }
}
