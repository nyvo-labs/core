pub mod file;
pub mod format;

pub enum Error {
    ReaderCreationFailed,
    ReaderNotFound,
    ReaderFailed,
}

pub type Result<T> = std::result::Result<T, Error>;
