use stry_common::{
    error::{ErrorResponse, StatusCodeErrorResponse},
    prelude::*,
};

use axum::{http::StatusCode, Json};

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

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Self {
        Self(err.into())
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
    fn into_response(self) -> axum::response::Response {
        let err = self.0;

        error!(error = ?err, "error handling request");

        let (status, message) = match err {
            err if err.is::<stry_common::error::NotFound>() => {
                (StatusCode::NOT_FOUND, "no resource found at this url")
            }
            err if err.is::<stry_common::error::Unauthenticated>() => (
                StatusCode::UNAUTHORIZED,
                "an account is required to access this resource",
            ),
            err if err.is::<stry_common::error::Unauthorized>() => (
                StatusCode::FORBIDDEN,
                "forbidden from accessing this resource",
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
        };

        let body = ErrorResponse {
            error: StatusCodeErrorResponse {
                code: status.as_u16(),
                status: status.canonical_reason(),
                message,
            },
        };

        (status, Json(body)).into_response()
    }
}
