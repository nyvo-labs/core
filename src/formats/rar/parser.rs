use chrono::DateTime;

use crate::{
    file::FileReader,
    formats::rar::{RarCompression, RarPlatform},
    helpers::hash::crc32,
};

use super::{RarArchiveMetadata, RarEncryption, RarFileEntry};

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

    let mut headers = vec![];

    let file_size = file.get_size();
    while file.get_position() < file_size {
        headers.push(parse_header(file));
    }

    let encrypted = matches!(headers[0].header, HeaderType::Encryption(_));

    let main_header = match if encrypted {
        &headers[1].header
    } else {
        &headers[0].header
    } {
        HeaderType::Main(header) => header,
        _ => panic!("Invalid RAR file"),
    };

    let files: Vec<RarFileEntry> = headers
        .iter()
        .filter_map(|header| match &header.header {
            HeaderType::File(file) => Some(file),
            _ => None,
        })
        .map(|file| {
            let dict_size = ((file.compression_info & 0x7c00) >> 10) as u64;
            let dict_size_add = ((file.compression_info & 0x7c00) >> 10) as u64;
            let compression = RarCompression {
                version: (file.compression_info & 0x3f) as u8,
                solid: file.compression_info & 0x40 != 0,
                method: ((file.compression_info & 0x380) >> 7) as u8,
                dict_size: (dict_size
                    + if dict_size_add > 0 {
                        (dict_size * dict_size_add) / 32
                    } else {
                        0
                    }),
            };
            RarFileEntry {
                path: file.name.clone(),
                offset: file.offset,
                size: file.size as u64,
                uncompressed_size: file.size_uncompressed.map(|size| size as u64),
                is_directory: file.is_directory,
                modified: DateTime::from_timestamp(0, 0), // TODO
                checksum: file.checksum,
                encryption: None, // TODO
                compression: if compression.method > 0 {
                    Some(compression)
                } else {
                    None
                },
                creation_platform: match file.created_with {
                    0 => Some(RarPlatform::Windows),
                    1 => Some(RarPlatform::Unix),
                    _ => None,
                },
            }
        })
        .collect();

    let locator = main_header.locator.as_ref().unwrap();
    let qo_offset = locator.quick_open_offset.map(|offset| offset as u64);
    let rr_offset = locator.recovery_record_offset.map(|offset| offset as u64);

    RarArchiveMetadata {
        files,
        archive_start: i,
        version: (maj_version, min_version),
        multivolume: main_header.multivolume,
        volume: main_header.volume,
        solid: main_header.solid,
        has_recovery: main_header.has_recovery,
        locked: main_header.locked,
        original_name: main_header
            .metadata
            .as_ref()
            .and_then(|meta| meta.name.clone()),
        created: DateTime::from_timestamp(0, 0), // TODO
        qo_offset,
        rr_offset,
    }
}

pub fn get_file(file: &mut FileReader, entry: &RarFileEntry) -> Result<Vec<u8>, String> {
    if entry.compression.is_some() {
        return Err("Compressed RAR files are not supported yet.".to_string());
    }
    file.seek(&entry.offset);
    Ok(file.read_u8array(&entry.size))
}

pub fn check_integrity(
    source: &mut FileReader,
    file: &RarFileEntry,
    buffer_size: &u64,
) -> Result<Option<bool>, String> {
    if file.compression.is_some() {
        return Err("Compressed RAR files are not supported yet.".to_string());
    }
    let checksum = match file.checksum {
        Some(checksum) => checksum,
        None => return Ok(None),
    };

    let hash = crc32::hash(source, &file.offset, &file.size, buffer_size);
    Ok(Some(hash == checksum))
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Header {
    header: HeaderType,
    checksum: u32,
    offset: u64,
    size: u128,
    extra_size: u128,
    data_size: u128,
    flags: HeaderFlags,
}

#[derive(Debug)]
struct Locator {
    quick_open_offset: Option<u128>,
    recovery_record_offset: Option<u128>,
}

#[derive(Debug)]
struct Metadata {
    name: Option<String>,
    created: Option<u64>,
}

#[derive(Debug)]
struct MainHeader {
    multivolume: bool,
    volume: u128,
    solid: bool,
    has_recovery: bool,
    locked: bool,
    locator: Option<Locator>,
    metadata: Option<Metadata>,
}

#[derive(Debug)]
struct FileHeader {
    is_directory: bool,
    file_flags: u128,
    size_uncompressed: Option<u128>,
    size: u128,
    attributes: u128,
    modified: Option<u32>,
    checksum: Option<u32>,
    compression_info: u128,
    created_with: u128,
    name: String,
    offset: u64,
}

#[derive(Debug)]
struct ServiceHeader {
    size_uncompressed: Option<u128>,
    checksum: Option<u32>,
    compression_info: u128,
    created_with: u128,
    name: String,
}

#[derive(Debug)]
struct EncryptionHeader {
    algorithm: RarEncryption,
    kdf_count: u8,
    salt: u128,
    password_check_value: Option<(u64, u16)>,
}

#[derive(Debug)]
struct EndHeader {
    is_last_volume: bool,
}

#[derive(Debug)]
enum HeaderType {
    Main(MainHeader),
    File(FileHeader),
    Service(ServiceHeader),
    Encryption(EncryptionHeader),
    End(EndHeader),

    Unknown(u128),
}

fn parse_header(file: &mut FileReader) -> Header {
    let crc = file.read_u32le();
    let size = file.read_vu7();
    let header_offset = file.get_position();
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
        2 => {
            let file_flags = file.read_vu7();
            let size_uncompressed = if file_flags & 0x8 == 0 {
                Some(file.read_vu7())
            } else {
                None
            };
            let attributes = file.read_vu7();
            let modified = if file_flags & 0x2 != 0 {
                Some(file.read_u32le())
            } else {
                None
            };
            let checksum = if file_flags & 0x4 != 0 {
                Some(file.read_u32le())
            } else {
                None
            };
            let compression_info = file.read_vu7(); // TODO: Parse
            let created_with = file.read_vu7();
            let name_length = file.read_vu7() as u64;
            let name = file
                .read_utf8(&name_length)
                .split('\0')
                .collect::<Vec<&str>>()[0]
                .to_string();
            let mut size = size_uncompressed.unwrap_or(0);
            if extra_size > 0 {
                size = file.read_vu7();
            }
            let file_offset = file.get_position();
            file.jump(&(data_size as i128)); // TODO: parse records
            Header {
                header: HeaderType::File(FileHeader {
                    is_directory: file_flags & 0x1 != 0,
                    file_flags,
                    size_uncompressed,
                    size,
                    attributes,
                    modified,
                    checksum,
                    compression_info,
                    created_with,
                    name,
                    offset: file_offset,
                }),
                checksum: crc,
                offset: header_offset,
                size,
                extra_size,
                data_size,
                flags,
            }
        }
        3 => {
            let service_flags = file.read_vu7();
            let size_uncompressed = if service_flags & 0x8 == 0 {
                Some(file.read_vu7())
            } else {
                None
            };
            file.read_vu7();
            if service_flags & 0x2 != 0 {
                file.jump(&4);
            }
            let checksum = if service_flags & 0x4 != 0 {
                Some(file.read_u32le())
            } else {
                None
            };
            let compression_info = file.read_vu7(); // TODO: Parse
            let created_with = file.read_vu7();
            let name_length = file.read_vu7() as u64;
            let name = file.read_utf8(&name_length);
            file.jump(&(extra_size as i128));
            file.jump(&(data_size as i128)); // TODO: parse records
            Header {
                header: HeaderType::Service(ServiceHeader {
                    size_uncompressed,
                    checksum,
                    compression_info,
                    created_with,
                    name,
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
            file.jump(&(extra_size as i128));
            file.jump(&(data_size as i128));
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
        5 => Header {
            header: HeaderType::End(EndHeader {
                is_last_volume: file.read_vu7() & 0x1 == 0,
            }),
            checksum: crc,
            offset: header_offset,
            size,
            extra_size,
            data_size,
            flags,
        },
        _ => {
            file.jump(&(size as i128));
            file.jump(&(extra_size as i128));
            file.jump(&(data_size as i128));
            Header {
                header: HeaderType::Unknown(header_type),
                checksum: crc,
                offset: header_offset,
                size,
                extra_size,
                data_size,
                flags,
            }
        }
    }
}
