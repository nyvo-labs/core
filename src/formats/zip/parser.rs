use crate::{types::ArchiveMetadata, File};

pub fn metadata(path: &str) -> ArchiveMetadata {
    let mut file = File::new(path);
    let filecount = file.read_u128le();
    ArchiveMetadata { file_count: filecount } // NO! THIS IS NOT A FILE COUNT, THIS IS JUST A VALUE READING TEST
}