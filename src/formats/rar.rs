pub mod parser;

#[derive(Debug)]
pub struct RarArchiveMetadata {
    pub files: Vec<RarFileEntry>,
    pub archive_start: u64,
    pub version: (u8, u8),
}

#[derive(Debug)]
pub struct RarFileEntry {
    pub path: String,
}

pub enum RarEncryption {
    Aes256,
}
