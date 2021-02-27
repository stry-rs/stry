use {
    crate::models::{blog::Post, story::Story, Id},
    chrono::{DateTime, Utc},
};

/// Universal site settings.
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub key: String,
    pub value: String,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct User {
    pub id: Id,

    pub account: SettingsAccount,
    pub site: SettingsSite,

    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub stories: Option<Vec<Story>>,
    /// # Variant
    ///
    /// Is `None` when this type is used indirectly (ie in another entity).
    pub posts: Option<Vec<Post>>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

/// User settings for the user themself, ie name, biography, and security details
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsAccount {
    pub email: String,
    pub name: String,
    pub biography: String,
}

/// User settings for the site itself, ie appearance and notifications
// TODO: support color blindness
#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct SettingsSite {
    pub theme: SiteTheme,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum SiteTheme {
    Dark,
    Light,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Part {
    pub id: Id,

    pub kind: PartKind,
    pub comments: Vec<Comment>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub enum PartKind {
    Image { url: String, alt: Option<String>, },
    Text { content: String, },
}

#[rustfmt::skip]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Comment {
    pub id: Id,

    pub author: User,
    pub main: Vec<Part>,
    pub children: Vec<Comment>,

    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}
