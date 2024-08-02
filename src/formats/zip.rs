use chrono::{DateTime, Utc};

use crate::{
    archive::{ArchiveMetadata, OriginalArchiveMetadata},
    file::{File, FileEntry, FileReader, OriginalFileEntry},
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

    fn get_files(&self) -> Vec<&dyn FileEntry> {
        self.files
            .iter()
            .map(|file| file as &dyn FileEntry)
            .collect()
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
        //_ => panic!("This could never happen, this is only here for type safety"),
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

impl<'a> FileEntry<'a> for ZipFileEntry<'a> {
    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_offset(&self) -> &u64 {
        &self.offset
    }

    fn get_size(&self) -> &u64 {
        &self.size
    }

    fn get_modified(&self) -> &DateTime<Utc> {
        &self.modified
    }

    fn get_is_directory(&self) -> &bool {
        &self.is_directory
    }

    fn get_uncompressed_size(&self) -> &u32 {
        &self.uncompressed_size
    }

    fn get_original(&'a self) -> OriginalFileEntry<'a> {
        OriginalFileEntry::Zip(self)
    }
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

pub fn to_zip_entry<'a>(from: &'a (dyn FileEntry<'a> + 'a)) -> ZipFileEntry<'a> {
    let original = from.get_original();
    match original {
        OriginalFileEntry::Zip(zip_file) => zip_file.clone(),
        //_ => panic!("This could never happen, this is only here for type safety"),
    }
}

pub fn to_zip_entries<'a>(from: Vec<&'a (dyn FileEntry<'a> + 'a)>) -> Vec<ZipFileEntry<'a>> {
    from.into_iter()
        .map(|file| {
            let original = file.get_original();
            match original {
                OriginalFileEntry::Zip(zip_file) => zip_file.clone(),
                //_ => panic!("This could never happen, this is only here for type safety"),
            }
        })
        .collect()
}

#[derive(Debug)]
pub struct ZipFile {
    pub path: String,
    pub offset: u64,
    pub size: u64,
    pub modified: DateTime<Utc>,
    pub is_directory: bool,
    pub source: Option<FileReader>,
    pub checksum: u32,
}

impl File for ZipFile {
    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_offset(&self) -> &u64 {
        &self.offset
    }

    fn get_size(&self) -> &u64 {
        &self.size
    }

    fn get_modified(&self) -> &DateTime<Utc> {
        &self.modified
    }

    fn get_is_directory(&self) -> &bool {
        &self.is_directory
    }

    fn get_source(&mut self) -> Option<&mut FileReader> {
        match &mut self.source {
            Some(source) => Some(source),
            None => None,
        }
    }

    fn get_checksum(&self) -> &u32 {
        &self.checksum
    }
}

#[derive(Debug)]
pub struct ZipArchiveData {
    pub files: Vec<ZipFile>,
}
