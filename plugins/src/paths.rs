use anyhow::{Context, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Walk up from `start` looking for `.claude-plugin/plugin.json`.
/// Returns the directory that contains `.claude-plugin/` if found.
fn find_plugin_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".claude-plugin").join("plugin.json").is_file() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Resolve the plugin root directory.
///
/// Resolution order:
/// 1. `SYNTEK_PLUGIN_DIR` environment variable (explicit override for CI or
///    when the binary has been copied out of the plugin tree)
/// 2. Walk up from the running executable's location — works when the binary
///    lives at `<plugin_root>/plugins/target/release/syntek-infra-tool`
pub fn resolve_plugin_root() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SYNTEK_PLUGIN_DIR") {
        let p = PathBuf::from(&path);
        if p.is_dir() {
            return Some(p);
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            return find_plugin_root(parent);
        }
    }

    None
}

#[derive(Serialize)]
struct PluginPaths {
    plugin_root: Option<String>,
    templates: Option<String>,
    examples: Option<String>,
    agents: Option<String>,
    commands: Option<String>,
    hint: Option<String>,
}

fn opt_dir(root: &Path, name: &str) -> Option<String> {
    let p = root.join(name);
    if p.is_dir() {
        Some(p.display().to_string())
    } else {
        None
    }
}

/// Return a JSON object containing every important plugin directory.
/// Agents call this once at startup to resolve all file references dynamically.
pub fn all() -> Result<String> {
    match resolve_plugin_root() {
        None => Ok(serde_json::to_string_pretty(&PluginPaths {
            plugin_root: None,
            templates: None,
            examples: None,
            agents: None,
            commands: None,
            hint: Some(
                "Plugin root not found. Set SYNTEK_PLUGIN_DIR to the plugin directory.".to_string(),
            ),
        })?),
        Some(root) => Ok(serde_json::to_string_pretty(&PluginPaths {
            templates: opt_dir(&root, "templates"),
            examples: opt_dir(&root, "examples"),
            agents: opt_dir(&root, "agents"),
            commands: opt_dir(&root, "commands"),
            plugin_root: Some(root.display().to_string()),
            hint: None,
        })?),
    }
}

/// Return the absolute path for a single named directory within the plugin.
///
/// Accepted names: `plugin-root`, `root`, `templates`, `examples`,
/// `agents`, `commands`.
pub fn get(name: &str) -> Result<String> {
    let root = resolve_plugin_root()
        .context("Plugin root not found. Set SYNTEK_PLUGIN_DIR to the plugin directory.")?;

    let path = match name {
        "plugin-root" | "root" => root.clone(),
        "templates" => root.join("templates"),
        "examples" => root.join("examples"),
        "agents" => root.join("agents"),
        "commands" => root.join("commands"),
        other => anyhow::bail!(
            "Unknown path '{}'. Available: plugin-root, templates, examples, agents, commands",
            other
        ),
    };

    Ok(serde_json::json!({
        "name": name,
        "path": path.display().to_string(),
        "exists": path.exists(),
    })
    .to_string())
}
