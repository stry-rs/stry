pub mod blog;
pub mod core;
pub mod story;

crate::newtype! {
    /// The database entry id newtype, is a `String` by default
    #[derive(serde::Deserialize, serde::Serialize)]
    Id: String
}
