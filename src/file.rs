use std::{
    cmp::min,
    fs::{self, FileTimes, OpenOptions},
    io::{Read, Seek, Write},
    mem::drop,
};

use chrono::{DateTime, Utc};

use crate::formats::{rar::RarFileEntry, zip::ZipFileEntry};

pub struct FsFile {
    pub size: u64,
    pub reader: Option<FileReader>,
    pub modified: DateTime<Utc>,
    pub is_directory: bool,
}

impl Clone for FsFile {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            reader: self.reader.clone(),
            modified: self.modified,
            is_directory: self.is_directory,
        }
    }
}

impl FsFile {
    pub fn new(path: &String) -> Self {
        if fs::metadata(path).unwrap().is_dir() {
            return Self {
                size: 0,
                reader: None,
                modified: fs::metadata(path).unwrap().modified().unwrap().into(),
                is_directory: true,
            };
        };
        let reader = FileReader::new(path);
        Self {
            size: reader.get_size(),
            modified: reader.get_times().modified,
            reader: Some(reader),
            is_directory: false,
        }
    }
}

pub trait File {
    fn get_path(&self) -> &String;
    fn get_offset(&self) -> &u64;
    fn get_size(&self) -> &u64;
    fn get_modified(&self) -> &DateTime<Utc>;
    fn get_is_directory(&self) -> &bool;
    fn get_source(&mut self) -> Option<&mut FileReader>;
    fn get_checksum(&self) -> &u32;
}

pub enum OriginalFileEntry<'a> {
    Zip(&'a ZipFileEntry<'a>),
    Rar(&'a RarFileEntry),
}

pub trait FileEntry<'a> {
    fn get_path(&self) -> &String;
    fn get_offset(&self) -> &u64;
    fn get_size(&self) -> &u64;
    fn get_modified(&self) -> &DateTime<Utc>;
    fn get_is_directory(&self) -> &bool;
    fn get_uncompressed_size(&self) -> &u32;
    fn get_original(&'a self) -> OriginalFileEntry<'a>;
}

#[derive(Debug)]
pub struct Times {
    pub created: DateTime<Utc>,
    pub accessed: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

#[derive(Debug)]
pub struct FileReader {
    path: String,
    file: std::fs::File,
    pos: u64,
}

impl<'a> FileReader {
    pub fn new(path: &'a String) -> Self {
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        file.rewind().unwrap();

        Self {
            path: path.to_owned(),
            file,
            pos: 0,
        }
    }

    pub fn set_end(&mut self, end: &u64) -> LimitedFileReader {
        LimitedFileReader {
            file: self,
            end: *end,
        }
    }

    pub fn close(self) {
        self.file.sync_all().unwrap();
        drop(self);
    }

    pub fn export(
        &mut self,
        offset: &u64,
        len: &u64,
        target: &mut FileWriter,
        modified: &DateTime<Utc>,
        buffer_size: &u64,
    ) {
        let pos_before = self.get_position();
        self.seek(offset);
        let mut buf = vec![0; *buffer_size as usize];
        let mut remaining = *len;

        while remaining > 0 {
            let to_read = min(*buffer_size, remaining) as usize;
            let read = self.read(&mut buf[..to_read]);
            target.write(read);
            remaining -= to_read as u64;
        }

        let time = FileTimes::new().set_modified(modified.to_owned().into());
        target.set_times(&time);

        self.seek(&pos_before);
    }

    pub fn get_times(&self) -> Times {
        let metadata = self.file.metadata().unwrap();
        Times {
            created: metadata
                .created()
                .unwrap_or_else(|_| metadata.modified().unwrap())
                .into(),
            accessed: metadata.accessed().unwrap().into(),
            modified: metadata.modified().unwrap().into(),
        }
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn seek(&mut self, pos: &u64) {
        self.file.seek(std::io::SeekFrom::Start(*pos)).unwrap();
        self.pos = *pos;
    }

    pub fn rewind(&mut self) {
        self.seek(&0);
    }

    pub fn jump(&mut self, offset: &i128) {
        self.seek(&((self.pos as i128 + offset) as u64));
    }

    pub fn get_position(&self) -> u64 {
        self.pos
    }

    pub fn get_size(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    pub fn read<'b>(&mut self, buf: &'b mut [u8]) -> &'b mut [u8] {
        let _ = self.file.read_exact(buf);
        self.pos += buf.len() as u64;
        buf
    }

    pub fn read_utf8(&mut self, len: &u64) -> String {
        let mut buf = vec![0; *len as usize];
        self.read(&mut buf);
        String::from_utf8(buf).unwrap()
    }

    pub fn read_u8array(&mut self, len: &u64) -> Vec<u8> {
        let mut buf = vec![0; *len as usize];
        self.read(&mut buf);
        buf
    }

    pub fn read_u8(&mut self) -> u8 {
        let mut buf = [0; 1];
        self.read(&mut buf);
        u8::from_le_bytes(buf)
    }

    pub fn read_u16le(&mut self) -> u16 {
        let mut buf = [0; 2];
        self.read(&mut buf);
        u16::from_le_bytes(buf)
    }

    pub fn read_u16be(&mut self) -> u16 {
        let mut buf = [0; 2];
        self.read(&mut buf);
        u16::from_be_bytes(buf)
    }

    pub fn read_u32le(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.read(&mut buf);
        u32::from_le_bytes(buf)
    }

    pub fn read_u32be(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.read(&mut buf);
        u32::from_be_bytes(buf)
    }

    pub fn read_u64le(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.read(&mut buf);
        u64::from_le_bytes(buf)
    }

    pub fn read_u64be(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.read(&mut buf);
        u64::from_be_bytes(buf)
    }

    pub fn read_u128le(&mut self) -> u128 {
        let mut buf = [0; 16];
        self.read(&mut buf);
        u128::from_le_bytes(buf)
    }

    pub fn read_u128be(&mut self) -> u128 {
        let mut buf = [0; 16];
        self.read(&mut buf);
        u128::from_be_bytes(buf)
    }

    pub fn read_vu7(&mut self) -> u128 {
        // referred to as vint in the RAR 5.0 spec
        let mut result = 0;
        let mut shift = 0u16;
        loop {
            let byte = self.read_u8();
            result |= ((byte & 0x7F) as u128) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        result
    }
}

impl Clone for FileReader {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            file: OpenOptions::new().read(true).open(&self.path).unwrap(),
            pos: self.pos,
        }
    }
}

#[derive(Debug)]
pub struct LimitedFileReader<'a> {
    file: &'a mut FileReader,
    end: u64,
}

impl Read for LimitedFileReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.file.get_position() >= self.end {
            return Ok(0);
        }
        let to_read = min(buf.len(), (self.end - self.file.get_position()) as usize);
        let read = self.file.read(&mut buf[..to_read]);
        Ok(read.len())
    }
}

#[derive(Debug)]
pub struct FileWriter {
    path: String,
    file: std::fs::File,
    pos: u64,
}

impl<'a> FileWriter {
    pub fn new(path: &'a String, append: &bool) -> Self {
        if *append {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap();
            file.rewind().unwrap();
            return Self {
                path: path.to_owned(),
                pos: file.metadata().unwrap().len(),
                file,
            };
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        file.rewind().unwrap();

        Self {
            path: path.to_owned(),
            file,
            pos: 0,
        }
    }

    pub fn close(self) {
        self.file.sync_all().unwrap();
        drop(self);
    }

    pub fn set_times(&self, times: &FileTimes) {
        self.file.set_times(*times).unwrap();
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn seek(&mut self, pos: &u64) {
        self.file.seek(std::io::SeekFrom::Start(*pos)).unwrap();
        self.pos = *pos;
    }

    pub fn rewind(&mut self) {
        self.seek(&0);
    }

    pub fn jump(&mut self, offset: &i128) {
        self.seek(&((self.pos as i128 + offset) as u64));
    }

    pub fn get_position(&self) -> u64 {
        self.pos
    }

    pub fn get_size(&self) -> u64 {
        self.file.metadata().unwrap().len()
    }

    pub fn write(&mut self, buf: &[u8]) {
        self.file.write_all(buf).unwrap();
        self.pos += buf.len() as u64;
    }

    pub fn write_utf8(&mut self, s: &String) {
        self.write(s.as_bytes());
    }

    pub fn write_u8array(&mut self, buf: &Vec<u8>) {
        self.write(buf.as_slice());
    }

    pub fn write_u8(&mut self, n: &u8) {
        self.write(&n.to_le_bytes());
    }

    pub fn write_u16le(&mut self, n: &u16) {
        self.write(&n.to_le_bytes());
    }

    pub fn write_u16be(&mut self, n: &u16) {
        self.write(&n.to_be_bytes());
    }

    pub fn write_u32le(&mut self, n: &u32) {
        self.write(&n.to_le_bytes());
    }

    pub fn write_u32be(&mut self, n: &u32) {
        self.write(&n.to_be_bytes());
    }

    pub fn write_u64le(&mut self, n: &u64) {
        self.write(&n.to_le_bytes());
    }

    pub fn write_u64be(&mut self, n: &u64) {
        self.write(&n.to_be_bytes());
    }

    pub fn write_u128le(&mut self, n: &u128) {
        self.write(&n.to_le_bytes());
    }

    pub fn write_u128be(&mut self, n: &u128) {
        self.write(&n.to_be_bytes());
    }

    pub fn write_vu7(&mut self, n: &u128) {
        let mut n = *n;
        loop {
            let mut byte = (n & 0x7F) as u8;
            n >>= 7;
            if n != 0 {
                byte |= 0x80;
            }
            self.write(&[byte]);
            if n == 0 {
                break;
            }
        }
    }
}
