use anyhow::Result;

pub struct Firewall {
    pub enabled: bool,
}

impl Firewall {
    pub fn new() -> Self { Self { enabled: false } }
    pub fn enable(&mut self) { self.enabled = true; }
    pub fn block_ip(&self, ip: &str) -> Result<()> {
        println!("Blocking IP: {}", ip);
        Ok(())
    }
}
