pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Database { inner: Box<dyn DatabaseError> },
}

pub trait DatabaseError {}
