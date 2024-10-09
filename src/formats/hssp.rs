use std::cell::RefCell;

use crate::file::Readable;

pub mod parser;
pub mod writer;
pub struct HsspMetadata {
    pub version: u8,
    pub checksum: u32,
    pub encryption: Option<HsspEncryption>,
    pub files: Vec<HsspFileEntry>,
    pub has_main: bool,
}

pub struct HsspArchiveData {
    pub version: u8,
    pub encryption: Option<HsspEncryptionData>,
    pub files: Vec<(HsspFileEntry, Option<Box<dyn Readable>>)>,
}

pub struct HsspEncryptionData {
    pub key: Vec<u8>,
    pub iv: [u8; 16],
}

pub struct HsspEncryption {
    pub hash: [u8; 32],
    pub in_hash: [u8; 32],
    pub iv: [u8; 16],
    pub data: Option<RefCell<Vec<u8>>>,
}

#[derive(Debug)]
pub struct HsspFileEntry {
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub is_main: bool,
    pub is_directory: bool,
}
