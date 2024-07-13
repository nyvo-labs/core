use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ArchiveMetadata<'a> {
    pub format: &'a str,
}

#[derive(Debug)]
pub struct ZipArchiveMetadata<'a> {
    pub archive: ArchiveMetadata<'a>,
    pub files: Vec<ZipFileEntry<'a>>,
}

#[derive(Debug)]
pub struct FileEntry {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub is_directory: bool,
}

#[derive(Debug)]
pub struct ZipFileEntry<'a> {
    pub file: FileEntry,
    pub uncompressed_size: u32,
    pub checksum: u32,
    pub extra_field: Vec<u8>,
    pub version: u16,
    pub bit_flag: u16,
    pub compression: &'a str,
}
