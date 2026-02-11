//! OpenClaw Windows installer — Tauri commands for install, config, and security.

mod commands;
mod config;
mod install;
mod run;
mod security;

use std::sync::Mutex;

pub use commands::*;

/// Shared state for the installer (e.g. last audit report).
pub struct AppState {
    pub last_audit: Mutex<Option<security::AuditResult>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            last_audit: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            get_config_path,
            get_config_value,
            set_config_value,
            check_node,
            check_openclaw,
            install_node,
            install_openclaw,
            ensure_path,
            run_security_audit,
            run_security_fix,
            get_last_audit,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
