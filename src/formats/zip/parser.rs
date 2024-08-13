use super::{ZipArchiveMetadata, ZipFileEntry};
use crate::{
    file::{DataReader, FileWriter},
    helpers::{datetime::msdos, hash::crc32},
};

pub fn metadata<'a>(file: &mut dyn DataReader) -> ZipArchiveMetadata<'a> {
    let local_files = read_local_files(file);

    //let signature = local_files.1;

    //if signature == 0x02014b50 {}

    //println!("0x{:x}", signature);
    ZipArchiveMetadata {
        files: local_files.0,
    }
}

pub fn get_file(file: &mut dyn DataReader, entry: &ZipFileEntry) -> Vec<u8> {
    file.seek(&entry.offset);
    file.read_u8array(&(entry.uncompressed_size as u64))
}

pub fn extract(
    file: &mut dyn DataReader,
    entries: &Vec<ZipFileEntry>,
    buffer_size: &u64,
    path_rewriter: &dyn Fn(&String) -> String,
) {
    for entry in entries {
        let path = path_rewriter(&entry.path);
        if !entry.is_directory {
            let mut target = FileWriter::new(&path, &false);
            file.export(
                &entry.offset,
                &entry.size,
                &mut target,
                &entry.modified,
                buffer_size,
            );
        } else {
            std::fs::create_dir_all(path).unwrap();
        };
    }
}

fn read_local_files<'a>(file: &mut dyn DataReader) -> (Vec<ZipFileEntry<'a>>, u32) {
    let mut files: Vec<ZipFileEntry> = Vec::new();

    let mut signature: u32 = file.read_u32le();
    while signature == 0x04034b50 {
        let version = file.read_u16le();
        let bit_flag = file.read_u16le();
        let compression_method = match file.read_u16le() {
            0 => "stored",      // The file is stored (no compression)
            1 => "shrunk",      // The file is Shrunk
            2 => "reduced1",    // The file is Reduced with compression factor 1
            3 => "reduced2",    // The file is Reduced with compression factor 2
            4 => "reduced3",    // The file is Reduced with compression factor 3
            5 => "reduced4",    // The file is Reduced with compression factor 4
            6 => "imploded",    // The file is Imploded
            7 => "tokenizing",  // Reserved for Tokenizing compression algorithm
            8 => "deflated",    // The file is Deflated
            9 => "deflated64",  // Enhanced Deflating using Deflate64(tm)
            10 => "dcli",       // PKWARE Data Compression Library Imploding (old IBM TERSE)
            11 => "reserved",   // Reserved by PKWARE
            12 => "bzip2",      // File is compressed using BZIP2 algorithm
            13 => "reserved2",  // Reserved by PKWARE
            14 => "lzma",       // LZMA
            15 => "reserved3",  // Reserved by PKWARE
            16 => "cmpsc",      // IBM z/OS CMPSC Compression
            17 => "reserved4",  // Reserved by PKWARE
            18 => "terse",      // IBM TERSE (new)
            19 => "lz77",       // IBM LZ77 z Architecture (PFS)
            20 => "deprecated", // deprecated (use method 93 for zstd)
            93 => "zstd",       // Zstandard
            94 => "mp3",        // MP3 Compression
            95 => "xz",         // XZ Compression
            96 => "jpeg",       // JPEG variant
            97 => "wavpack",    // WavPack compressed data
            98 => "ppmd",       // PPMd version I, Rev 1
            99 => "aes",        // AE-x encryption (see APPENDIX E)
            _ => "unknown",
        };
        let lastmod_time = file.read_u16le();
        let lastmod_date = file.read_u16le();
        let crc32 = file.read_u32le();
        let size_compressed = file.read_u32le() as i128;
        let size_uncompressed = file.read_u32le();
        let name_length = file.read_u16le() as u64;
        let extra_length = file.read_u16le() as u64;
        let name = file.read_utf8(&name_length);
        let extra = file.read_u8array(&extra_length);
        files.push(ZipFileEntry {
            offset: file.get_position(),
            size: size_compressed as u64,
            modified: msdos::parse(&lastmod_date, &lastmod_time).into(),
            is_directory: name.ends_with('/'),
            path: name,
            version,
            bit_flag,
            compression: compression_method,
            uncompressed_size: size_uncompressed,
            checksum: crc32,
            extra_field: extra,
        });
        file.jump(&size_compressed);
        signature = file.read_u32le();
    }

    (files, signature)
}

pub fn check_integrity(
    source: &mut dyn DataReader,
    file: &ZipFileEntry,
    buffer_size: &u64,
) -> bool {
    let hash = crc32::hash(source, &file.offset, &file.size, buffer_size);
    hash == file.checksum
}

pub fn check_integrity_all(
    source: &mut dyn DataReader,
    files: &Vec<ZipFileEntry>,
    buffer_size: &u64,
) -> bool {
    for file in files {
        if !check_integrity(source, file, buffer_size) {
            return false;
        }
    }
    true
}
