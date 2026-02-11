# OpenClaw Windows Installer

Native Windows GUI installer and configurator for OpenClaw. No WSL required.

## What it does

- **Install**: Ensures Node.js 22+ is present (via winget if needed), installs OpenClaw CLI with `npm install -g openclaw@latest`, and helps with PATH.
- **Configure**: Edit gateway (mode, port), Brave Search API key, and other key settings. Writes to the same `openclaw.json` the CLI uses (`%USERPROFILE%\.openclaw\openclaw.json` unless overridden).
- **Security check**: Runs `openclaw security audit --json`, shows a **security score (1–100)**, and offers "Run fix" and "View details".

## Prerequisites

- **Windows 10/11** (native; this app does not use WSL).
- **Rust** (stable) and **Visual Studio Build Tools** (or equivalent) to build from source. For Tauri 2 you also need the [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) (e.g. WebView2 on Windows, which is usually already present).

## Build

From the repo root or this directory:

```bash
cd apps/windows-installer
cargo install tauri-cli --version "^2.0.0"
cargo tauri build
```

Or with Rust and Tauri CLI already installed:

```bash
cargo tauri build
```

The built executable and optional installer will be under `src-tauri/target/release/` (and `target/release/bundle/` for NSIS/installer output if configured).

## Run in development

```bash
cargo tauri dev
```

This builds the Rust app and opens the window. The UI is static HTML/JS in `ui/` (no separate frontend build).

## Project layout

- `src-tauri/` — Tauri 2 Rust app (commands, config path resolution, install and security logic).
- `ui/` — Static frontend (`index.html`, `style.css`, `app.js`). Loaded by Tauri via `frontendDist: "../ui"`.

## Config path

Same as the OpenClaw CLI:

- Config file: `%USERPROFILE%\.openclaw\openclaw.json` unless `OPENCLAW_CONFIG_PATH` or `OPENCLAW_STATE_DIR` is set.
- See [Configuration](https://docs.openclaw.ai/gateway/configuration) and [Windows](https://docs.openclaw.ai/platforms/windows).

## Security

- Config reads/writes use `openclaw config get/set` (no shell; arguments passed as arrays) to avoid injection.
- Security score is computed from `openclaw security audit --json` (same audit as the CLI).
- No secrets are kept in process memory longer than needed when writing config.
