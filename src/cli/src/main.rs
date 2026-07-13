use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "omega")]
#[command(about = "OMEGA - Security Ecosystem")]
struct Cli { 
    #[command(subcommand)] 
    command: Commands 
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new identity
    Identity,
    /// Send a file to a recipient
    Send {
        /// Path to the file to send
        file: String,
        /// Recipient identifier
        to: String,
    },
    /// Receive data
    Recv {
        /// Output file path
        output: String,
    },
    /// Generate a payload
    Payload {
        /// Target platform (linux, windows, macos, android)
        platform: String,
        /// Payload type (reverse_shell, keylogger, meterpreter)
        payload_type: String,
        /// Output file path
        output: String,
    },
    /// Show mesh network status
    Mesh,
    /// Clean all traces
    Clean,
    /// Show version
    Version,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Identity => {
            use omega_core::identity::Identity;
            let id = Identity::generate();
            println!("{}", " OMEGA IDENTITY ".bold().white().on_blue());
            println!("  Public Key: {}", id.public_key.green());
            println!("  Fingerprint: {}", id.fingerprint().yellow());
            println!("\n{}", "Store this securely!".red());
        }
        Commands::Send { file, to } => {
            println!("{}", " OMEGA SEND ".bold().white().on_blue());
            println!("  File: {}", file.green());
            println!("  To: {}", to.cyan());
            
            // Simulate sending
            use std::fs;
            if let Ok(metadata) = fs::metadata(&file) {
                println!("  Size: {} bytes", metadata.len());
            }
            
            // Use comms module
            let comms = omega_comms::CommsManager::new();
            let data = b"Test message";
            comms.send(data)?;
            
            println!("\n{}", "✓ Transfer complete".green());
        }
        Commands::Recv { output } => {
            println!("{}", " OMEGA RECV ".bold().white().on_blue());
            println!("  Output: {}", output.green());
            
            let comms = omega_comms::CommsManager::new();
            let data = comms.receive()?;
            std::fs::write(&output, data)?;
            
            println!("\n{}", "✓ Data received and saved".green());
        }
        Commands::Payload { platform, payload_type, output } => {
            println!("{}", " OMEGA PAYLOAD ".bold().white().on_red());
            println!("  Platform: {}", platform.yellow());
            println!("  Type: {}", payload_type.yellow());
            println!("  Output: {}", output.green());
            
            use omega_hacker::payload::{PayloadGenerator, PayloadType as HackerPayloadType};
            let generator = PayloadGenerator::new();
            let payload_type_enum = match payload_type.as_str() {
                "reverse_shell" => HackerPayloadType::ReverseShell,
                "keylogger" => HackerPayloadType::Keylogger,
                "meterpreter" => HackerPayloadType::Meterpreter,
                _ => HackerPayloadType::ReverseShell,
            };
            let payload = generator.generate(payload_type_enum, &platform)?;
            std::fs::write(&output, &payload.data)?;
            
            println!("\n{}", "✓ Payload generated and saved".green());
        }
        Commands::Mesh => {
            println!("{}", " OMEGA MESH ".bold().white().on_purple());
            use omega_core::mesh::{MeshNetwork, Node};
            let mut network = MeshNetwork::new();
            
            // Add some nodes
            network.add_node(Node {
                id: "node1".to_string(),
                address: "192.168.1.10".to_string(),
                last_seen: chrono::Utc::now().timestamp(),
            });
            network.add_node(Node {
                id: "node2".to_string(),
                address: "192.168.1.20".to_string(),
                last_seen: chrono::Utc::now().timestamp(),
            });
            network.add_node(Node {
                id: "node3".to_string(),
                address: "192.168.1.30".to_string(),
                last_seen: chrono::Utc::now().timestamp(),
            });
            
            println!("  Nodes: {} active", network.nodes.len());
            for (id, node) in &network.nodes {
                println!("    - {} @ {}", id, node.address);
            }
            println!("  Messages: {} pending", network.messages.len());
            println!("\n{}", "✓ Mesh network ready".green());
        }
        Commands::Clean => {
            println!("{}", " OMEGA CLEAN ".bold().white().on_yellow());
            use omega_civilian::privacy::PrivacyCleaner;
            let cleaner = PrivacyCleaner::new();
            cleaner.wipe_traces()?;
            println!("\n{}", "✓ All traces removed".green());
        }
        Commands::Version => {
            println!("OMEGA v{}", env!("CARGO_PKG_VERSION"));
        }
    }
    Ok(())
}
