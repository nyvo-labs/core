pub mod formats;
pub mod file;
pub mod types;
pub use file::File as File;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}