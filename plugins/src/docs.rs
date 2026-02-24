use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};

/// The four documentation files every syntek-infra project carries in `.claude/`.
const DOC_FILES: &[(&str, &str)] = &[
    ("CODING-PRINCIPLES", "CODING-PRINCIPLES.md"),
    ("TESTING", "TESTING.md"),
    ("SECURITY", "SECURITY.md"),
    ("DEVELOPMENT", "DEVELOPMENT.md"),
];

#[derive(Serialize)]
struct DocEntry {
    name: String,
    filename: String,
    path: Option<String>,
    found: bool,
}

#[derive(Serialize)]
struct DocsListResult {
    docs_dir: Option<String>,
    all_found: bool,
    files: Vec<DocEntry>,
    hint: Option<String>,
}

#[derive(Serialize)]
struct DocShowResult {
    found: bool,
    name: String,
    path: Option<String>,
    content: Option<String>,
    error: Option<String>,
}

/// Walk up from `start` until a `.claude/` directory is found, or the filesystem
/// root is reached. Returns the path to `.claude/` if found.
fn find_claude_dir(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        let candidate = current.join(".claude");
        if candidate.is_dir() {
            return Some(candidate);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Resolve the documentation directory.
///
/// Resolution order:
/// 1. `SYNTEK_DOCS_PATH` environment variable (explicit override for CI/automation)
/// 2. Walk up from the current working directory looking for `.claude/`
fn resolve_docs_dir() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SYNTEK_DOCS_PATH") {
        let p = PathBuf::from(&path);
        if p.is_dir() {
            return Some(p);
        }
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    find_claude_dir(&cwd)
}

/// Build a compact docs summary suitable for embedding inside other command outputs.
/// Agents read this to know where to find the coding standards, testing guide, etc.
#[allow(dead_code)]
pub fn docs_summary() -> serde_json::Value {
    let docs_dir = resolve_docs_dir();

    let Some(dir) = docs_dir else {
        return serde_json::json!({
            "found": false,
            "hint": "No .claude/ directory found. Run /syntek-infra:init to create project documentation."
        });
    };

    let mut files = serde_json::Map::new();
    let mut all_found = true;

    for (name, filename) in DOC_FILES {
        let path = dir.join(filename);
        if path.exists() {
            files.insert(name.to_string(), serde_json::json!(path.display().to_string()));
        } else {
            files.insert(name.to_string(), serde_json::Value::Null);
            all_found = false;
        }
    }

    let hint = if all_found {
        None
    } else {
        Some("Some documentation files are missing. Re-run /syntek-infra:init to regenerate.")
    };

    serde_json::json!({
        "found": all_found,
        "docs_dir": dir.display().to_string(),
        "files": files,
        "hint": hint,
    })
}

/// List all project documentation files, showing whether each is present.
pub fn list() -> Result<String> {
    let docs_dir = resolve_docs_dir();
    let docs_dir_str = docs_dir.as_deref().map(|d| d.display().to_string());

    let files: Vec<DocEntry> = DOC_FILES
        .iter()
        .map(|(name, filename)| {
            let path = docs_dir.as_ref().map(|d| d.join(filename));
            let found = path.as_ref().map(|p| p.exists()).unwrap_or(false);
            let path_str = path
                .filter(|p| p.exists())
                .map(|p| p.display().to_string());
            DocEntry {
                name: name.to_string(),
                filename: filename.to_string(),
                path: path_str,
                found,
            }
        })
        .collect();

    let hint = if docs_dir_str.is_none() {
        Some("No .claude/ directory found. Run /syntek-infra:init to initialise project documentation.".to_string())
    } else if files.iter().any(|f| !f.found) {
        Some("Some files are missing. Re-run /syntek-infra:init to regenerate.".to_string())
    } else {
        None
    };

    let result = DocsListResult {
        docs_dir: docs_dir_str,
        all_found: files.iter().all(|f| f.found),
        files,
        hint,
    };

    Ok(serde_json::to_string_pretty(&result)?)
}

/// Show the full content of a named documentation file.
///
/// `name` accepts: `CODING-PRINCIPLES`, `TESTING`, `SECURITY`, `DEVELOPMENT`
/// (case-insensitive, hyphens and underscores are interchangeable).
pub fn show(name: &str) -> Result<String> {
    let normalised = name.to_uppercase().replace(".MD", "").replace('_', "-");

    let filename = DOC_FILES
        .iter()
        .find(|(n, _)| n.replace('-', "") == normalised.replace('-', ""))
        .map(|(_, f)| *f);

    let filename = match filename {
        Some(f) => f,
        None => {
            let result = DocShowResult {
                found: false,
                name: name.to_string(),
                path: None,
                content: None,
                error: Some(format!(
                    "Unknown document '{}'. Available: CODING-PRINCIPLES, TESTING, SECURITY, DEVELOPMENT",
                    name
                )),
            };
            return Ok(serde_json::to_string_pretty(&result)?);
        }
    };

    let docs_dir = resolve_docs_dir();
    let mut result = DocShowResult {
        found: false,
        name: name.to_string(),
        path: None,
        content: None,
        error: None,
    };

    match docs_dir {
        None => {
            result.error = Some(
                "No .claude/ directory found. Run /syntek-infra:init to initialise project documentation.".to_string(),
            );
        }
        Some(dir) => {
            let doc_path = dir.join(filename);
            if doc_path.exists() {
                match std::fs::read_to_string(&doc_path) {
                    Ok(content) => {
                        result.found = true;
                        result.path = Some(doc_path.display().to_string());
                        result.content = Some(content);
                    }
                    Err(e) => {
                        result.error =
                            Some(format!("Failed to read {}: {}", doc_path.display(), e));
                    }
                }
            } else {
                result.error = Some(format!(
                    "{} not found at {}. Re-run /syntek-infra:init to regenerate.",
                    filename,
                    doc_path.display()
                ));
            }
        }
    }

    Ok(serde_json::to_string_pretty(&result)?)
}
