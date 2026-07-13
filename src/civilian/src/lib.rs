pub mod firewall;
pub mod privacy;

use anyhow::Result;

pub struct CivilianSuite;

impl CivilianSuite {
    pub fn new() -> Self { Self }
    pub fn enable_firewall(&self) -> Result<()> {
        println!("Firewall enabled");
        Ok(())
    }
}
