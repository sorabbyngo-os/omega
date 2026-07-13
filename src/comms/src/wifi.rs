use anyhow::Result;

pub struct WiFiMesh {
    nodes: Vec<String>,
}

impl WiFiMesh {
    pub fn new() -> Self { Self { nodes: Vec::new() } }
    pub fn discover(&mut self) -> Result<Vec<String>> {
        self.nodes.push("node1".to_string());
        self.nodes.push("node2".to_string());
        Ok(self.nodes.clone())
    }
    pub fn send(&self, data: &[u8], to: &str) -> Result<()> {
        println!("Sending {} bytes to {}", data.len(), to);
        Ok(())
    }
}
