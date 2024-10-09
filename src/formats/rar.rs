use chrono::{DateTime, Utc};

pub mod parser;

use parser::{EncryptionHeader, Header};

use crate::{archive::ArchiveMetadata, file::File};

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
    pub encryption_header: Option<EncryptionHeader>,
    pub is_last: bool,
    pub headers: Vec<Header>,
}

impl<'a> ArchiveMetadata<'a> for RarArchiveMetadata {
    fn get_files(&self) -> Vec<&File> {
        self.files.iter().map(|x| x).collect()
    }

    fn get_format(&self) -> super::Formats {
        super::Formats::Rar
    }

    fn get_original(&'a self) -> crate::archive::OriginalArchiveMetadata<'a> {
        crate::archive::OriginalArchiveMetadata::Rar(self.clone())
    }
}

impl Clone for RarArchiveMetadata {
    fn clone(&self) -> Self {
        RarArchiveMetadata {
            files: self.files.clone(),
            archive_start: self.archive_start,
            version: self.version,
            multivolume: self.multivolume,
            volume: self.volume,
            solid: self.solid,
            has_recovery: self.has_recovery,
            locked: self.locked,
            original_name: self.original_name.clone(),
            created: self.created,
            qo_offset: self.qo_offset,
            rr_offset: self.rr_offset,
            encryption_header: self.encryption_header.clone(),
            is_last: self.is_last,
            headers: self.headers.clone(),
        }
    }
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

impl Clone for RarEncryption {
    fn clone(&self) -> Self {
        match self {
            RarEncryption::Aes256 => RarEncryption::Aes256,
        }
    }
}

#[derive(Debug)]
pub struct RarCompression {
    pub version: u8,
    pub solid: bool,
    pub method: u8,
    pub dict_size: u64,
}

impl Clone for RarCompression {
    fn clone(&self) -> Self {
        RarCompression {
            version: self.version,
            solid: self.solid,
            method: self.method,
            dict_size: self.dict_size,
        }
    }
}

#[derive(Debug)]
pub enum RarPlatform {
    Windows,
    Unix,
}

impl Clone for RarPlatform {
    fn clone(&self) -> Self {
        match self {
            RarPlatform::Windows => RarPlatform::Windows,
            RarPlatform::Unix => RarPlatform::Unix,
        }
    }
}
