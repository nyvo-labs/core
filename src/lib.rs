pub mod file;
pub mod formats;
pub mod helpers;
pub mod archive;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
