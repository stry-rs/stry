use std::convert::Infallible;

use stry_common::prelude::*;

use axum::{
    body::{Bytes, Full},
    http::StatusCode,
    Json,
};

#[derive(Debug)]
pub struct Error(stry_common::prelude::Error);

impl Error {
    pub fn from_any<A>(err: A) -> Self
    where
        A: Into<stry_common::prelude::Error>,
    {
        Self(err.into())
    }
}

impl From<stry_common::prelude::Error> for Error {
    fn from(err: stry_common::prelude::Error) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl axum::response::IntoResponse for Error {
    type Body = Full<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        #[derive(serde::Serialize)]
        struct Res {
            error: ResErr,
        }

        #[derive(serde::Serialize)]
        struct ResErr {
            code: u16,
            status: &'static str,
        }

        let err = self.0;

        error!(error = ?err, "error handling request");

        let (status, message) = match err {
            err if err.is::<stry_common::error::NotFound>() => (StatusCode::NOT_FOUND, "not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
        };

        let body = Res {
            error: ResErr {
                code: status.as_u16(),
                status: message,
            },
        };

        (status, Json(body)).into_response()
    }
}
