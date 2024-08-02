use crate::{
    file::{FileEntry, FileReader, FileWriter, FsFile},
    formats::{
        self,
        zip::{ZipFile, ZipFileEntry},
        Formats,
    },
    helpers::hash::crc32,
};
use std::fs::create_dir_all;

pub enum OriginalArchiveMetadata<'a> {
    Zip(formats::zip::ZipArchiveMetadata<'a>),
}

pub trait ArchiveMetadata<'a> {
    fn get_format(&self) -> Formats;
    fn get_files(&self) -> Vec<&dyn FileEntry>;
    fn get_original(&'a self) -> OriginalArchiveMetadata<'a>;
}

pub fn metadata<'a>(
    format: Formats,
    input: String,
    check_integrity: bool,
    buffer_size: u64,
) -> Result<Box<OriginalArchiveMetadata<'a>>, String> {
    let mut file = FileReader::new(&input);
    let metadata = match format {
        Formats::Zip => {
            let metadata = formats::zip::parser::metadata(&mut file);
            if check_integrity
                && !formats::zip::parser::check_integrity_all(
                    &mut file,
                    &metadata.files,
                    &buffer_size,
                )
            {
                return Err("Integrity check failed".to_string());
            }
            OriginalArchiveMetadata::Zip(metadata)
        }
    };

    Ok(Box::new(metadata))
}

pub fn extract(
    format: Formats,
    input: String,
    output: String,
    index: Option<u32>,
    path: Option<String>,
    all: bool,
    check_integrity: bool,
    buffer_size: u64,
) -> Result<(), String> {
    let mut file = FileReader::new(&input);
    create_dir_all(&output).unwrap();

    let metadata: &dyn ArchiveMetadata = match format {
        Formats::Zip => {
            let metadata = formats::zip::parser::metadata(&mut file);
            if check_integrity
                && !formats::zip::parser::check_integrity_all(
                    &mut file,
                    &metadata.files,
                    &buffer_size,
                )
            {
                return Err("Integrity check failed".to_string());
            }
            &metadata.clone() as &dyn ArchiveMetadata
        }
    };

    let files = metadata.get_files();

    if all {
        match format {
            Formats::Zip => {
                let zip_files = formats::zip::to_zip_entries(files);
                formats::zip::parser::extract(&mut file, &zip_files, &buffer_size, &|path| {
                    format!("{}/{}", &output, &path)
                });
            }
        }
    } else if index.is_some() {
        let index = index.unwrap();
        if index >= files.len() as u32 {
            return Err("Index out of range".to_string());
        }
        formats::zip::parser::extract(
            &mut file,
            &formats::zip::to_zip_entries(files),
            &buffer_size,
            &|path| format!("{}/{}", &output, &path),
        );
    } else {
        let path = path.unwrap();
        let files: Vec<ZipFileEntry> = metadata
            .get_files()
            .iter()
            .filter_map(|file| {
                if file.get_path().starts_with(&path) {
                    Some(formats::zip::to_zip_entry(*file))
                } else {
                    None
                }
            })
            .collect();
        formats::zip::parser::extract(&mut file, &files, &buffer_size, &|path| {
            format!("{}/{}", &output, &path)
        });
    };

    Ok(())
}

pub struct EntrySource {
    pub path: String,
    pub source: FsFile,
}

impl Clone for EntrySource {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            source: self.source.clone(),
        }
    }
}

pub fn create(
    format: Formats,
    output: String,
    input: Vec<EntrySource>,
    buffer_size: u64,
) -> Result<(), String> {
    let mut file = FileWriter::new(&output, &false);

    match format {
        Formats::Zip => {
            let files: Vec<ZipFile> = input
                .iter()
                .cloned()
                .map(|entry| {
                    if entry.source.is_directory {
                        return ZipFile {
                            checksum: 0,
                            path: entry.path.clone(),
                            offset: 0,
                            size: 0,
                            modified: entry.source.modified,
                            is_directory: true,
                            source: None,
                        };
                    };
                    let size = entry.source.size.to_owned();
                    let mut reader = entry.source.reader.unwrap();
                    ZipFile {
                        checksum: crc32::hash(&mut reader, &0, &size, &buffer_size),
                        path: entry.path.clone(),
                        offset: 0,
                        size,
                        modified: entry.source.modified,
                        is_directory: entry.source.is_directory,
                        source: Some(reader),
                    }
                })
                .collect();
            formats::zip::writer::write(
                &mut file,
                formats::zip::ZipArchiveData { files },
                &buffer_size,
            );
        }
    }

    Ok(())
}
