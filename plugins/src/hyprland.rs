use anyhow::Result;
use serde_json::json;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn detect() -> Result<String> {
    let mut result = json!({
        "hyprland_installed": false,
        "hyprland_running": false,
        "version": null,
        "config_exists": false,
        "config_path": null,
        "wayland_session": false,
    });

    // Check if Hyprland is installed
    if let Ok(output) = Command::new("which").arg("Hyprland").output() {
        if output.status.success() {
            result["hyprland_installed"] = json!(true);

            // Get Hyprland version
            if let Ok(version_output) = Command::new("Hyprland").arg("--version").output() {
                if let Ok(version_str) = String::from_utf8(version_output.stdout) {
                    result["version"] = json!(version_str.trim());
                }
            }
        }
    }

    // Check if Hyprland is currently running
    if let Ok(session) = std::env::var("XDG_SESSION_TYPE") {
        if session == "wayland" {
            result["wayland_session"] = json!(true);
        }
    }

    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        if desktop.to_lowercase().contains("hyprland") {
            result["hyprland_running"] = json!(true);
        }
    }

    // Check for hyprland.conf in standard locations
    let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/root"));
    let config_paths = vec![
        format!("{}/.config/hypr/hyprland.conf", home),
        "/etc/hypr/hyprland.conf".to_string(),
    ];

    for path in config_paths {
        if Path::new(&path).exists() {
            result["config_exists"] = json!(true);
            result["config_path"] = json!(path);
            break;
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}

pub fn status() -> Result<String> {
    let mut result = json!({
        "connected": false,
        "monitors": [],
        "workspaces": [],
        "active_window": null,
        "hyprctl_available": false,
    });

    // Check if hyprctl is available
    if let Ok(output) = Command::new("which").arg("hyprctl").output() {
        if !output.status.success() {
            result["error"] = json!("hyprctl not found - is Hyprland installed?");
            return Ok(serde_json::to_string_pretty(&result)?);
        }
        result["hyprctl_available"] = json!(true);
    }

    // Check if we can connect to Hyprland
    if let Ok(output) = Command::new("hyprctl").arg("version").output() {
        if output.status.success() {
            result["connected"] = json!(true);

            // Get monitors
            if let Ok(monitors_output) = Command::new("hyprctl").args(&["monitors", "-j"]).output()
            {
                if let Ok(monitors_str) = String::from_utf8(monitors_output.stdout) {
                    if let Ok(monitors) = serde_json::from_str::<serde_json::Value>(&monitors_str) {
                        result["monitors"] = monitors;
                    }
                }
            }

            // Get workspaces
            if let Ok(workspaces_output) =
                Command::new("hyprctl").args(&["workspaces", "-j"]).output()
            {
                if let Ok(workspaces_str) = String::from_utf8(workspaces_output.stdout) {
                    if let Ok(workspaces) =
                        serde_json::from_str::<serde_json::Value>(&workspaces_str)
                    {
                        result["workspaces"] = workspaces;
                    }
                }
            }

            // Get active window
            if let Ok(window_output) = Command::new("hyprctl")
                .args(&["activewindow", "-j"])
                .output()
            {
                if let Ok(window_str) = String::from_utf8(window_output.stdout) {
                    if let Ok(window) = serde_json::from_str::<serde_json::Value>(&window_str) {
                        result["active_window"] = window;
                    }
                }
            }
        } else {
            result["error"] = json!("Cannot connect to Hyprland - is it running?");
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}

pub fn validate(path: &str) -> Result<String> {
    let mut result = json!({
        "valid": false,
        "errors": [],
        "warnings": [],
        "config_path": path,
    });

    let config_path = Path::new(path);

    // Check if path exists
    if !config_path.exists() {
        result["errors"] = json!(["Configuration file does not exist"]);
        return Ok(serde_json::to_string_pretty(&result)?);
    }

    // Read the config file
    let config_content = match fs::read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => {
            result["errors"] = json!([format!("Failed to read config: {}", e)]);
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    };

    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    // Basic syntax validation
    let lines: Vec<&str> = config_content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }

        // Check for common syntax errors
        if trimmed.contains('=') {
            let parts: Vec<&str> = trimmed.split('=').collect();
            if parts.len() > 2 {
                errors.push(format!("Line {}: Multiple '=' signs", i + 1));
            }
        }

        // Check for unclosed braces
        let open_braces = trimmed.matches('{').count();
        let close_braces = trimmed.matches('}').count();
        if open_braces != close_braces {
            warnings.push(format!(
                "Line {}: Unbalanced braces (might be multiline)",
                i + 1
            ));
        }
    }

    // Check for required sections
    if !config_content.contains("monitor=") {
        warnings.push("No monitor configuration found".to_string());
    }

    if !config_content.contains("input {") {
        warnings.push("No input configuration found".to_string());
    }

    // Check for common executables referenced in config
    let common_apps = vec!["kitty", "waybar", "wofi", "dunst"];
    for app in common_apps {
        if config_content.contains(app) {
            // Check if app is installed
            if let Ok(output) = Command::new("which").arg(app).output() {
                if !output.status.success() {
                    warnings.push(format!(
                        "Application '{}' referenced but not found in PATH",
                        app
                    ));
                }
            }
        }
    }

    result["errors"] = json!(errors);
    result["warnings"] = json!(warnings);
    result["valid"] = json!(errors.is_empty());

    Ok(serde_json::to_string_pretty(&result)?)
}

pub fn reload() -> Result<String> {
    let mut result = json!({
        "success": false,
        "message": null,
    });

    // Send reload signal to Hyprland via hyprctl
    if let Ok(output) = Command::new("hyprctl").arg("reload").output() {
        if output.status.success() {
            result["success"] = json!(true);
            result["message"] = json!("Hyprland configuration reloaded successfully");
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            result["message"] = json!(format!("Failed to reload: {}", error));
        }
    } else {
        result["message"] = json!("hyprctl not found or not executable");
    }

    Ok(serde_json::to_string_pretty(&result)?)
}
