//! Tauri command handlers.

use crate::config;
use crate::install;
use crate::run;
use crate::security;
use tauri::State;

type AppState<'a> = State<'a, crate::AppState>;

/// Returns the resolved OpenClaw config file path.
#[tauri::command]
pub fn get_config_path() -> Result<String, String> {
    config::resolve_config_path()
}

/// Get a single config value via `openclaw config get <path>`. Uses cmd /c on Windows.
#[tauri::command]
pub fn get_config_value(path: String) -> Result<String, String> {
    let path = path.trim();
    if path.is_empty() {
        return Err("path is empty".to_string());
    }
    let output = run::command("openclaw", &["config", "get", path])
        .output()
        .map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() {
        Ok(stdout.trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("{}", stderr.trim().min(stdout.trim())))
    }
}

/// Set a single config value via `openclaw config set <path> <value>`.
/// Value is passed as a single argument (no shell) to avoid injection. Uses cmd /c on Windows.
#[tauri::command]
pub fn set_config_value(path: String, value: String) -> Result<(), String> {
    let path = path.trim();
    if path.is_empty() {
        return Err("path is empty".to_string());
    }
    let output = run::command("openclaw", &["config", "set", path, value.as_str()])
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!("{} {}", stderr.trim(), stdout.trim()))
    }
}

/// Check Node.js (version 22+).
#[tauri::command]
pub fn check_node() -> install::NodeCheckResult {
    install::check_node()
}

/// Check if openclaw CLI is available and return version/message.
#[tauri::command]
pub fn check_openclaw() -> install::OpenClawCheckResult {
    install::check_openclaw()
}

/// Install Node.js via winget.
#[tauri::command]
pub fn install_node() -> Result<String, String> {
    let (ok, msg) = install::install_node_via_winget();
    if ok {
        Ok(msg)
    } else {
        Err(msg)
    }
}

/// Install OpenClaw via npm install -g openclaw@latest.
#[tauri::command]
pub fn install_openclaw() -> Result<String, String> {
    let (ok, msg) = install::install_openclaw_npm();
    if ok {
        Ok(msg)
    } else {
        Err(msg)
    }
}

/// Ensure npm global bin is on user PATH (best-effort).
#[tauri::command]
pub fn ensure_path() -> Result<String, String> {
    let (ok, msg) = install::ensure_npm_on_path();
    if ok {
        Ok(msg)
    } else {
        Err(msg)
    }
}

/// Run security audit (optionally deep), return score and summary.
#[tauri::command]
pub fn run_security_audit(deep: bool, state: AppState<'_>) -> Result<security::AuditResult, String> {
    let result = security::run_audit(deep)?;
    if let Ok(mut last) = state.last_audit.lock() {
        *last = Some(result.clone());
    }
    Ok(result)
}

/// Run security audit --fix.
#[tauri::command]
pub fn run_security_fix() -> Result<String, String> {
    let (ok, msg) = security::run_fix();
    if ok {
        Ok(msg)
    } else {
        Err(msg)
    }
}

/// Return last audit result if any (so UI can show details without re-running).
#[tauri::command]
pub fn get_last_audit(state: AppState<'_>) -> Option<security::AuditResult> {
    state.last_audit.lock().ok().and_then(|g| g.clone())
}
