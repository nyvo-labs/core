use std::{io::Result, path::Path};

use crate::{formats::Format, prepare_output_dir};
use dh::recommended::*;
use neozip;

pub fn extract_all(input: &Path, output: &Path, format: Format, buffer_size: u64) -> Result<()> {
    let mut reader = dh::file::open_r(input)?;

    prepare_output_dir(output)?;

    match format {
        Format::Zip => {
            let meta = neozip::metadata(&mut reader)?;
            for file in meta.files {
                let mut writer = dh::file::open_w(output.join(&file.path))?;
                neozip::extract_content(&mut reader, &mut writer, &file, buffer_size)?;
            }
        }
    };

    Ok(())
}
