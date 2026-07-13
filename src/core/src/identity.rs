use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Identity {
    pub public_key: String,
    pub fingerprint: String,
    pub display_name: Option<String>,
}

impl Identity {
    pub fn generate() -> Self {
        let key: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        let fp: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        Self {
            public_key: key,
            fingerprint: fp,
            display_name: None,
        }
    }
    pub fn fingerprint(&self) -> String {
        self.fingerprint.clone()
    }
}
