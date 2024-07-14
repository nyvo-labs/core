use std::cmp::min;

use crc32fast::Hasher;

use crate::FileReader;

pub fn hash(file: &mut FileReader, offset: u64, size: u64, buffer_size: u64) -> u32 {
    let pos_before = file.get_position();
    file.seek(offset);

    let mut hasher = Hasher::new();

    let mut buf = vec![0; buffer_size as usize];

    let mut remaining = size;
    while remaining > 0 {
        let to_read = min(buffer_size as u64, remaining) as usize;
        let read = file.read(&mut buf[..to_read]);
        hasher.update(&read);
        remaining -= to_read as u64;
    }

    file.seek(pos_before);
    hasher.finalize()
}
