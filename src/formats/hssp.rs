pub mod parser;

#[derive(Debug)]
pub struct HsspMetadata {
    pub version: u8,
    pub checksum: u32,
    pub encryption: Option<HsspEncryption>,
    pub files: Vec<HsspFileEntry>,
    pub has_main: bool,
}

#[derive(Debug)]
pub struct HsspEncryption {
    pub hash: [u8; 32],
    pub in_hash: [u8; 32],
    pub iv: [u8; 16],
}

#[derive(Debug)]
pub struct HsspFileEntry {
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub is_main: bool,
    pub is_directory: bool,
}
