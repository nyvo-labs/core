use std::str::FromStr;

use crate::{
    Error, Result,
    file::FileEntry,
    format::{ArchiveFormat, ArchiveFormatReader, ArchiveFormatType},
};
use chrono::DateTime;
use dh::ReadSeek;
use zip::{HasZipMetadata, ZipArchive};

pub struct ZipFormat<'a> {
    reader: Option<ZipArchive<&'a mut dyn ReadSeek>>,
}

impl<'a> ArchiveFormat for ZipFormat<'a> {
    fn get_type(&self) -> ArchiveFormatType {
        ArchiveFormatType::ZipLike
    }
}

impl<'a> ArchiveFormatReader<'a> for ZipFormat<'a> {
    fn new(reader: &'a mut dyn ReadSeek) -> Result<Self> {
        Ok(ZipFormat {
            reader: Some(ZipArchive::new(reader).map_err(|_| Error::ReaderCreationFailed)?),
        })
    }

    fn list_files(&mut self) -> Result<Vec<FileEntry>> {
        let mut entries = Vec::new();
        if let Some(archive) = self.reader.as_mut() {
            for i in 0..archive.len() {
                let file_seek = archive.by_index_seek(i).map_err(|_| Error::ReaderFailed)?;
                let file = file_seek.get_metadata();
                let file_times = file.extra_fields.get(0);
                entries.push(FileEntry {
                    index: i as u64,
                    name: file
                        .file_name
                        .strip_suffix("/")
                        .unwrap_or(file.file_name.strip_suffix(r"\").unwrap_or(&file.file_name))
                        .to_string(),
                    size: file.uncompressed_size,
                    directory: file.is_dir(),
                    encrypted: file.encrypted,
                    accessed: match file_times {
                        _ => {
                            let time = file.last_modified_time.unwrap_or_default();
                            DateTime::from_str(
                                format!(
                                    "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}Z",
                                    time.year(),
                                    time.month(),
                                    time.day(),
                                    time.hour(),
                                    time.minute(),
                                    time.second()
                                )
                                .as_str(),
                            )
                            .unwrap_or_default()
                        }
                    },
                    created: match file_times {
                        _ => {
                            let time = file.last_modified_time.unwrap_or_default();
                            DateTime::from_str(
                                format!(
                                    "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}Z",
                                    time.year(),
                                    time.month(),
                                    time.day(),
                                    time.hour(),
                                    time.minute(),
                                    time.second()
                                )
                                .as_str(),
                            )
                            .unwrap_or_default()
                        }
                    },
                    modified: match file_times {
                        _ => {
                            let time = file.last_modified_time.unwrap_or_default();
                            DateTime::from_str(
                                format!(
                                    "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}Z",
                                    time.year(),
                                    time.month(),
                                    time.day(),
                                    time.hour(),
                                    time.minute(),
                                    time.second()
                                )
                                .as_str(),
                            )
                            .unwrap_or_default()
                        }
                    },
                });
            }
        } else {
            return Err(Error::ReaderNotFound);
        }
        Ok(entries)
    }
}
