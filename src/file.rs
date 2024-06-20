use std::{
    fs::OpenOptions,
    io::{Read, Seek},
};

pub struct File<'a> {
    path: &'a str,
    file: std::fs::File,
    pos: u64,
}

impl<'a> File<'a> {
    pub fn new(path: &'a str) -> Self {
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        file.rewind().unwrap();
        
        Self {
            path,
            file,
            pos: 0,
        }
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

    pub fn read<'b>(&mut self, buf: &'b mut [u8]) -> &'b mut [u8] {
        let _ = self.file.read_exact(buf);
        self.pos += buf.len() as u64;
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
