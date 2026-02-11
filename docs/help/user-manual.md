---
summary: "End-to-end user guide: install, configure, channels, security, and troubleshooting"
read_when:
  - You want a single place to learn OpenClaw from zero
  - You need install, config, and daily-use reference in one doc
title: "User manual"
---

# User manual

This manual walks you through installing OpenClaw, configuring it, connecting channels, and using it day to day. For a very short path to a first chat, see [Getting started](/start/getting-started).

## Introduction

**OpenClaw** is a personal AI assistant you run on your own machines. It talks to you on the apps you already use (WhatsApp, Telegram, Slack, Discord, Google Chat, Signal, iMessage, Microsoft Teams, WebChat, and more). The **Gateway** is the control plane: it keeps config, sessions, and channel connections on your side. You choose which AI models and API keys to use.

**Who it is for:** Anyone who wants a single-user, local-first assistant that can reply on multiple messaging channels, use tools (browser, canvas, nodes), and optionally use voice (macOS/iOS/Android). Optional companion apps add a macOS menu bar, iOS/Android nodes, and Voice Wake.

## Installation

You need **Node.js 22 or newer**. Check with `node --version`.

### macOS and Linux

```bash
curl -fsSL https://openclaw.ai/install.sh | bash
```

Then open a new terminal (or re-source your shell config) so `openclaw` is on your PATH.

### Windows (PowerShell)

```powershell
iwr -useb https://openclaw.ai/install.ps1 | iex
```

Use a **new** terminal after install so PATH updates apply.

### Windows (no WSL, GUI installer)

If you prefer not to use WSL, use the native **Windows GUI installer**. It installs Node if needed, runs `npm install -g openclaw@latest`, helps with PATH, and provides a GUI for config and security checks.

- Download the installer artifact from the [Windows Installer workflow](https://github.com/openclaw/openclaw/actions) or build from source. Full steps: [Windows GUI installer](/install/windows-gui).

### npm (any OS)

```bash
npm install -g openclaw@latest
```

Ensure the global npm bin directory is on your PATH. See [Install](/install) for Node/npm PATH sanity and other methods (Docker, Nix, from source).

## First run

1. **Run the onboarding wizard**

   ```bash
   openclaw onboard --install-daemon
   ```

   The wizard sets up auth (e.g. Anthropic, OpenAI), gateway options, optional channels, and installs the Gateway as a background service (launchd on macOS, systemd on Linux, Scheduled Task on Windows). Details: [Onboarding wizard](/start/wizard).

2. **Check gateway status**

   ```bash
   openclaw gateway status
   ```

   You should see the gateway running and RPC probe OK. If not, see [Gateway troubleshooting](/gateway/troubleshooting) and [Doctor](/gateway/doctor).

3. **Open the Control UI**

   ```bash
   openclaw dashboard
   ```

   Or open `http://127.0.0.1:18789/` in a browser on the gateway host. You can chat in the Control UI without configuring any messaging channel.

## Configuration

- **Config file:** By default `~/.openclaw/openclaw.json` (or `%USERPROFILE%\.openclaw\openclaw.json` on Windows). Override with `OPENCLAW_CONFIG_PATH` or `OPENCLAW_STATE_DIR` if needed.
- **Main settings:** Gateway port and bind, default model, channel enablement, DM policy, and more. Full reference: [Configuration](/gateway/configuration).
- **CLI:** Use `openclaw config get <key>` and `openclaw config set <key> <value>` to read and write config. Interactive: `openclaw configure`.

Minimal example (default model only):

```json
{
  "agent": {
    "model": "anthropic/claude-opus-4-6"
  }
}
```

## Channels

Channels are how OpenClaw receives and sends messages (WhatsApp, Telegram, Slack, Discord, Google Chat, Signal, iMessage/BlueBubbles, Microsoft Teams, WebChat, etc.). The onboarding wizard can guide you through linking and pairing.

- **Overview and concepts:** [Channels](/channels).
- **Per-channel setup:** See the channel docs (e.g. [WhatsApp](/channels/whatsapp), [Telegram](/channels/telegram), [Slack](/channels/slack), [Discord](/channels/discord)).
- **Pairing (DM access):** By default, unknown DMs get a pairing code; you approve with `openclaw pairing approve <channel> <code>`. See [Pairing](/channels/pairing) and [Security](/gateway/security).

Channel status:

```bash
openclaw channels status
openclaw channels status --probe
```

## Security

- **DM policy:** Default is pairing-only for DMs; only people you approve can talk to the assistant. See [Security](/gateway/security).
- **Security audit:** Run `openclaw security audit` for a report and hardening tips. The Windows GUI can show a 1–100 score and "Run fix" for safe fixes. See [Security audit](/cli/security).
- **Exposing the gateway:** If you use Tailscale or SSH tunnels for remote access, read [Remote access](/gateway/remote) and [Security](/gateway/security) before opening the dashboard to the internet.

## Common tasks

- **Send a one-off message:** `openclaw message send --to <channel/peer> --message "Your text"`. See [message send](/cli/message).
- **Talk to the agent (CLI):** `openclaw agent --message "Your question" --thinking low`. Reply can be delivered to any connected channel. See [agent](/cli/agent).
- **Chat commands in channels:** In WhatsApp/Telegram/Slack/WebChat etc. you can use `/status`, `/new`, `/compact`, `/think <level>`, `/verbose on|off`, `/usage off|tokens|full`, and in groups `/activation mention|always`. See [Slash commands](/tools/slash-commands).
- **Reset session:** `/new` or `/reset` in chat, or start a new session from the Control UI.

## Troubleshooting

- **First steps:** Run in order: `openclaw status`, `openclaw gateway status`, `openclaw doctor`. See [Troubleshooting](/help/troubleshooting).
- **Gateway not running:** Use `openclaw gateway status` and, if you use the installed service, restart it (e.g. via the OpenClaw Mac app on macOS, or the service manager on Linux/Windows). [Gateway runbook](/gateway) and [Background process](/gateway/background-process).
- **Logs:** `openclaw logs --follow`. See [Logging](/logging) and [Gateway logging](/gateway/logging).
- **Config and repairs:** `openclaw doctor` checks config and can apply safe fixes. See [Doctor](/gateway/doctor).

## Reference

- [Full configuration reference](/gateway/configuration)
- [CLI reference](/cli)
- [Gateway runbook](/gateway)
- [Docs index](/) for all guides and deep dives
