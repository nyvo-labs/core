use crate::{file::FileReader, helpers::hash::murmur3};

use super::{HsspEncryption, HsspFileEntry, HsspMetadata};

pub fn metadata(file: &mut FileReader) -> HsspMetadata {
    let mut version = 2;
    let magic = file.read_utf8(&4);
    if magic == "SFA\0" {
        version = 1;
    }
    let checksum = file.read_u32le();
    let file_count = file.read_u32le();
    let pwd_hash: [u8; 32] = file.read_u8array(&32).try_into().unwrap();
    let iv: [u8; 16] = file.read_u8array(&16).try_into().unwrap();
    let main = file.read_u32le();
    if version == 2 {
        let bytes = file.read_u128le();
        if bytes == 0 {
            version = 3;
        } else {
            file.jump(&-16);
        }
    }

    let encrypted = !(pwd_hash == [0; 32] && iv == [0; 16]); // TODO: handle encryption

    let mut files = Vec::new();

    for idx in 0..file_count {
        let size = file.read_u64le();
        println!("size: {}", size);
        let name_len = file.read_u16le();
        println!("name_len: {}", name_len);
        let mut name = file.read_utf8(&(name_len as u64));
        let is_directory = name.starts_with("//");
        if is_directory {
            name = name[2..].to_string();
        }
        let offset = file.get_position();
        files.push(HsspFileEntry {
            name,
            offset,
            size,
            is_main: idx + 1 == main,
            is_directory,
        });
        file.jump(&(size as i128 + name_len as i128)); // that actually was a bug
    }

    HsspMetadata {
        version,
        checksum,
        encryption: if encrypted {
            Some(HsspEncryption { hash: pwd_hash, iv })
        } else {
            None
        },
        files,
        has_main: main != 0,
    }
}

pub fn check_integrity_all(file: &mut FileReader, metadata: &HsspMetadata) -> bool {
    let offset = if metadata.version > 2 { 128 } else { 64 };
    if murmur3::hash(file, &offset, &(file.get_size() - offset), &822616071) == metadata.checksum {
        return true;
    }
    false
}
