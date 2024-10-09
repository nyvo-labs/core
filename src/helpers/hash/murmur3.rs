use crate::file::Readable;
use murmur3::murmur3_32;

// Buffer size is fixed to 4 Bytes
pub fn hash(file: &mut dyn Readable, offset: &u64, size: &u64, seed: &u32) -> u32 {
    let pos_before = file.get_position();
    let mut file = file.partial(offset, &(offset + size));
    file.rewind();
    let result = murmur3_32(&mut file, *seed).unwrap();
    file.seek(&pos_before);
    result
}
