#![allow(clippy::too_many_arguments)]

pub mod archive;
pub mod file;
pub mod formats;
pub mod helpers;

pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
