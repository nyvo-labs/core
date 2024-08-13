use libaes::Cipher;

pub fn decrypt(data: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> Vec<u8> {
    let cipher = Cipher::new_256(key);
    cipher.cbc_decrypt(iv, data)
}
