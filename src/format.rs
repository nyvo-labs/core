use crate::{Result, file::FileEntry};
use dh::ReadSeek;

pub mod zip;

pub trait ArchiveFormat {
    fn get_type(&self) -> ArchiveFormatType;
}

pub trait ArchiveFormatReader<'a>: ArchiveFormat + Sized {
    fn new(reader: &'a mut dyn ReadSeek) -> Result<Self>;
    fn list_files(&mut self) -> Result<Vec<FileEntry>>;
}

pub enum ArchiveFormatType {
    ZipLike, // every file is compressed & encrypted separately, index is never encrypted
}
