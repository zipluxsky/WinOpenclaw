//! Install flow: detect Node, install Node via winget, npm install openclaw, PATH.

use std::env;
use std::process::Command;
use std::str::FromStr;

use crate::run;

/// Result of checking Node.js availability.
#[derive(serde::Serialize)]
pub struct NodeCheckResult {
    pub found: bool,
    pub version: Option<String>,
    pub major: Option<u32>,
    pub ok: bool,
    pub message: String,
}

/// Check if Node.js 22+ is available.
pub fn check_node() -> NodeCheckResult {
    let output = match Command::new("node").arg("-v").output() {
        Ok(o) => o,
        Err(e) => {
            return NodeCheckResult {
                found: false,
                version: None,
                major: None,
                ok: false,
                message: format!("Node not found: {}", e),
            };
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = stdout.trim().trim_start_matches('v').to_string();
    let major = parse_node_major(version.as_str());
    let ok = major.map(|m| m >= 22).unwrap_or(false);
    let message = if ok {
        format!("Node {} (OK)", version)
    } else {
        format!(
            "Node {} found but need 22+ (major: {:?})",
            version, major
        )
    };
    NodeCheckResult {
        found: true,
        version: Some(version),
        major,
        ok,
        message,
    }
}

fn parse_node_major(s: &str) -> Option<u32> {
    let mut it = s.split('.');
    let first = it.next()?;
    u32::from_str(first).ok()
}

/// Run winget to install Node.js LTS (OpenJS.NodeJS.LTS).
/// Returns (success, message).
pub fn install_node_via_winget() -> (bool, String) {
    let status = Command::new("winget")
        .args([
            "install",
            "OpenJS.NodeJS.LTS",
            "--accept-package-agreements",
            "--accept-source-agreements",
        ])
        .status();
    match status {
        Ok(s) if s.success() => (true, "Node.js installed. Restart the app and run Check again.".to_string()),
        Ok(s) => (false, format!("winget exited with code: {:?}", s.code())),
        Err(e) => (false, format!("winget failed: {}", e)),
    }
}

/// Run npm install -g openclaw@latest.
/// Returns (success, message). Uses cmd /c on Windows so npm.cmd works.
pub fn install_openclaw_npm() -> (bool, String) {
    let output = match run::output("npm", &["install", "-g", "openclaw@latest"]) {
        Ok(o) => o,
        Err(e) => return (false, format!("npm not found or failed: {}", e)),
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if output.status.success() {
        (true, format!("Installed. {}", stdout.trim()))
    } else {
        (false, format!("npm error: {} {}", stderr.trim(), stdout.trim()))
    }
}

/// Check if openclaw CLI is available and return its version.
#[derive(serde::Serialize)]
pub struct OpenClawCheckResult {
    pub found: bool,
    pub version: Option<String>,
    pub message: String,
}

pub fn check_openclaw() -> OpenClawCheckResult {
    let output = match run::output("openclaw", &["--version"]) {
        Ok(o) => o,
        Err(_) => {
            return OpenClawCheckResult {
                found: false,
                version: None,
                message: "openclaw not found in PATH".to_string(),
            };
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let version = stdout.trim().to_string();
    let found = output.status.success();
    OpenClawCheckResult {
        found,
        version: if found && !version.is_empty() {
            Some(version.clone())
        } else {
            None
        },
        message: if found {
            format!("OpenClaw {}", version)
        } else {
            "openclaw failed".to_string()
        },
    }
}

/// Get npm global prefix (path to global node_modules parent). Uses cmd /c on Windows.
pub fn get_npm_prefix() -> Result<String, String> {
    let output = run::output("npm", &["config", "get", "prefix"]).map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let prefix = stdout.trim().trim_matches('"').to_string();
    if prefix.is_empty() {
        return Err("npm prefix empty".to_string());
    }
    Ok(prefix)
}

/// Ensure npm global bin is on user PATH (Windows: User env var Path).
/// Best-effort: if already in PATH, report OK; otherwise suggest manual add (setx has length limits).
#[cfg(windows)]
pub fn ensure_npm_on_path() -> (bool, String) {
    let prefix = match get_npm_prefix() {
        Ok(p) => p,
        Err(e) => return (false, e),
    };
    let bin = format!("{}\\bin", prefix.replace('/', "\\"));
    let path_var = env::var("Path").or_else(|_| env::var("PATH"));
    if let Ok(current) = path_var {
        if current.to_lowercase().contains(bin.to_lowercase().as_str()) {
            return (true, "PATH already contains npm bin".to_string());
        }
    }
    (
        true,
        format!(
            "Add to user PATH manually if needed: {} (Then restart this app.)",
            bin
        ),
    )
}

#[cfg(not(windows))]
pub fn ensure_npm_on_path() -> (bool, String) {
    (true, "N/A on this platform".to_string())
}
