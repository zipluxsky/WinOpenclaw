//! Resolve OpenClaw config path (mirror src/config/paths.ts semantics).
//! Windows-only: uses backslash and USERPROFILE.

use std::env;

fn normalize_windows_path(s: &str) -> String {
    s.trim().replace('/', "\\").trim_end_matches('\\').to_string()
}

/// Resolves the OpenClaw config directory (state dir).
/// Uses OPENCLAW_STATE_DIR or %USERPROFILE%\.openclaw on Windows.
pub fn resolve_state_dir() -> Result<String, String> {
    if let Ok(dir) = env::var("OPENCLAW_STATE_DIR") {
        let trimmed = normalize_windows_path(&dir);
        if !trimmed.is_empty() {
            return Ok(trimmed);
        }
    }
    let userprofile = env::var("USERPROFILE")
        .or_else(|_| env::var("HOME"))
        .map_err(|_| "USERPROFILE and HOME are not set")?;
    Ok(format!("{}\\.openclaw", userprofile.trim_end_matches(['\\', '/'])))
}

/// Resolves the full path to openclaw.json.
/// Uses OPENCLAW_CONFIG_PATH or state_dir\openclaw.json.
pub fn resolve_config_path() -> Result<String, String> {
    if let Ok(path) = env::var("OPENCLAW_CONFIG_PATH") {
        let trimmed = normalize_windows_path(&path);
        if !trimmed.is_empty() {
            return Ok(trimmed);
        }
    }
    let state_dir = resolve_state_dir()?;
    Ok(format!("{}\\openclaw.json", state_dir.trim_end_matches('\\')))
}
