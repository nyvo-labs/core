use chrono::{DateTime, Utc};

pub mod parser;

#[derive(Debug)]
pub struct RarArchiveMetadata {
    pub files: Vec<RarFileEntry>,
    pub archive_start: u64,
    pub version: (u8, u8),
    pub multivolume: bool,
    pub volume: u128,
    pub solid: bool,
    pub has_recovery: bool,
    pub locked: bool,
    pub original_name: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub qo_offset: Option<u64>,
    pub rr_offset: Option<u64>,
}

#[derive(Debug)]
pub struct RarFileEntry {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub uncompressed_size: Option<u64>,
    pub is_directory: bool,
    pub modified: Option<DateTime<Utc>>,
    pub checksum: Option<u32>,
    pub encryption: Option<RarEncryption>,
    pub compression: Option<RarCompression>,
    pub creation_platform: Option<RarPlatform>,
}

#[derive(Debug)]
pub enum RarEncryption {
    Aes256,
}

#[derive(Debug)]
pub struct RarCompression {
    pub version: u8,
    pub solid: bool,
    pub method: u8,
    pub dict_size: u64,
}

#[derive(Debug)]
pub enum RarPlatform {
    Windows,
    Unix,
}
