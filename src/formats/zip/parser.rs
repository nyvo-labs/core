use crate::types::ArchiveMetadata;
use std::{
    fs::OpenOptions,
    io::{Read, Seek},
};

pub fn metadata(path: &str) -> ArchiveMetadata {
    let mut file = OpenOptions::new().read(true).open(path).unwrap();
    file.rewind().unwrap();
    let mut filecount = [0; 4];
    let _ = file.read_exact(&mut filecount);
    let filecount = u32::from_le_bytes(filecount);
    ArchiveMetadata {
        file_count: filecount as u128,
    } // NO! THIS IS NOT A FILE COUNT, THIS IS JUST A VALUE READING TEST
}
