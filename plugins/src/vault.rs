use anyhow::Result;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct VaultStatus {
    connected: bool,
    authenticated: bool,
    vault_addr: Option<String>,
    version: Option<String>,
    sealed: Option<bool>,
    error: Option<String>,
}

#[derive(Serialize)]
struct SecretReadResult {
    success: bool,
    path: String,
    data: Option<serde_json::Value>,
    error: Option<String>,
}

#[derive(Serialize)]
struct SecretWriteResult {
    success: bool,
    path: String,
    error: Option<String>,
}

/// Check Vault connectivity and authentication status
pub fn status() -> Result<String> {
    let mut result = VaultStatus {
        connected: false,
        authenticated: false,
        vault_addr: env::var("VAULT_ADDR").ok(),
        version: None,
        sealed: None,
        error: None,
    };

    // Check if VAULT_ADDR is set
    if result.vault_addr.is_none() {
        result.error = Some("VAULT_ADDR environment variable not set".to_string());
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Check if vault CLI is available
    let output = std::process::Command::new("vault").arg("version").output();

    match output {
        Ok(o) if o.status.success() => {
            result.version = Some(String::from_utf8_lossy(&o.stdout).trim().to_string());
        }
        Ok(_) | Err(_) => {
            result.error = Some("Vault CLI not available".to_string());
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    }

    // Check Vault status
    let output = std::process::Command::new("vault")
        .arg("status")
        .args(["-format=json"])
        .output();

    match output {
        Ok(o) => {
            if let Ok(status) = serde_json::from_slice::<serde_json::Value>(&o.stdout) {
                result.connected = true;
                result.sealed = status.get("sealed").and_then(|s| s.as_bool());

                if result.sealed == Some(true) {
                    result.error = Some("Vault is sealed".to_string());
                }
            } else if o.status.code() == Some(2) {
                // Exit code 2 means sealed
                result.connected = true;
                result.sealed = Some(true);
                result.error = Some("Vault is sealed".to_string());
            }
        }
        Err(e) => {
            result.error = Some(format!("Failed to connect to Vault: {}", e));
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    }

    // Check authentication
    if result.connected && result.sealed != Some(true) {
        let output = std::process::Command::new("vault")
            .args(["token", "lookup", "-format=json"])
            .output();

        match output {
            Ok(o) if o.status.success() => {
                result.authenticated = true;
            }
            _ => {
                result.error = Some("Not authenticated to Vault".to_string());
            }
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}

/// Read a secret from Vault
pub fn read(path: &str) -> Result<String> {
    let mut result = SecretReadResult {
        success: false,
        path: path.to_string(),
        data: None,
        error: None,
    };

    // Check VAULT_ADDR
    if env::var("VAULT_ADDR").is_err() {
        result.error = Some("VAULT_ADDR environment variable not set".to_string());
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Read secret using KV v2
    let output = std::process::Command::new("vault")
        .args(["kv", "get", "-format=json", path])
        .output();

    match output {
        Ok(o) if o.status.success() => {
            if let Ok(response) = serde_json::from_slice::<serde_json::Value>(&o.stdout) {
                result.success = true;
                result.data = response.get("data").and_then(|d| d.get("data")).cloned();
            }
        }
        Ok(o) => {
            result.error = Some(String::from_utf8_lossy(&o.stderr).trim().to_string());
        }
        Err(e) => {
            result.error = Some(format!("Failed to read secret: {}", e));
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}

/// Write a secret to Vault
pub fn write(path: &str, data: &str) -> Result<String> {
    let mut result = SecretWriteResult {
        success: false,
        path: path.to_string(),
        error: None,
    };

    // Check VAULT_ADDR
    if env::var("VAULT_ADDR").is_err() {
        result.error = Some("VAULT_ADDR environment variable not set".to_string());
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Parse the data to validate JSON
    let parsed: serde_json::Value = match serde_json::from_str(data) {
        Ok(v) => v,
        Err(e) => {
            result.error = Some(format!("Invalid JSON data: {}", e));
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    };

    // Build the vault kv put command with key=value pairs
    let mut cmd = std::process::Command::new("vault");
    cmd.args(["kv", "put", path]);

    if let Some(obj) = parsed.as_object() {
        for (key, value) in obj {
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            cmd.arg(format!("{}={}", key, value_str));
        }
    } else {
        result.error = Some("Data must be a JSON object".to_string());
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    let output = cmd.output();

    match output {
        Ok(o) if o.status.success() => {
            result.success = true;
        }
        Ok(o) => {
            result.error = Some(String::from_utf8_lossy(&o.stderr).trim().to_string());
        }
        Err(e) => {
            result.error = Some(format!("Failed to write secret: {}", e));
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}
