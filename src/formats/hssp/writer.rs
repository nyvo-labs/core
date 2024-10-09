use crate::helpers::hash::{murmur3, sha256};
use dh::{recommended::*, Rw, Writable};

use super::HsspArchiveData;

pub fn write(target: &mut dyn Rw, data: HsspArchiveData, buffer_size: &u64) {
    if data.version == 1 {
        target.write(b"SFA\0");
    } else {
        target.write(b"HSSP");
    }

    Writable::jump(target, &4);

    target.write_u32le(&(data.files.len() as u32));

    let encrypted = data.encryption.is_some();
    let key;
    let iv;
    if encrypted {
        key = Some(sha256::hash_buf(&data.encryption.as_ref().unwrap().key));
        target.write(&sha256::hash_buf(key.as_ref().unwrap()));
        iv = Some(data.encryption.as_ref().unwrap().iv);
        target.write(iv.as_ref().unwrap());
        Writable::jump(target, &4);
    } else {
        Writable::jump(target, &52);
        key = None;
        iv = None;
    }

    let mut main = None;

    if data.version > 2 {
        Writable::jump(target, &64);
    }

    if encrypted {
        todo!();
    }

    for (i, (file, reader)) in data.files.into_iter().enumerate() {
        if file.is_main {
            main = Some(i);
        }

        target.write_u64le(if file.is_directory { &0 } else { &file.size });
        target.write_u16le(&(file.name.len() as u16 + if file.is_directory { 2 } else { 0 }));
        target.write_utf8(
            &(if file.is_directory {
                "//".to_string() + &file.name
            } else {
                file.name
            }),
        );

        if let Some(mut reader) = reader {
            reader.seek(&file.offset);
            reader
                .stream_into_rw(target, &file.size, buffer_size)
                .unwrap()
        }
    }

    Writable::seek(target, &4);
    let hash;
    {
        let size = target.size().unwrap();
        let offset = if data.version > 2 { 128 } else { 64 };
        hash = murmur3::hash(
            &mut target.rw_limit(&0, &size),
            &offset,
            &(size - offset),
            &822616071,
        );
    }
    target.write_u32le(&hash);
    Writable::seek(target, &60);
    target.write_u32le(&(main.unwrap_or(0) as u32));
}
