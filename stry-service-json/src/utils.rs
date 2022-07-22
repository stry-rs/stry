// pub async fn handle<F, R, T>(f: F) -> Result<crate::router::Response, stry_common::prelude::Error>
// where
//     F: FnOnce() -> R,
//     R: std::future::Future<Output = Result<T, stry_common::prelude::Error>>,
//     T: serde::Serialize,
// {
//     match f().await {
//         Ok(entry) => Ok(self::responses::ok(serde_json::to_vec(&entry)?)),
//         Err(err) if err.is::<stry_common::error::NotFound>() => Ok(self::responses::not_found()),
//         Err(err) => Err(err),
//     }
// }

// pub mod responses {
//     use hyper::{
//         header::{HeaderValue, CONTENT_TYPE},
//         Body, StatusCode,
//     };

//     use crate::router::Response;

//     fn error<B: Into<Body>>(status: StatusCode, body: B) -> Response {
//         let mut res = Response::new(body.into());

//         *res.status_mut() = status;
//         res.headers_mut().insert(
//             CONTENT_TYPE,
//             HeaderValue::from_static("application/json; charset=utf-8"),
//         );

//         res
//     }

//     pub fn ok<B: Into<Body>>(body: B) -> Response {
//         error(StatusCode::OK, body)
//     }

//     pub fn bad_request() -> Response {
//         error(
//             StatusCode::BAD_REQUEST,
//             r#"{ "error": { "code": 400, "status": "bad request" }, "message": "body is too large" }"#,
//         )
//     }

//     pub fn not_found() -> Response {
//         error(
//             StatusCode::NOT_FOUND,
//             r#"{ "error": { "code": 404, "status": "not found" } }"#,
//         )
//     }

//     pub fn internal_server_error() -> Response {
//         error(
//             StatusCode::INTERNAL_SERVER_ERROR,
//             r#"{ "error": { "code": 500, "status": "internal server error" } }"#,
//         )
//     }
// }

// pub mod guard {
//     use hyper::header::{HeaderValue, CONTENT_TYPE};

//     use crate::router::{Request, Response};

//     pub async fn content_type(req: &Request, typ: &'static str) -> Option<Response> {
//         if let Some(header) = req.headers().get(CONTENT_TYPE) {
//             if header != HeaderValue::from_static(typ) {
//                 return Some(super::responses::bad_request());
//             }
//         }

//         None
//     }
// }
