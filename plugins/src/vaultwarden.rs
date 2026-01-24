use anyhow::Result;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct VaultwardenStatus {
    connected: bool,
    server_url: Option<String>,
    authenticated: bool,
    error: Option<String>,
}

#[derive(Serialize)]
struct SyncResult {
    success: bool,
    items_synced: u32,
    error: Option<String>,
}

/// Check Vaultwarden connectivity
pub fn status() -> Result<String> {
    let mut result = VaultwardenStatus {
        connected: false,
        server_url: env::var("VAULTWARDEN_URL")
            .ok()
            .or_else(|| env::var("BW_URL").ok()),
        authenticated: false,
        error: None,
    };

    // Check if server URL is set
    if result.server_url.is_none() {
        result.error = Some("VAULTWARDEN_URL or BW_URL environment variable not set".to_string());
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Check if bw CLI is available
    let output = std::process::Command::new("bw").arg("--version").output();

    match output {
        Ok(o) if o.status.success() => {
            // CLI is available
        }
        Ok(_) | Err(_) => {
            result.error = Some("Bitwarden CLI (bw) not available".to_string());
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    }

    // Check login status
    let output = std::process::Command::new("bw").arg("status").output();

    match output {
        Ok(o) if o.status.success() => {
            if let Ok(status) = serde_json::from_slice::<serde_json::Value>(&o.stdout) {
                let status_str = status
                    .get("status")
                    .and_then(|s| s.as_str())
                    .unwrap_or("unknown");

                result.connected = true;
                result.authenticated = status_str == "unlocked";

                if status_str == "locked" {
                    result.error = Some("Vault is locked. Run: bw unlock".to_string());
                } else if status_str == "unauthenticated" {
                    result.error = Some("Not logged in. Run: bw login".to_string());
                }
            }
        }
        Ok(o) => {
            result.error = Some(String::from_utf8_lossy(&o.stderr).trim().to_string());
        }
        Err(e) => {
            result.error = Some(format!("Failed to check status: {}", e));
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}

/// Sync secrets from Vault to Vaultwarden
/// Note: This is a placeholder - actual implementation would need
/// proper API integration with both Vault and Vaultwarden
pub fn sync() -> Result<String> {
    // In a real implementation, this would:
    // 1. Connect to Vault and read secrets from a specific path
    // 2. Connect to Vaultwarden via API
    // 3. Create/update items in Vaultwarden
    // 4. Return count of synced items

    // For now, we just check if both services are available
    let vault_status = super::vault::status()?;
    let vw_status = status()?;

    let vault_ok = serde_json::from_str::<serde_json::Value>(&vault_status)
        .ok()
        .and_then(|v| v.get("authenticated").and_then(|a| a.as_bool()))
        .unwrap_or(false);

    let vw_ok = serde_json::from_str::<serde_json::Value>(&vw_status)
        .ok()
        .and_then(|v| v.get("authenticated").and_then(|a| a.as_bool()))
        .unwrap_or(false);

    let result = if vault_ok && vw_ok {
        SyncResult {
            success: false,
            items_synced: 0,
            error: Some(
                "Both services connected. Sync logic not yet implemented. \
                 Use vault-manager agent for manual sync guidance."
                    .to_string(),
            ),
        }
    } else {
        SyncResult {
            success: false,
            items_synced: 0,
            error: Some(format!(
                "Services not ready. Vault authenticated: {}, Vaultwarden authenticated: {}",
                vault_ok, vw_ok
            )),
        }
    };

    Ok(serde_json::to_string_pretty(&result)?)
}
