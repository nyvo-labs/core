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
    path: &Path,
    input: &mut dyn Readable,
    accuracy: &DetectionAccuracy,
) -> Result<Format> {
    Err(Error::new(ErrorKind::Other, "Failed to detect format"))
}
