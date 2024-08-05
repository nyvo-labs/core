use crate::{file::FileReader, helpers::hash::crc32};

use super::{RarArchiveMetadata, RarEncryption};

pub fn metadata(file: &mut FileReader) -> RarArchiveMetadata {
    let mut i = 0;
    let maj_version;
    let min_version;
    let mut max_size = file.get_size();
    if max_size > 10_000_000 {
        max_size = 10_000_000;
    }
    loop {
        if i >= max_size {
            panic!("Could not find RAR signature");
        }
        let mut buf = [0; 8];
        let bytes = file.read(&mut buf);
        if bytes == b"Rar!\x1A\x07\x01\x00" {
            maj_version = 5;
            min_version = 0;
            break;
        }
        file.seek(&i);
        i += 1;
    }

    // TODO: Encryption

    parse_header(file);

    RarArchiveMetadata {
        files: vec![],
        archive_start: i,
        version: (maj_version, min_version),
    }
}

struct HeaderFlags {
    extra_area: bool,
    data_area: bool,
    skip_when_unknown: bool,
    continues_prev: bool,
    continues_next: bool,
    depends_on_prev: bool,
    preserve_child: bool,

    raw: u128,
}

struct Header {
    header: HeaderType,
    checksum: u32,
    offset: u64,
    size: u128,
    extra_size: u128,
    data_size: u128,
    flags: HeaderFlags,
}

struct EncryptionHeader {
    algorithm: RarEncryption,
    kdf_count: u8,
    salt: u128,
    password_check_value: Option<(u64, u16)>,
}

enum HeaderType {
    Main,
    Encryption(EncryptionHeader),
}

fn parse_header(file: &mut FileReader) -> Header {
    let crc = file.read_u32le();
    let header_offset = file.get_position();
    let size = file.read_vu7();
    let header_type = file.read_vu7();
    let flags = file.read_vu7();
    let flags = HeaderFlags {
        extra_area: flags & 0x1 != 0,
        data_area: flags & 0x2 != 0,
        skip_when_unknown: flags & 0x4 != 0,
        continues_prev: flags & 0x8 != 0,
        continues_next: flags & 0x10 != 0,
        depends_on_prev: flags & 0x20 != 0,
        preserve_child: flags & 0x40 != 0,
        raw: flags,
    };
    let extra_size = if flags.extra_area { file.read_vu7() } else { 0 };
    let data_size = if flags.data_area { file.read_vu7() } else { 0 };
    match header_type {
        1 => {
            file.jump(&(extra_size as i128));
            file.jump(&(data_size as i128));
            Header {
                header: HeaderType::Main,
                checksum: crc,
                offset: header_offset,
                size,
                extra_size,
                data_size,
                flags,
            }
        }
        4 => {
            let algorithm = match file.read_vu7() {
                0 => RarEncryption::Aes256,
                _ => panic!("Unknown encryption algorithm"),
            };
            let enc_flags = file.read_vu7();
            let kdf_count = file.read_u8();
            let salt = file.read_u128le();
            let password_check_value = if enc_flags & 0x1 != 0 {
                Some((file.read_u64le(), file.read_u16le()))
            } else {
                None
            };
            Header {
                header: HeaderType::Encryption(EncryptionHeader {
                    algorithm,
                    kdf_count,
                    salt,
                    password_check_value,
                }),
                checksum: crc,
                offset: header_offset,
                size,
                extra_size,
                data_size,
                flags,
            }
        }
        _ => panic!("Unknown header type {}", header_type),
    }
}
