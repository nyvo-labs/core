use std::{
    cmp::min,
    fs::{FileTimes, OpenOptions},
    io::{Read, Seek, Write},
};

use chrono::{DateTime, Utc};

pub struct FileReader<'a> {
    path: &'a str,
    file: std::fs::File,
    pos: u64,
}

impl<'a> FileReader<'a> {
    pub fn new(path: &'a str) -> Self {
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        file.rewind().unwrap();

        Self { path, file, pos: 0 }
    }

    pub fn export(
        &mut self,
        offset: u64,
        len: u64,
        path: &str,
        modified: DateTime<Utc>,
        buffer_size: u64,
    ) {
        let mut target = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        let pos_before = self.get_position();
        self.seek(offset);
        let mut buf = vec![0; buffer_size as usize];
        let mut remaining = len;

        while remaining > 0 {
            let to_read = min(buffer_size as u64, remaining) as usize;
            let read = self.read(&mut buf[..to_read]);
            target.write_all(read).unwrap();
            remaining -= to_read as u64;
        }

        let time = FileTimes::new().set_modified(modified.into());
        target.set_times(time).unwrap();

        self.seek(pos_before);
    }

    pub fn get_path(&self) -> &str {
        self.path
    }

    pub fn seek(&mut self, pos: u64) {
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        self.pos = pos;
    }

    pub fn rewind(&mut self) {
        self.seek(0);
    }

    pub fn jump(&mut self, offset: i128) {
        self.seek((self.pos as i128 + offset) as u64);
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

    pub fn read_utf8(&mut self, len: u64) -> String {
        let mut buf = vec![0; len as usize];
        self.read(&mut buf);
        String::from_utf8(buf).unwrap()
    }

    pub fn read_u8array(&mut self, len: u64) -> Vec<u8> {
        let mut buf = vec![0; len as usize];
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
}

pub struct FileWriter<'a> {
    path: &'a str,
    file: std::fs::File,
    pos: u64,
}

impl<'a> FileWriter<'a> {
    pub fn new(path: &'a str, append: bool) -> Self {
        if append {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(path)
                .unwrap();
            file.rewind().unwrap();
            return Self {
                path,
                pos: file.metadata().unwrap().len(),
                file,
            };
        }

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        file.rewind().unwrap();

        Self { path, file, pos: 0 }
    }

    pub fn get_path(&self) -> &str {
        self.path
    }

    pub fn seek(&mut self, pos: u64) {
        self.file.seek(std::io::SeekFrom::Start(pos)).unwrap();
        self.pos = pos;
    }

    pub fn rewind(&mut self) {
        self.seek(0);
    }

    pub fn jump(&mut self, offset: i128) {
        self.seek((self.pos as i128 + offset) as u64);
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

    pub fn write_utf8(&mut self, s: &str) {
        self.write(s.as_bytes());
    }

    pub fn write_u8array(&mut self, buf: &Vec<u8>) {
        self.write(buf.as_slice());
    }
}
