use anyhow::Result;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct NixosStatus {
    is_nixos: bool,
    version: Option<String>,
    current_generation: Option<u32>,
    current_configuration: Option<String>,
    kernel_version: Option<String>,
    system_type: Option<String>,
}

#[derive(Serialize)]
struct Generation {
    number: u32,
    date: String,
    nixos_version: String,
    kernel_version: String,
    current: bool,
}

#[derive(Serialize)]
struct GenerationList {
    generations: Vec<Generation>,
    current: u32,
}

#[derive(Serialize)]
struct ValidationResult {
    valid: bool,
    path: String,
    errors: Vec<String>,
    warnings: Vec<String>,
}

/// Get current NixOS system status
pub fn status() -> Result<String> {
    let mut result = NixosStatus {
        is_nixos: false,
        version: None,
        current_generation: None,
        current_configuration: None,
        kernel_version: None,
        system_type: None,
    };

    // Check if running NixOS
    if std::path::Path::new("/etc/NIXOS").exists() {
        result.is_nixos = true;
    } else {
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Get NixOS version
    if let Ok(version) = std::fs::read_to_string("/run/current-system/nixos-version") {
        result.version = Some(version.trim().to_string());
    }

    // Get current generation
    if let Ok(output) = Command::new("nixos-rebuild")
        .args(["list-generations", "--json"])
        .output()
    {
        if output.status.success() {
            if let Ok(gens) = serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout) {
                for gen in gens.iter().rev() {
                    if gen
                        .get("current")
                        .and_then(|c| c.as_bool())
                        .unwrap_or(false)
                    {
                        result.current_generation = gen
                            .get("generation")
                            .and_then(|g| g.as_u64())
                            .map(|g| g as u32);
                        break;
                    }
                }
            }
        }
    }

    // Fallback: parse generation from symlink
    if result.current_generation.is_none() {
        if let Ok(link) = std::fs::read_link("/run/current-system") {
            let link_str = link.to_string_lossy();
            // Format: /nix/store/xxx-nixos-system-hostname-version
            if let Some(gen_str) = link_str.split('-').last() {
                if let Ok(gen) = gen_str.parse::<u32>() {
                    result.current_generation = Some(gen);
                }
            }
        }
    }

    // Get kernel version
    if let Ok(output) = Command::new("uname").arg("-r").output() {
        if output.status.success() {
            result.kernel_version =
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    // Get system type
    if let Ok(output) = Command::new("uname").arg("-m").output() {
        if output.status.success() {
            result.system_type = Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
    }

    // Check for current configuration path
    if std::path::Path::new("/etc/nixos/flake.nix").exists() {
        result.current_configuration = Some("/etc/nixos".to_string());
    } else if std::path::Path::new("/etc/nixos/configuration.nix").exists() {
        result.current_configuration = Some("/etc/nixos".to_string());
    }

    Ok(serde_json::to_string_pretty(&result)?)
}

/// List NixOS generations
pub fn generations() -> Result<String> {
    let mut result = GenerationList {
        generations: Vec::new(),
        current: 0,
    };

    // Try JSON output first (newer NixOS)
    if let Ok(output) = Command::new("nixos-rebuild")
        .args(["list-generations", "--json"])
        .output()
    {
        if output.status.success() {
            if let Ok(gens) = serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout) {
                for gen in gens {
                    let number = gen.get("generation").and_then(|g| g.as_u64()).unwrap_or(0) as u32;

                    let current = gen
                        .get("current")
                        .and_then(|c| c.as_bool())
                        .unwrap_or(false);

                    if current {
                        result.current = number;
                    }

                    result.generations.push(Generation {
                        number,
                        date: gen
                            .get("date")
                            .and_then(|d| d.as_str())
                            .unwrap_or("")
                            .to_string(),
                        nixos_version: gen
                            .get("nixosVersion")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        kernel_version: gen
                            .get("kernelVersion")
                            .and_then(|k| k.as_str())
                            .unwrap_or("")
                            .to_string(),
                        current,
                    });
                }
                return Ok(serde_json::to_string_pretty(&result)?);
            }
        }
    }

    // Fallback: Parse text output
    if let Ok(output) = Command::new("nixos-rebuild")
        .arg("list-generations")
        .output()
    {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if line.trim().is_empty() {
                    continue;
                }

                // Parse lines like: "42   2024-01-15 12:30:00   NixOS 24.05..."
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    if let Ok(number) = parts[0].parse::<u32>() {
                        let current = line.contains("current");
                        if current {
                            result.current = number;
                        }

                        result.generations.push(Generation {
                            number,
                            date: parts.get(1).unwrap_or(&"").to_string(),
                            nixos_version: parts.get(3..).map(|p| p.join(" ")).unwrap_or_default(),
                            kernel_version: String::new(),
                            current,
                        });
                    }
                }
            }
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}

/// Validate a NixOS configuration
pub fn validate(path: &str) -> Result<String> {
    let mut result = ValidationResult {
        valid: false,
        path: path.to_string(),
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    // Check for flake.nix or configuration.nix
    let flake_path = std::path::Path::new(path).join("flake.nix");
    let config_path = std::path::Path::new(path).join("configuration.nix");

    if flake_path.exists() {
        // Validate flake
        let output = Command::new("nix")
            .args(["flake", "check", path, "--no-build"])
            .output();

        match output {
            Ok(o) => {
                result.valid = o.status.success();
                if !o.status.success() {
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    for line in stderr.lines() {
                        if line.contains("error:") {
                            result.errors.push(line.to_string());
                        } else if line.contains("warning:") {
                            result.warnings.push(line.to_string());
                        }
                    }
                }
            }
            Err(e) => {
                result
                    .errors
                    .push(format!("Failed to run validation: {}", e));
            }
        }
    } else if config_path.exists() {
        // Validate legacy configuration
        let output = Command::new("nix-instantiate")
            .args(["--parse", config_path.to_str().unwrap()])
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
                    .push(format!("Failed to run validation: {}", e));
            }
        }
    } else {
        result.errors.push(format!(
            "No flake.nix or configuration.nix found at {}",
            path
        ));
    }

    Ok(serde_json::to_string_pretty(&result)?)
}
