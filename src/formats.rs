use auto::{detect, DetectionAccuracy};
use dh::Readable;
use std::{io::Result, path::Path};

pub mod auto;
pub mod msbt;
pub mod umsbt;
pub mod zip;

pub enum Format {
    Zip,
    Umsbt,
    Msbt,
    Hssp1,
    Hssp2,
    Hssp3,
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
