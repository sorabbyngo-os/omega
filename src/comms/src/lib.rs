pub mod qr;
pub mod audio;
pub mod wifi;
pub use omega_core;
#[derive(Debug, Clone, Copy)]
pub enum Medium { QR, Audio, WiFi }
pub struct CommsManager { pub active_medium: Medium }
impl CommsManager {
    pub fn new() -> Self { Self { active_medium: Medium::QR } }
    pub fn send(&self, data: &[u8]) -> Result<(), anyhow::Error> {
        println!("Sending {} bytes via {:?}", data.len(), self.active_medium);
        Ok(())
    }
    pub fn receive(&self) -> Result<Vec<u8>, anyhow::Error> {
        Ok(vec![0x48, 0x65, 0x6c, 0x6c, 0x6f])
    }
}
