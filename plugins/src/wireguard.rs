use anyhow::Result;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct KeyPair {
    private_key: String,
    public_key: String,
}

#[derive(Serialize)]
struct KeygenResult {
    success: bool,
    keys: Option<KeyPair>,
    error: Option<String>,
}

#[derive(Serialize)]
struct QrResult {
    success: bool,
    config_path: String,
    qr_text: Option<String>,
    error: Option<String>,
}

/// Generate a Wireguard key pair
pub fn keygen() -> Result<String> {
    let mut result = KeygenResult {
        success: false,
        keys: None,
        error: None,
    };

    // Check if wg is available
    let wg_check = Command::new("wg").arg("--version").output();
    if wg_check.is_err() || !wg_check.unwrap().status.success() {
        result.error = Some("Wireguard tools (wg) not available".to_string());
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Generate private key
    let private_output = Command::new("wg").arg("genkey").output();

    let private_key = match private_output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Ok(o) => {
            result.error = Some(String::from_utf8_lossy(&o.stderr).trim().to_string());
            return Ok(serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            result.error = Some(format!("Failed to generate private key: {}", e));
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    };

    // Generate public key from private key
    let mut pubkey_cmd = Command::new("wg");
    pubkey_cmd.arg("pubkey");
    pubkey_cmd.stdin(std::process::Stdio::piped());
    pubkey_cmd.stdout(std::process::Stdio::piped());

    let pubkey_child = pubkey_cmd.spawn();

    let public_key = match pubkey_child {
        Ok(mut child) => {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                let _ = stdin.write_all(private_key.as_bytes());
            }

            let output = child.wait_with_output()?;
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                result.error = Some("Failed to generate public key".to_string());
                return Ok(serde_json::to_string_pretty(&result)?);
            }
        }
        Err(e) => {
            result.error = Some(format!("Failed to generate public key: {}", e));
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    };

    result.success = true;
    result.keys = Some(KeyPair {
        private_key,
        public_key,
    });

    Ok(serde_json::to_string_pretty(&result)?)
}

/// Generate a QR code for mobile Wireguard configuration
pub fn qr(config_path: &str) -> Result<String> {
    let mut result = QrResult {
        success: false,
        config_path: config_path.to_string(),
        qr_text: None,
        error: None,
    };

    // Check if config file exists
    if !std::path::Path::new(config_path).exists() {
        result.error = Some(format!("Configuration file not found: {}", config_path));
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Read config file
    let config_content = match std::fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(e) => {
            result.error = Some(format!("Failed to read config file: {}", e));
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    };

    // Try to use qrencode if available
    let qr_output = Command::new("qrencode")
        .args(["-t", "UTF8", "-r", config_path])
        .output();

    match qr_output {
        Ok(o) if o.status.success() => {
            result.success = true;
            result.qr_text = Some(String::from_utf8_lossy(&o.stdout).to_string());
        }
        Ok(_) | Err(_) => {
            // qrencode not available, provide alternative instructions
            result.success = true;
            result.qr_text = Some(format!(
                "qrencode not installed. To generate QR code:\n\n\
                 1. Install qrencode: nix-env -iA nixpkgs.qrencode\n\
                 2. Run: qrencode -t UTF8 -r {}\n\n\
                 Or use an online tool with this config:\n\n{}",
                config_path, config_content
            ));
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}
