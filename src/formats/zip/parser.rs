use crate::{types::ArchiveMetadata, File};

pub fn metadata(path: &str) -> ArchiveMetadata {
    let mut file = File::new(path);
    let lfh_signature = file.read_u32le();
    let filecount = file.read_u128le();
    ArchiveMetadata {
        lfh_matches: lfh_signature == 0x04034b50, // local file header 1 signature matches
        file_count: filecount as u128,
    } // NO! THIS IS NOT A FILE COUNT, THIS IS JUST A VALUE READING TEST
}