use anyhow::Result;

pub struct Rootkit {
    pub name: String,
    pub installed: bool,
}

impl Rootkit {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), installed: false }
    }
    pub fn install(&mut self) -> Result<()> {
        self.installed = true;
        println!("Rootkit '{}' installed", self.name);
        Ok(())
    }
}
