use crate::file::FileReader;

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

struct Locator {
    quick_open_offset: Option<u128>,
    recovery_record_offset: Option<u128>,
}

struct Metadata {
    name: Option<String>,
    created: Option<u64>,
}

struct MainHeader {
    multivolume: bool,
    volume: u128,
    solid: bool,
    has_recovery: bool,
    locked: bool,
    locator: Option<Locator>,
    metadata: Option<Metadata>,
}

struct EncryptionHeader {
    algorithm: RarEncryption,
    kdf_count: u8,
    salt: u128,
    password_check_value: Option<(u64, u16)>,
}

enum HeaderType {
    Main(MainHeader),
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
            let archive_flags = file.read_vu7();
            let mut volume = 0;
            if archive_flags & 0x2 != 0 {
                volume = file.read_vu7();
            }
            let mut locator = None;
            let mut metadata = None;
            if flags.extra_area {
                let start_offset = file.get_position();
                let end = start_offset + (extra_size as u64);
                while file.get_position() < end {
                    let entry_size = file.read_vu7();
                    let entry_type = file.read_vu7();
                    match entry_type {
                        1 => {
                            let locator_flags = file.read_vu7();
                            let mut quick_open_offset = None;
                            if locator_flags & 0x1 != 0 {
                                quick_open_offset = Some(file.read_vu7());
                            }
                            let mut recovery_record_offset = None;
                            if locator_flags & 0x2 != 0 {
                                recovery_record_offset = Some(file.read_vu7());
                            }
                            locator = Some(Locator {
                                quick_open_offset,
                                recovery_record_offset,
                            });
                        }
                        2 => {
                            let meta_flags = file.read_vu7();
                            let name = if meta_flags & 0x1 != 0 {
                                let name_length = file.read_vu7() as u64;
                                Some(
                                    file.read_utf8(&name_length)
                                        .split('\0')
                                        .collect::<Vec<&str>>()[0]
                                        .to_string(),
                                )
                            } else {
                                None
                            };
                            let created = if meta_flags & 0x2 != 0 {
                                if meta_flags & 0x4 != 0 || meta_flags & 0x8 != 0 {
                                    // TODO: Parse
                                    Some(file.read_u64le())
                                } else {
                                    Some(file.read_u32le() as u64)
                                }
                            } else {
                                None
                            };
                            metadata = Some(Metadata { name, created });
                        }
                        _ => {
                            file.jump(&(entry_size as i128));
                        }
                    }
                }
            } else {
                file.jump(&(extra_size as i128));
            }
            file.jump(&(data_size as i128));
            Header {
                header: HeaderType::Main(MainHeader {
                    multivolume: archive_flags & 0x1 != 0,
                    volume,
                    solid: archive_flags & 0x4 != 0,
                    has_recovery: archive_flags & 0x8 != 0,
                    locked: archive_flags & 0x10 != 0,
                    locator,
                    metadata,
                }),
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
