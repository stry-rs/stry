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

pub mod guard {
    use syndrome::{
        header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE},
        Body, Request, Response, StatusCode,
    };

    const CONFIG_CONTENT_LENGTH: usize = 1024 * 4;

    pub async fn content_length(req: &Request) -> Option<Response> {
        if let Some(length) = req.headers().get(CONTENT_LENGTH) {
            let state = match length.to_str() {
                Ok(length) => match length.parse::<usize>() {
                    Ok(length) if length > CONFIG_CONTENT_LENGTH => Some((
                        StatusCode::BAD_REQUEST,
                        Body::from(r#"{"error": "bad request", "message": "body is too large"}"#),
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
                    r#"{"error": "bad request", "message": "incorrect content type"}"#,
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
