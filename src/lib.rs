pub mod formats;
pub mod types;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}