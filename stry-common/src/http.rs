use std::{
    io::{Cursor, Write as _},
    time::SystemTime,
};

use hyper::{http, Body, Request};
use rand::{rngs::OsRng, RngCore as _};

use crate::{models::Id, prelude::*};

#[rustfmt::skip]
#[derive(serde::Deserialize)]
pub struct Pagination {
    pub cursor: Option<Id>,
    #[serde(default  = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    10
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            cursor: None,
            limit: 10,
        }
    }
}

pub fn make_span(req: &Request<Body>) -> Span {
    let span = debug_span!(
        "request",
        method = %req.method(),
        path = %req.uri().path(),
        version = ?req.version(),
        accept = field::Empty,
        referer = field::Empty,
        user_agent = field::Empty,
        id = %random_id(),
    );

    let headers = req.headers();

    fn set_record(
        span: &Span,
        headers: &http::HeaderMap,
        field: &str,
        header: http::header::HeaderName,
    ) {
        if let Some(Ok(referer)) = headers.get(header).map(|value| value.to_str()) {
            span.record(field, &field::display(referer));
        }
    }

    set_record(&span, headers, "accept", http::header::ACCEPT);
    set_record(&span, headers, "referer", http::header::REFERER);
    set_record(&span, headers, "user_agent", http::header::USER_AGENT);

    span
}

fn random_id() -> String {
    let length = 32;

    let byte_length: usize = (length / 4) * 3;

    let mut raw: Vec<u8> = vec![0; byte_length];

    {
        let now = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();

        let secs: u64 = now.as_secs();
        let nano_secs: u32 = now.subsec_nanos();

        let mut cursor = Cursor::new(&mut *raw);

        cursor.write_all(&nano_secs.to_le_bytes()).unwrap();
        cursor.write_all(&secs.to_le_bytes()).unwrap();
    }

    OsRng.fill_bytes(&mut raw[12..byte_length]);

    base64::encode_config(&raw, base64::STANDARD)
}

#[cfg(unix)]
pub async fn shutdown_signal() {
    use std::io;
    use tokio::signal::unix::SignalKind;

    async fn terminate() -> io::Result<()> {
        tokio::signal::unix::signal(SignalKind::terminate())?
            .recv()
            .await;

        Ok(())
    }

    tokio::select! {
        _ = terminate() => {},
        _ = tokio::signal::ctrl_c() => {},
    }

    info!("signal received, starting graceful shutdown");
}

#[cfg(windows)]
pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");

    info!("signal received, starting graceful shutdown");
}
