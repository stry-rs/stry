//! A simple parser for database connection URIs.

use std::{
    borrow::Cow,
    collections::BTreeMap,
    convert::TryFrom,
    error::Error,
    fmt::{self, Display, Formatter},
    num::ParseIntError,
    str::{FromStr, Utf8Error},
};

/// A URI connection string.
///
/// # Format
///
/// ```not_rust
/// scheme://[username:password@]host[:port1][,...hostN[:portN]][/[database][?options]]
/// ```
///
/// # Warning
///
/// The parser does **not** support IPv6 or Unix pipes, manually build
/// the config instead.
#[derive(Debug, PartialEq)]
pub struct Uri {
    pub scheme: String,

    pub username: Option<String>,
    pub password: Option<Vec<u8>>,

    pub hosts: Vec<String>,
    pub ports: Vec<u16>,

    pub database: Option<String>,

    pub options: Option<BTreeMap<String, String>>,
}

impl Uri {
    pub fn new<S, H>(scheme: S, host: H, port: u16) -> Uri
    where
        S: Into<String>,
        H: Into<String>,
    {
        Uri {
            scheme: scheme.into(),
            username: None,
            password: None,
            hosts: vec![host.into()],
            ports: vec![port],
            database: None,
            options: None,
        }
    }

    pub fn parse<S>(text: S) -> Result<Self, UriError>
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        let config = Parser::parse(text)?;

        Ok(config)
    }

    pub fn username<U>(mut self, username: U) -> Self
    where
        U: Into<String>,
    {
        self.username = Some(username.into());

        self
    }

    pub fn password<P>(mut self, password: P) -> Self
    where
        P: Into<Vec<u8>>,
    {
        self.password = Some(password.into());

        self
    }

    pub fn database<D>(mut self, database: D) -> Self
    where
        D: Into<String>,
    {
        self.database = Some(database.into());

        self
    }

    pub fn option<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        let tree = self.options.get_or_insert_with(BTreeMap::new);

        tree.insert(key.into(), value.into());

        self
    }
}

impl FromStr for Uri {
    type Err = UriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config = Uri::parse(s)?;

        Ok(config)
    }
}

impl<'s> TryFrom<&'s str> for Uri {
    type Error = UriError;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        let config = Uri::parse(value)?;

        Ok(config)
    }
}

impl TryFrom<String> for Uri {
    type Error = UriError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let config = Uri::parse(value)?;

        Ok(config)
    }
}

// A macro version of `take_until` as the pattern api isn't stable yet
macro_rules! take_until {
    ($text:expr, $patt:expr) => {{
        match $text.find($patt) {
            Some(index) => {
                let (head, tail) = $text.split_at(index);

                $text = tail;

                Some(head)
            }
            None => None,
        }
    }};
}

// This parser is based off the one from tokio-postgres.
// Just needed one that didn't have too many dependencies.
struct Parser<'s> {
    text: &'s str,
}

impl<'s> Parser<'s> {
    fn parse(text: &'s str) -> Result<Uri, UriError> {
        let mut parser = Parser { text };

        let scheme = take_until!(parser.text, ':').ok_or(UriError::MissingScheme)?;

        parser.eat(':')?;
        parser.eat('/')?;
        parser.eat('/')?;

        let (username, password) = if parser.text.contains('@') {
            parser.parse_credentials()?
        } else {
            (None, None)
        };
        let (hosts, ports) = parser.parse_hosts()?;
        let database = parser.parse_path();
        let options = parser.parse_params()?;

        Ok(Uri {
            scheme: scheme.to_string(),

            username,
            password,

            hosts,
            ports,

            database,

            options,
        })
    }

    fn eat(&mut self, target: char) -> Result<(), UriError> {
        if self.text.starts_with(target) {
            let (_, tail) = self.text.split_at(1);

            self.text = tail;

            Ok(())
        } else {
            Err(UriError::UnexpectedCharacter {
                expected: target,
                got: self.text.chars().next().unwrap(),
            })
        }
    }

    fn parse_credentials(&mut self) -> Result<(Option<String>, Option<Vec<u8>>), UriError> {
        match take_until!(self.text, '@') {
            Some(taken) => {
                let mut it = taken.splitn(2, ':');

                let username = it.next().ok_or(UriError::MissingUsername)?;
                let password = percent_encoding::percent_decode(
                    it.next().ok_or(UriError::MissingPassword)?.as_bytes(),
                );

                self.eat('@')?;

                Ok((
                    Some(username.to_string()),
                    Some(Cow::from(password).to_vec()),
                ))
            }
            None => Ok((None, None)),
        }
    }

    fn parse_hosts(&mut self) -> Result<(Vec<String>, Vec<u16>), UriError> {
        match take_until!(self.text, &['/', '?'] as &[char]) {
            Some(taken) => {
                let pairs = taken.split(',');

                let mut hosts = Vec::new();
                let mut ports = Vec::new();

                for pair in pairs {
                    if let Some(index) = pair.find(':') {
                        let (head, tail) = pair.split_at(index);

                        hosts.push(head.to_string());
                        ports.push(
                            (tail[1..])
                                .parse()
                                .map_err(|err| (tail[1..].to_string(), err))?,
                        );
                    }
                }

                Ok((hosts, ports))
            }
            None => {
                if self.text.is_empty() {
                    Err(UriError::MissingHostPort)
                } else {
                    let mut hosts = Vec::new();
                    let mut ports = Vec::new();

                    if let Some(index) = self.text.find(':') {
                        let (head, tail) = self.text.split_at(index);

                        hosts.push(head.to_string());
                        ports.push(
                            (tail[1..])
                                .parse()
                                .map_err(|err| (tail[1..].to_string(), err))?,
                        );
                    }

                    Ok((hosts, ports))
                }
            },
        }
    }

    fn parse_path(&mut self) -> Option<String> {
        if self.text.starts_with('/') {
            self.text = &self.text[1..];

            if self.text.is_empty() {
                None
            } else if let Some(index) = self.text.find('?') {
                let (head, tail) = self.text.split_at(index);

                self.text = tail;

                Some(String::from(head))
            } else {
                Some(String::from(self.text))
            }
        } else {
            None
        }
    }

    fn parse_params(&mut self) -> Result<Option<BTreeMap<String, String>>, UriError> {
        if self.text.starts_with('?') {
            self.text = &self.text[1..];

            let mut tree = BTreeMap::new();

            for pair in self.text.split('&') {
                let mut splitter = pair.split('=');

                if let (Some(key), Some(value)) = (splitter.next(), splitter.next()) {
                    let key = percent_encoding::percent_decode(key.as_bytes()).decode_utf8()?;
                    let value = percent_encoding::percent_decode(value.as_bytes()).decode_utf8()?;

                    tree.insert(key.to_string(), value.to_string());
                }
            }

            Ok(if tree.is_empty() { None } else { Some(tree) })
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UriError {
    InvalidHostPort { port: String, err: ParseIntError },
    InvalidEncoding { err: Utf8Error },
    MissingScheme,
    MissingUsername,
    MissingPassword,
    MissingHostPort,
    UnexpectedEof,
    UnexpectedCharacter { expected: char, got: char },
}

impl Display for UriError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            UriError::InvalidHostPort { port, .. } => write!(f, "invalid host port: `{}`", port)?,
            UriError::InvalidEncoding { .. } => write!(f, "invalid param encoding")?,
            UriError::MissingScheme => write!(f, "missing scheme")?,
            UriError::MissingUsername => write!(f, "missing username from credentials")?,
            UriError::MissingPassword => write!(f, "missing password from credentials")?,
            UriError::MissingHostPort => write!(f, "missing host and or port")?,
            UriError::UnexpectedEof => write!(f, "unexpected EOF")?,
            UriError::UnexpectedCharacter { expected, got } => write!(
                f,
                "unexpected character: expected `{}` but got `{}`",
                expected, got,
            )?,
        }

        Ok(())
    }
}

impl Error for UriError {}

impl From<(String, ParseIntError)> for UriError {
    fn from((port, err): (String, ParseIntError)) -> Self {
        Self::InvalidHostPort { port, err }
    }
}

impl From<Utf8Error> for UriError {
    fn from(err: Utf8Error) -> Self {
        Self::InvalidEncoding { err }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_minimal_no_encoding() {
        let expected = Uri::new("postgres", "localhost", 54123);
        let actual = Uri::parse("postgres://localhost:54123");

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_options_encoding() {
        let expected = Uri::new("postgres", "localhost", 54123)
            .option("with a space", "for sure");
        let actual = Uri::parse("postgres://localhost:54123?with%20a%20space=for%20sure");

        assert_eq!(Ok(expected), actual);
    }

    #[test]
    fn test_all_no_encoding() {
        let expected = Uri::new("postgres", "localhost", 54123)
            .username("username")
            .password("password")
            .database("database")
            .option("tls", "true");
        let actual = Uri::parse("postgres://username:password@localhost:54123/database?tls=true");

        assert_eq!(Ok(expected), actual);
    }
}
