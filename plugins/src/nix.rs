use anyhow::Result;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct NixDetectResult {
    installed: bool,
    version: Option<String>,
    flakes_enabled: bool,
    nix_command_enabled: bool,
    multi_user: bool,
    channels: Vec<String>,
}

#[derive(Serialize)]
struct ChannelInfo {
    name: String,
    url: String,
}

#[derive(Serialize)]
struct FlakeCheckResult {
    valid: bool,
    path: String,
    inputs: Vec<String>,
    outputs: Vec<String>,
    errors: Vec<String>,
}

/// Detect Nix installation and configuration
pub fn detect() -> Result<String> {
    let mut result = NixDetectResult {
        installed: false,
        version: None,
        flakes_enabled: false,
        nix_command_enabled: false,
        multi_user: false,
        channels: Vec::new(),
    };

    // Check if nix is installed
    if let Ok(output) = Command::new("nix").arg("--version").output() {
        if output.status.success() {
            result.installed = true;
            result.version = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    if !result.installed {
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Check for flakes support
    if let Ok(output) = Command::new("nix").args(["flake", "--help"]).output() {
        result.flakes_enabled = output.status.success();
    }

    // Check for nix-command experimental feature
    if let Ok(output) = Command::new("nix").args(["eval", "--help"]).output() {
        result.nix_command_enabled = output.status.success();
    }

    // Check if multi-user installation
    result.multi_user = std::path::Path::new("/nix/var/nix/daemon-socket/socket").exists();

    // Get channels
    if let Ok(output) = Command::new("nix-channel").arg("--list").output() {
        if output.status.success() {
            result.channels = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.split_whitespace().next().unwrap_or("").to_string())
                .collect();
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}

/// List configured Nix channels
pub fn channels() -> Result<String> {
    let mut channels: Vec<ChannelInfo> = Vec::new();

    if let Ok(output) = Command::new("nix-channel").arg("--list").output() {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    channels.push(ChannelInfo {
                        name: parts[0].to_string(),
                        url: parts[1].to_string(),
                    });
                }
            }
        }
    }

    Ok(serde_json::to_string_pretty(&channels)?)
}

/// Validate a Nix flake
pub fn flake_check(path: &str) -> Result<String> {
    let mut result = FlakeCheckResult {
        valid: false,
        path: path.to_string(),
        inputs: Vec::new(),
        outputs: Vec::new(),
        errors: Vec::new(),
    };

    // Check if flake.nix exists
    let flake_path = std::path::Path::new(path).join("flake.nix");
    if !flake_path.exists() {
        result
            .errors
            .push(format!("flake.nix not found at {}", path));
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Run nix flake check
    let output = Command::new("nix")
        .args(["flake", "check", path, "--no-build"])
        .output();

    match output {
        Ok(o) => {
            result.valid = o.status.success();
            if !o.status.success() {
                result
                    .errors
                    .push(String::from_utf8_lossy(&o.stderr).trim().to_string());
            }
        }
        Err(e) => {
            result
                .errors
                .push(format!("Failed to run nix flake check: {}", e));
        }
    }

    // Get flake metadata if valid
    if result.valid {
        if let Ok(output) = Command::new("nix")
            .args(["flake", "metadata", path, "--json"])
            .output()
        {
            if output.status.success() {
                if let Ok(metadata) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                    if let Some(inputs) = metadata.get("locks").and_then(|l| l.get("nodes")) {
                        if let Some(obj) = inputs.as_object() {
                            result.inputs = obj.keys().filter(|k| *k != "root").cloned().collect();
                        }
                    }
                }
            }
        }

        // Get outputs
        if let Ok(output) = Command::new("nix")
            .args(["flake", "show", path, "--json"])
            .output()
        {
            if output.status.success() {
                if let Ok(show) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                    if let Some(obj) = show.as_object() {
                        result.outputs = obj.keys().cloned().collect();
                    }
                }
            }
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}
