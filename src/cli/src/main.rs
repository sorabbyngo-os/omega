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
    /// Generate QR code from text
    Qr {
        /// Text to encode in QR code
        text: String,
    },
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
            
            let data = std::fs::read(&file)?;
            println!("  Size: {} bytes", data.len());
            
            let comms = omega_comms::CommsManager::new();
            comms.send(&data)?;
            
            // Generate QR code
            println!("\n{}", " QR CODE FOR SCANNING ".bold().white().on_green());
            let qr_transfer = omega_comms::qr::QRTransfer::new();
            match qr_transfer.generate_terminal_qr(&data) {
                Ok(qr_ascii) => {
                    println!("\n{}", qr_ascii);
                    println!("{}", "📱 Scan this QR code with your phone camera".yellow());
                }
                Err(e) => {
                    println!("{}", format!("Could not generate QR: {}", e).red());
                }
            }
            
            println!("\n{}", "✓ Transfer complete".green());
        }
        Commands::Recv { output } => {
            println!("{}", " OMEGA RECV ".bold().white().on_blue());
            println!("  Output: {}", output.green());
            
            let comms = omega_comms::CommsManager::new();
            let data = comms.receive()?;
            std::fs::write(&output, &data)?;
            println!("  Received: {} bytes", data.len());
            
            if let Ok(text) = String::from_utf8(data.clone()) {
                println!("\n{}", " Received Text ".bold().white().on_green());
                println!("{}", text.cyan());
            }
            
            println!("\n{}", "✓ Data received and saved".green());
        }
        Commands::Payload { platform, payload_type, output } => {
            println!("{}", " OMEGA PAYLOAD ".bold().white().on_red());
            println!("  Platform: {}", platform.yellow());
            println!("  Type: {}", payload_type.yellow());
            println!("  Output: {}", output.green());
            
            use omega_hacker::payload::{PayloadGenerator, PayloadType};
            let gen = PayloadGenerator::new();
            let pt = match payload_type.as_str() {
                "keylogger" => PayloadType::Keylogger,
                "meterpreter" => PayloadType::Meterpreter,
                _ => PayloadType::ReverseShell,
            };
            let payload = gen.generate(pt, &platform)?;
            std::fs::write(&output, &payload.data)?;
            println!("  Size: {} bytes", payload.data.len());
            
            // Generate QR code for payload
            println!("\n{}", " QR CODE FOR PAYLOAD ".bold().white().on_green());
            let qr_transfer = omega_comms::qr::QRTransfer::new();
            match qr_transfer.generate_terminal_qr(&payload.data) {
                Ok(qr_ascii) => {
                    println!("\n{}", qr_ascii);
                    println!("{}", "📱 Scan this QR code to transfer the payload".yellow());
                }
                Err(e) => {
                    println!("{}", format!("Could not generate QR: {}", e).red());
                }
            }
            
            println!("\n{}", "✓ Payload generated and saved".green());
        }
        Commands::Mesh => {
            println!("{}", " OMEGA MESH ".bold().white().on_purple());
            use omega_core::mesh::{MeshNetwork, Node};
            let mut network = MeshNetwork::new();
            
            network.add_node(Node {
                id: "node1".to_string(),
                address: "192.168.1.10".to_string(),
                last_seen: 0,
            });
            network.add_node(Node {
                id: "node2".to_string(),
                address: "192.168.1.20".to_string(),
                last_seen: 0,
            });
            network.add_node(Node {
                id: "node3".to_string(),
                address: "192.168.1.30".to_string(),
                last_seen: 0,
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
            PrivacyCleaner::new().wipe_traces()?;
            println!("\n{}", "✓ All traces removed".green());
        }
        Commands::Version => {
            println!("OMEGA v{}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Qr { text } => {
            println!("{}", " QR CODE GENERATOR ".bold().white().on_blue());
            println!("  Text: {}", text.cyan());
            
            let qr_transfer = omega_comms::qr::QRTransfer::new();
            match qr_transfer.generate_terminal_qr(text.as_bytes()) {
                Ok(qr_ascii) => {
                    println!("\n{}", qr_ascii);
                    println!("\n{}", "📱 Scan this QR code with your phone camera".yellow());
                }
                Err(e) => {
                    println!("{}", format!("Error generating QR: {}", e).red());
                }
            }
        }
    }
    Ok(())
}
