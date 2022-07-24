use std::convert::Infallible;

use axum::{
    extract::{FromRequest, RequestParts},
    http::header::ACCEPT_LANGUAGE,
};
use unic_langid::{parser::parse_language_identifier, LanguageIdentifier};

#[derive(Debug, Clone)]
pub struct AcceptLanguage {
    pub languages: Vec<LanguageIdentifier>,
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for AcceptLanguage
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Some(this) = req.extensions().get::<Self>().cloned() {
            return Ok(this);
        }

        if let Some(header) = req.headers().get(ACCEPT_LANGUAGE) {
            if let Ok(header_value) = header.to_str() {
                let languages = accept_language::parse(header_value)
                    .iter()
                    .filter_map(|al| parse_language_identifier(al.as_bytes()).ok())
                    .collect::<Vec<_>>();

                let this = AcceptLanguage { languages };
                req.extensions_mut().insert(this.clone());
                return Ok(this);
            }
        }

        return Ok(AcceptLanguage { languages: vec![] });
    }
}
