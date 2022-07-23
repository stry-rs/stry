use std::convert::Infallible;

use axum::{
    extract::{FromRequest, RequestParts},
    http::header::ACCEPT_LANGUAGE,
};
use unic_langid::{parser::parse_language_identifier, subtags::Language, LanguageIdentifier};

pub struct AcceptLanguage {
    pub languages: Vec<LanguageIdentifier>,
}

impl AcceptLanguage {
    pub fn first_language(&self) -> Option<Language> {
        self.languages.get(0).map(|language| language.language)
    }
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for AcceptLanguage
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        if let Some(header) = req.headers().get(ACCEPT_LANGUAGE) {
            if let Ok(header_value) = header.to_str() {
                let languages = accept_language::parse(header_value)
                    .iter()
                    .filter_map(|al| parse_language_identifier(al.as_bytes()).ok())
                    .collect::<Vec<_>>();

                return Ok(AcceptLanguage { languages });
            }
        }

        return Ok(AcceptLanguage { languages: vec![] });
    }
}
