use auto::{detect, DetectionAccuracy};
use dh::Readable;
use std::{io::Result, path::Path};

pub mod auto;
pub mod zip;

pub enum Format {
    Zip,
}

impl Format {
    pub fn auto(
        path: &Path,
        input: &mut dyn Readable,
        accuracy: &DetectionAccuracy,
    ) -> Result<Format> {
        detect(path, input, accuracy)
    }
}
