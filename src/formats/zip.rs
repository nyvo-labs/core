use chrono::{DateTime, Utc};

use crate::{
    archive::{ArchiveMetadata, OriginalArchiveMetadata},
    file::{File, Readable},
    formats::Formats,
};

pub mod parser;
pub mod writer;

/*
Useful links:

- https://pkware.cachefly.net/webdocs/casestudies/APPNOTE.TXT
- http://justsolve.archiveteam.org/wiki/ZIP
- https://developers.acridotheres.com/formats/zip
*/

#[derive(Debug)]
pub struct ZipArchiveMetadata<'a> {
    pub files: Vec<ZipFileEntry<'a>>,
}

impl<'a> ArchiveMetadata<'a> for ZipArchiveMetadata<'a> {
    fn get_format(&self) -> Formats {
        Formats::Zip
    }

    fn get_files(&self) -> Vec<File> {
        self.files.iter().map(|file| file).collect()
    }

    fn get_original(&'a self) -> OriginalArchiveMetadata<'a> {
        OriginalArchiveMetadata::Zip(self.clone())
    }
}

impl<'a> Clone for ZipArchiveMetadata<'a> {
    fn clone(&self) -> Self {
        ZipArchiveMetadata {
            files: self.files.clone(),
        }
    }
}

pub fn to_zip_archive_metadata<'a>(
    from: &'a (dyn ArchiveMetadata<'a> + 'a),
) -> ZipArchiveMetadata<'a> {
    let original = from.get_original();
    match original {
        OriginalArchiveMetadata::Zip(zip_archive) => zip_archive.clone(),
        _ => panic!("This could never happen, this is only here for type safety"),
    }
}

#[derive(Debug)]
pub struct ZipFileEntry<'a> {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub is_directory: bool,
    pub uncompressed_size: u32,
    pub checksum: u32,
    pub extra_field: Vec<u8>,
    pub version: u16,
    pub bit_flag: u16,
    pub compression: &'a str,
}

impl<'a> Clone for ZipFileEntry<'a> {
    fn clone(&self) -> Self {
        ZipFileEntry {
            path: self.path.clone(),
            offset: self.offset,
            size: self.size,
            modified: self.modified,
            is_directory: self.is_directory,
            uncompressed_size: self.uncompressed_size,
            checksum: self.checksum,
            extra_field: self.extra_field.clone(),
            version: self.version,
            bit_flag: self.bit_flag,
            compression: self.compression,
        }
    }
}
#[derive(Debug)]
pub struct ZipFile {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub is_directory: bool,
    pub source: Option<Box<dyn Readable>>,
    pub checksum: u32,
}

#[derive(Debug)]
pub struct ZipArchiveData {
    pub files: Vec<ZipFile>,
}
