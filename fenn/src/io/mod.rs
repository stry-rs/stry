pub mod read;
pub mod write;

pub use self::{read::ReadExt, write::WriteExt};

/// An enum used for reading and writing raw bytes.
///
/// # Default
///
/// Calling `Endian::default()` will always result in `Endian::Little`.
#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub enum Endian {
    Big,
    Little,
}

impl Default for Endian {
    fn default() -> Self {
        Endian::Little
    }
}
