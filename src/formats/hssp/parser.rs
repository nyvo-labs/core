use crate::{
    file::{DataReader, VirtualFileReader},
    helpers::{
        encryption::aes256cbc,
        hash::{murmur3, sha256},
    },
};

use super::{HsspEncryption, HsspFileEntry, HsspMetadata};

pub fn metadata(file: &mut dyn DataReader, password: Option<&String>) -> HsspMetadata {
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
        let mut is_empty = true;
        file.read_u8array(&64).into_iter().for_each(|byte| {
            if !is_empty {
                return;
            }
            if byte != 0 {
                is_empty = false;
            }
        });
        if is_empty {
            version = 3;
        } else {
            file.jump(&-64);
        }
    }

    let encrypted = !(pwd_hash == [0; 32] && iv == [0; 16]);

    let mut decrypted_data: Option<Vec<u8>> = None;
    let body: &mut dyn DataReader = if encrypted {
        let key = sha256::hash_buf(password.unwrap_or(&"".to_string()).as_bytes());
        let in_hash = sha256::hash_buf(&key);
        if in_hash != pwd_hash {
            return HsspMetadata {
                version,
                checksum,
                encryption: Some(HsspEncryption {
                    hash: pwd_hash,
                    iv,
                    in_hash,
                    data: None,
                }),
                files: Vec::new(),
                has_main: false,
            };
        } else {
            let data = file.read_u8array(&{ file.get_size() - if version > 2 { 128 } else { 64 } });
            decrypted_data = Some(aes256cbc::decrypt(&data, &key, &iv));
            &mut VirtualFileReader::new(decrypted_data.as_mut().unwrap())
        }
    } else {
        file
    };

    let mut files = Vec::new();

    for idx in 0..file_count {
        let size = body.read_u64le();
        let name_len = body.read_u16le();
        let mut name = body.read_utf8(&(name_len as u64));
        let is_directory = name.starts_with("//");
        if is_directory {
            name = name[2..].to_string();
        }
        let offset = body.get_position();
        files.push(HsspFileEntry {
            name,
            offset,
            size,
            is_main: idx + 1 == main,
            is_directory,
        });
        body.jump(&(size as i128 + name_len as i128)); // that actually was a bug
    }

    HsspMetadata {
        version,
        checksum,
        encryption: if encrypted {
            Some(HsspEncryption {
                hash: pwd_hash,
                iv,
                in_hash: pwd_hash,
                data: decrypted_data,
            })
        } else {
            None
        },
        files,
        has_main: main != 0,
    }
}

pub fn check_integrity_all(file: &mut dyn DataReader, metadata: &HsspMetadata) -> bool {
    let offset = if metadata.version > 2 { 128 } else { 64 };
    let size = file.get_size() - offset;
    if murmur3::hash(file, &offset, &size, &822616071) == metadata.checksum {
        return true;
    }
    false
}

pub fn get_file(
    reader: &mut dyn DataReader,
    metadata: &HsspMetadata,
    entry: &HsspFileEntry,
) -> Vec<u8> {
    if metadata.encryption.is_some() {
        let mut body =
            VirtualFileReader::new(metadata.encryption.as_ref().unwrap().data.as_ref().unwrap());
        body.seek(&entry.offset);
        body.read_u8array(&entry.size)
    } else {
        reader.seek(&entry.offset);
        reader.read_u8array(&entry.size)
    }
}
