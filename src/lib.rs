pub mod file;
pub mod formats;
pub mod helpers;
pub mod types;
pub use file::File;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
