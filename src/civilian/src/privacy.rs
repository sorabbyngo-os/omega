use anyhow::Result;
use std::fs;

pub struct PrivacyCleaner;

impl PrivacyCleaner {
    pub fn new() -> Self { Self }
    pub fn wipe_traces(&self) -> Result<()> {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
        let _ = fs::remove_file(format!("{}/.bash_history", home));
        let _ = fs::remove_file(format!("{}/.zsh_history", home));
        println!("Traces wiped!");
        Ok(())
    }
}
