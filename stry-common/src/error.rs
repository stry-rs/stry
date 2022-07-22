//! Custom errors used for HTTP responses.

use std::{error::Error, fmt};

pub use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

macro_rules! http_error {
    ( $(
        $( #[$attr:meta] )*
        $name:ident : $msg:expr ,
    )* ) => {
        $(
            $( #[$attr] )*
            #[derive(Debug)]
            pub struct $name;

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, $msg)
                }
            }

            impl Error for $name {}
        )*
    };
}

// This are later translated into normal http status codes
// but they don't actually correspond to the code you mny think
http_error! {
    NotFound: "not found",
    Unauthenticated: "unauthenticated",
    Unauthorized: "unauthorized",
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ErrorResponse<Err> {
    pub error: Err,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct StatusCodeErrorResponse {
    pub code: u16,
    pub status: Option<&'static str>,
    pub message: &'static str,
}
