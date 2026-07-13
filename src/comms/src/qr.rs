use anyhow::Result;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub struct QRTransfer;

impl QRTransfer {
    pub fn new() -> Self { Self }
    
    pub fn generate(&self, data: &[u8]) -> Result<Vec<String>> {
        let encoded = STANDARD.encode(data);
        let chunks: Vec<String> = encoded.chars()
            .collect::<Vec<char>>()
            .chunks(100)
            .map(|c| c.iter().collect())
            .collect();
        Ok(chunks)
    }
    
    pub fn decode(&self, chunks: &[String]) -> Result<Vec<u8>> {
        let combined: String = chunks.concat();
        Ok(STANDARD.decode(combined)?)
    }
    
    // Generate QR code as ASCII art (scanable with phone)
    pub fn generate_ascii_qr(&self, data: &[u8]) -> Result<String> {
        use qrcode::QrCode;
        use qrcode::EcLevel;
        
        let qr = QrCode::with_error_correction_level(data, EcLevel::H)?;
        
        // Render as ASCII art
        let mut output = String::new();
        let width = qr.width();
        
        // Add border
        output.push_str("┌");
        for _ in 0..(width + 2) {
            output.push('─');
        }
        output.push_str("┐\n");
        
        for y in 0..width {
            output.push('│');
            output.push(' ');
            for x in 0..width {
                let pixel = qr[(x, y)];
                if pixel == qrcode::Color::Dark {
                    output.push('█');
                    output.push('█');
                } else {
                    output.push(' ');
                    output.push(' ');
                }
            }
            output.push(' ');
            output.push('│');
            output.push('\n');
        }
        
        output.push_str("└");
        for _ in 0..(width + 2) {
            output.push('─');
        }
        output.push_str("┘\n");
        
        Ok(output)
    }
    
    // Generate QR code as a text block that can be scanned
    pub fn generate_terminal_qr(&self, data: &[u8]) -> Result<String> {
        use qrcode::QrCode;
        use qrcode::EcLevel;
        
        let qr = QrCode::with_error_correction_level(data, EcLevel::H)?;
        
        let mut output = String::new();
        let width = qr.width();
        
        // Top border
        output.push_str("╔");
        for _ in 0..(width + 2) {
            output.push('═');
        }
        output.push_str("╗\n");
        
        // QR content
        for y in 0..width {
            output.push('║');
            output.push(' ');
            for x in 0..width {
                let pixel = qr[(x, y)];
                if pixel == qrcode::Color::Dark {
                    output.push('█');
                } else {
                    output.push(' ');
                }
            }
            output.push(' ');
            output.push('║');
            output.push('\n');
        }
        
        // Bottom border
        output.push_str("╚");
        for _ in 0..(width + 2) {
            output.push('═');
        }
        output.push_str("╝\n");
        
        // Add info line
        output.push_str(&format!("📱 Scan with your phone camera\n"));
        output.push_str(&format!("📊 {}x{} QR code ({} bytes encoded)\n", width, width, data.len()));
        output.push_str(&format!("🔢 {} chars encoded\n", data.len() * 2));
        
        Ok(output)
    }
}
