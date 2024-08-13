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
        let bytes = file.read_u128le();
        if bytes == 0 {
            version = 3;
        } else {
            file.jump(&-16);
        }
    }

    let encrypted = !(pwd_hash == [0; 32] && iv == [0; 16]);

    let decrypted_data: Vec<u8>;
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
                }),
                files: Vec::new(),
                has_main: false,
            };
        } else {
            let data = file.read_u8array(&{ file.get_size() - if version > 2 { 128 } else { 64 } });
            decrypted_data = aes256cbc::decrypt(&data, &key, &iv);
            &mut VirtualFileReader::new(&decrypted_data)
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
