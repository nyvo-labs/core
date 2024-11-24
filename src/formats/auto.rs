use dh::Readable;

use super::Format;
use std::{
    io::{Error, ErrorKind, Result},
    path::Path,
};

pub enum DetectionAccuracy {
    Extension,
    Magic,
    Parse,
}

pub fn detect(
    _path: &Path,
    _input: &mut dyn Readable,
    _accuracy: &DetectionAccuracy,
) -> Result<Format> {
    Err(Error::new(ErrorKind::Other, "Failed to detect format"))
}

pub fn from_str(s: &str) -> Option<Format> {
    use Format::*;
    match s {
        "zip" => Some(Zip),
        "umsbt" => Some(Umsbt),
        "msbt" => Some(Msbt),
        _ => None,
    }
}
