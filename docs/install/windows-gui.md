---
summary: "Native Windows GUI installer: install OpenClaw and configure it without WSL"
read_when:
  - You cannot or prefer not to use WSL on Windows
  - You want a GUI to install and configure OpenClaw on Windows
title: "Windows GUI installer"
---

# Windows GUI installer

The **OpenClaw Setup** app is a native Windows .exe that installs and configures OpenClaw **without WSL**. Use it if you cannot or prefer not to run OpenClaw inside WSL2.

## What it does

1. **Install**: Checks for Node.js 22+ (installs via winget if missing), then runs `npm install -g openclaw@latest`. Helps you ensure the npm global bin directory is on your PATH.
2. **Configure**: GUI to set gateway mode (local/remote), port, Brave Search API key, and other key options. Writes to the same config file the CLI uses: `%USERPROFILE%\.openclaw\openclaw.json` (or `OPENCLAW_CONFIG_PATH` / `OPENCLAW_STATE_DIR` if set).
3. **Security check**: Runs `openclaw security audit --json` and shows a **security score from 1 to 100**. You can run a full (deep) check, view findings, and run **Run fix** to apply safe fixes.

## Getting the app

- **From CI**: Build artifacts are produced by the [Windows Installer workflow](https://github.com/openclaw/openclaw/actions) when `apps/windows-installer` changes. Download the `openclaw-windows-installer` artifact from the latest run on `main` or from a PR.
- **From source**: Clone the repo, install [Rust](https://rustup.rs/) and [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) (Windows: WebView2 is usually already present), then run:
  ```powershell
  cd apps\windows-installer
  cargo install tauri-cli --version "^2.0.0"
  cargo tauri build
  ```
  The .exe is under `src-tauri\target\release\`.

See [apps/windows-installer/README.md](https://github.com/openclaw/openclaw/blob/main/apps/windows-installer/README.md) in the repo for build details.

## After install

- Open a **new** terminal (or restart the app) so PATH updates are picked up.
- Run `openclaw onboard` or use the Configure tab in the GUI to set gateway, model, and channels.
- To run the gateway in the background on Windows, use `openclaw gateway install` (installs a Windows Scheduled Task) or run `openclaw gateway run` in a terminal.

## Configuration and security

- Config file: [Configuration](/gateway/configuration).
- Security audit: [Security audit](/cli/security). The GUI score is computed from the same audit (1–100; higher is better; "Run fix" applies safe hardening).

## Links

- [Windows (WSL2)](/platforms/windows) — overview of Windows options (WSL2 vs native).
- [Configuration](/gateway/configuration) — all config options.
- [Installer internals](/install/installer) — script-based installers (install.ps1, install.sh).
