//! Base entities that are used internally and by other 'modules'.

use crate::models::{blog::Post, story::Story, Existing};

/// Universal site settings.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    /// A unique setting name.
    ///
    /// Left as a [`String`] to allow for other modules to use the settings
    /// without using extension types.
    pub key: String,

    /// The value of the key, encoded as JSON.
    pub value: String,
}

/// A user of the website, used from displaying authors to signing in.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct User {
    pub account: SettingsAccount,
    pub site: SettingsSite,

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

/// User settings for the user themself, ie name, biography, and security details.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsAccount {
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

/// User settings for the site itself, ie appearance and notifications.
// TODO: support color blindness
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsSite {
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
pub struct Part {
    pub kind: PartKind,

    /// Any comments on or replying to the current part.
    pub comments: Vec<Existing<Comment>>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum PartKind {
    Image { url: String, alt: Option<String>, },
    Text { content: String, words: i64, },
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
