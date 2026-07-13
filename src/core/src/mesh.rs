use serde::{Serialize, Deserialize};
use chrono::Utc;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub address: String,
    pub last_seen: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub payload: Vec<u8>,
    pub timestamp: i64,
    pub ttl: u32,
}

pub struct MeshNetwork {
    pub nodes: HashMap<String, Node>,
    pub messages: Vec<Message>,
}

impl MeshNetwork {
    pub fn new() -> Self {
        Self { nodes: HashMap::new(), messages: Vec::new() }
    }
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }
    pub fn add_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }
    pub fn prune_expired(&mut self) {
        let now = Utc::now().timestamp();
        self.messages.retain(|m| now - m.timestamp < m.ttl as i64);
    }
}
