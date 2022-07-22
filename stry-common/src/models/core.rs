//! Base entities that are used internally and by other 'modules'.

use sodiumoxide::crypto::pwhash::argon2id13;

use crate::{
    models::{blog::Post, story::Story, Existing},
    prelude::{err, Error, Validate},
};

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Validate)]
pub struct UserRegisterForm {
    #[validate(length(min = 4))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 512))]
    pub password: String,
}

/// A user of the website, used from displaying authors to signing in.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct User {
    pub account: Account,
    pub appearance: Appearance,
    pub notifications: Notifications,

    /// Stores all the stories that the user owns.
    ///
    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub stories: Option<Vec<Existing<Story>>>,

    /// Stores all the blog posts that the user has.
    ///
    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub posts: Option<Vec<Existing<Post>>>,
}

impl User {
    pub fn new(account: Account) -> Self {
        Self {
            account,
            appearance: Default::default(),
            notifications: Default::default(),
            stories: None,
            posts: None,
        }
    }
}

/// Information and settings for a user, ie name, biography, and security details.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Account {
    /// The user's visible username.
    ///
    /// # Note
    ///
    /// Usernames are not unique, users are tracked with their `Id` only.
    ///
    /// Due to this, multiple users can have the same username. If possible
    /// let the user choose which account they interact with instead of using
    /// the first retrieved user.
    pub name: String,

    /// The user's email address.
    ///
    /// # Variant
    ///
    /// Is only `Some` when returned for login, a email change and for a user
    /// profile 'view'.
    pub email: Option<String>,

    /// The hash of the user's password stored as bytes.
    ///
    /// # Variant
    ///
    /// This is only `Some` during a login attempt or password change.
    pub hash: Option<Vec<u8>>,

    /// The user's biography in parts.
    ///
    /// # Variant
    ///
    /// Is `None` if you aren't accessing a user profile 'view'.
    pub biography: Option<Vec<Existing<Part>>>,
}

impl Account {
    // TODO: run the hashing in its own thread to allow for more passes
    pub fn new(name: String, email: String, password: String) -> Result<Self, Error> {
        // NOTE: always call tis is any function that needs to use anything from sodiumoxide
        sodiumoxide::init().map_err(|_| err!("unable to initialize sodiumoxide"))?;

        // From https://libsodium.gitbook.io/doc/password_hashing/default_phf#key-derivation
        //
        // For interactive, online operations, crypto_pwhash_OPSLIMIT_INTERACTIVE and crypto_pwhash_MEMLIMIT_INTERACTIVE provide a baseline for these two parameters.
        // This currently requires 64 MiB of dedicated RAM.
        //
        // From https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
        //
        // Use Argon2id with a minimum configuration of 15 MiB of memory, an iteration count of 2, and 1 degree of parallelism.
        let hashed = argon2id13::pwhash(
            password.as_bytes(),
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        )
        .map_err(|_| err!("unable to hash supplied password"))?;

        Ok(Self {
            name,
            email: Some(email),
            hash: Some(hashed.0.to_vec()),
            biography: None,
        })
    }
}

/// User website/app appearance settings.
#[rustfmt::skip]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Appearance {
    pub theme: SiteTheme,
}

/// Website theme the user currently has selected, takes precedence over `prefers-color-scheme`.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum SiteTheme {
    Dark,
    Light,
}

impl Default for SiteTheme {
    fn default() -> Self {
        SiteTheme::Dark
    }
}

/// User notification settings.
#[rustfmt::skip]
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Notifications {
    pub comments: NotificationPreference,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum NotificationPreference {
    Both,
    Neither,
    Email,
    Web,
}

impl Default for NotificationPreference {
    fn default() -> Self {
        NotificationPreference::Both
    }
}

/// A chapter or comment segment that can be commented on.
///
/// # Notes
///
/// Due to parts having comments and comments being made of parts,
/// replies/comments could be nested.
/// It is better to store them separately rather than the whole tree.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum Part {
    Heading(PartHeading),
    Image(PartImage),
    Text(PartText),
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PartHeading {
    pub level: u8,

    /// Any comments on or replying to the current part.
    pub comments: Vec<Existing<Comment>>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PartImage {
    pub url: String,
    pub alt: Option<String>,

    /// Any comments on or replying to the current part.
    pub comments: Vec<Existing<Comment>>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PartText {
    pub content: String,
    pub words: i64,

    /// Any comments on or replying to the current part.
    pub comments: Vec<Existing<Comment>>,
}

/// A comment made of parts and comments that can be commented on.
///
/// # Notes
///
/// Due to parts having comments and comments being made of parts,
/// replies/comments could be nested.
/// It is better to store them separately rather than the whole tree.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Comment {
    pub author: Existing<User>,
    pub main: Vec<Existing<Part>>,
    pub children: Vec<Existing<Comment>>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Tag {
    pub content: String,

    pub description: String,
}
