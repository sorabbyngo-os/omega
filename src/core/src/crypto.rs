use rand::Rng;
use anyhow::Result;

pub fn generate_key(len: usize) -> Result<Vec<u8>> {
    let mut key = vec![0u8; len];
    rand::thread_rng().fill(&mut key[..]);
    Ok(key)
}

pub fn simple_encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut result = data.to_vec();
    for (i, byte) in result.iter_mut().enumerate() {
        *byte ^= key[i % key.len()];
    }
    Ok(result)
}

pub fn simple_decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    simple_encrypt(data, key)
}
