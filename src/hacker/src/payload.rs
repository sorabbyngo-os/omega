use anyhow::Result;

pub enum PayloadType { ReverseShell, Keylogger, Meterpreter }

pub struct Payload {
    pub payload_type: PayloadType,
    pub platform: String,
    pub data: Vec<u8>,
}

pub struct PayloadGenerator;

impl PayloadGenerator {
    pub fn new() -> Self { Self }
    pub fn generate(&self, payload_type: PayloadType, platform: &str) -> Result<Payload> {
        Ok(Payload {
            payload_type,
            platform: platform.to_string(),
            data: vec![0x7f, 0x45, 0x4c, 0x46],
        })
    }
}
