use crate::file::DataReader;
use crate::file::DataReaderType;
use murmur3::murmur3_32;

// Buffer size is fixed to 4 Bytes
pub fn hash(file: &mut dyn DataReader, offset: &u64, size: &u64, seed: &u32) -> u32 {
    let pos_before = file.get_position();
    file.set_start(offset);
    file.set_end(&(offset + size));
    file.rewind();
    let result = match file.get_reader() {
        DataReaderType::File(reader) => murmur3_32(reader, *seed).unwrap(),
        DataReaderType::Virtual(ref mut reader) => murmur3_32(reader, *seed).unwrap(),
    };
    file.reset_start();
    file.reset_end();
    file.seek(&pos_before);
    result
}
