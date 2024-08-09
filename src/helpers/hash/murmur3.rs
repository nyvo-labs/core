use crate::file::FileReader;
use murmur3::murmur3_32;

// Buffer size is fixed to 4 Bytes
pub fn hash(file: &mut FileReader, offset: &u64, size: &u64, seed: &u32) -> u32 {
    let pos_before = file.get_position();
    file.seek(offset);
    let mut reader = file.set_end(&(offset + size));
    let result = murmur3_32(&mut reader, *seed).unwrap();
    file.seek(&pos_before);
    result
}
