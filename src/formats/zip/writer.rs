use crate::{helpers::datetime::msdos, types::ZipArchiveData, FileWriter};

pub fn write(target: &mut FileWriter, data: &mut ZipArchiveData, buffer_size: u64) {
    for file in &mut data.files {
        target.write(b"PK\x03\x04");

        let version: u16 = 20;
        let bit_flag: u16 = 0;
        let compression_method: u16 = 0;
        let last_modified = msdos::serialize(file.file.modified);
        let lastmod_time = last_modified.1;
        let lastmod_date = last_modified.0;
        let crc32 = file.checksum;
        let size_compressed = file.file.size as u32;
        let size_uncompressed = file.file.size as u32;
        let name_length = file.file.path.len() as u16;
        let extra_field_length: u16 = 0;
        let name = &file.file.path;
        let extra: &Vec<u8> = &vec![];

        target.write_u16le(version);
        target.write_u16le(bit_flag);
        target.write_u16le(compression_method);
        target.write_u16le(lastmod_time);
        target.write_u16le(lastmod_date);
        target.write_u32le(crc32);
        target.write_u32le(size_compressed);
        target.write_u32le(size_uncompressed);
        target.write_u16le(name_length);
        target.write_u16le(extra_field_length);
        target.write_utf8(name);
        target.write_u8array(extra);

        file.file.source.export(
            file.file.offset,
            file.file.size,
            target,
            file.file.modified,
            buffer_size,
        );
    }
}
