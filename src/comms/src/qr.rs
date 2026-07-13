use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub struct QRTransfer;

impl QRTransfer {
    pub fn new() -> Self { Self }
    pub fn generate(&self, data: &[u8]) -> Result<Vec<String>> {
        let encoded = STANDARD.encode(data);
        let chunks: Vec<String> = encoded.chars()
            .collect::<Vec<char>>()
            .chunks(100)
            .map(|c| c.iter().collect())
            .collect();
        Ok(chunks)
    }
    pub fn decode(&self, chunks: &[String]) -> Result<Vec<u8>> {
        let combined: String = chunks.concat();
        Ok(STANDARD.decode(combined)?)
    }
}
