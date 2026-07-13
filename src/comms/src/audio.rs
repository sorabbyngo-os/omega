use anyhow::Result;

pub struct AudioModem;

impl AudioModem {
    pub fn new() -> Self { Self }
    pub fn send(&self, data: &[u8]) -> Result<()> {
        println!("Audio sending {} bytes", data.len());
        Ok(())
    }
    pub fn receive(&self) -> Result<Vec<u8>> {
        Ok(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f])
    }
}
