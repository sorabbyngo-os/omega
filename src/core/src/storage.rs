use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct SecureStore {
    pub data: Vec<u8>,
    pub encrypted: bool,
}

impl SecureStore {
    pub fn new() -> Self {
        Self { data: Vec::new(), encrypted: false }
    }
    pub fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string(self)?;
        fs::write(path, json)
    }
    pub fn load(path: &Path) -> Result<Self, std::io::Error> {
        let json = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
}
