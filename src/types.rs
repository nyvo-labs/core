use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ArchiveMetadata {
    pub format: &'static str,
}

#[derive(Debug)]
pub struct ZipArchiveMetadata {
    pub archive: ArchiveMetadata,
    pub files: Vec<ZipFileEntry>,
}

#[derive(Debug)]
pub struct FileEntry {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub modified: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ZipFileEntry {
    pub file: FileEntry,
    pub uncompressed_size: u32,
    pub checksum: u32,
    pub extra_field: Vec<u8>,
    pub version: u16,
    pub bit_flag: u16,
    pub compression: &'static str,
}
