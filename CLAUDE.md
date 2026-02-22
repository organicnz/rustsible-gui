# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Rustsible is a **pure Rust desktop GUI** for Ansible server provisioning, built with **egui/eframe 0.31**. It provides a macOS-native dark theme sidebar+detail layout for configuring and running `ansible-playbook` against remote Linux VPS servers.

**Key Capabilities:**
- Single-binary desktop app (no webview, no Node.js)
- macOS HIG dark mode styling with NSColor-equivalent palette
- Sidebar navigation: Connection, Features, Security, Maintenance, Tasks, Output
- Tasks section with unified scrollable view, Select All / Deselect All, and grouped categories
- Real-time ansible-playbook output streaming with color-coded terminal
- Config auto-saved to `~/.ansible_provisioning_cache.json`
- Graceful process cleanup on exit (SIGTERM/SIGINT handlers)
- Previous instance cleanup on startup

**Technology Stack:**
- **GUI Framework**: egui 0.31 / eframe 0.31
- **Async Runtime**: tokio (for ansible-playbook subprocess management)
- **Serialization**: serde / serde_json (config caching)
- **Process Management**: sysinfo, signal-hook
- **ANSI Stripping**: regex

## Project Structure

```
.
├── gui-egui/                     # Rust GUI application
│   ├── src/
│   │   └── main.rs              # Entire GUI application (single file)
│   ├── Cargo.toml               # Rust dependencies
│   └── target/                  # Build output
├── playbook.yml                 # Main Ansible playbook (2 plays)
├── roles/                       # Ansible roles
│   ├── base_setup/              # User management, packages, hostname
│   ├── swap/                    # Swap memory configuration
│   ├── security/                # SSH hardening, firewall, kernel, 2FA, backups
│   ├── docker/                  # Docker & Docker Compose
│   ├── lemp/                    # Nginx, MySQL, PHP
│   ├── dev_tools/               # Neovim, Node.js, Claude Code
│   ├── wordpress/               # WordPress CMS
│   ├── certbot/                 # SSL/TLS certificates
│   └── cron_jobs/               # Automated updates, periodic reboot
├── vars/
│   └── default.yml              # Default variable values for all roles
└── CLAUDE.md                    # This file
```

## Common Commands

```bash
# Build release binary
cd gui-egui && cargo build --release

# Run the app
./target/release/rustsible-gui

# Build and run
cargo run --release

# Check for warnings without building
cargo check
```

## Architecture

### GUI (gui-egui/src/main.rs)

Everything is in a single `main.rs` file:

- **`macos_colors` module** — NSColor-equivalent constants (WINDOW_BG, SIDEBAR_BG, TOOLBAR_BG, ACCENT_BLUE, SYSTEM_RED/GREEN/YELLOW, etc.)
- **`NavSection` enum** — Connection, Features, Security, Maintenance, Tasks, Output
- **`ProvisioningConfig` struct** — All configurable fields with serde serialization. New fields must use `#[serde(default)]` or `#[serde(default = "default_true")]` for backward cache compatibility.
- **`AnsibleProvisioningApp` struct** — Main app state: config, provisioning status, output lines, thread handles, signal flags, nav selection
- **`render_*` methods** — One per NavSection. `render_tasks()` provides unified view with Select All/Deselect All and grouped sections.
- **`setup_custom_style()`** — Configures egui Visuals::dark() with macOS-standard 6px rounding, compact density
- **`run_provisioning()`** — Async function that spawns `ansible-playbook` subprocess, streams stdout/stderr via channels
- **`cleanup_previous_instances()`** — Kills stale processes on startup (targeted by PID, never killall)

### Ansible Playbook (playbook.yml)

Two-play structure:
1. **Play 1 (localhost)**: Validates SSH connectivity, converts `prompt_*` variables to `install_*` host variables, builds dynamic inventory
2. **Play 2 (target host)**: Runs roles with conditional guards based on host variables

### Variable Flow (GUI -> Ansible)

The GUI passes variables via `ansible-playbook -e` flags:

| GUI Config Field | Ansible Variable | Used By |
|---|---|---|
| `fail2ban` | `prompt_enable_fail2ban` | Play 1 -> `enable_fail2ban` |
| `docker` | `prompt_install_docker` | Play 1 -> `install_docker` |
| `swap` | `prompt_enable_swap` | Play 1 -> `enable_swap` |
| `lemp` | `prompt_install_lemp` | Play 1 -> `install_lemp` |
| `devtools` | `prompt_install_dev_tools` | Play 1 -> `install_dev_tools` |
| `wordpress` | `prompt_install_wordpress` | Play 1 -> `install_wordpress` |
| `certbot` | `prompt_install_certbot` | Play 1 -> `install_certbot` |
| `cron_jobs` | `prompt_enable_cron_jobs` | Play 1 -> `enable_cron_jobs` |
| `periodic_reboot` | `prompt_enable_periodic_reboot` | Play 1 -> `enable_periodic_reboot` |
| `system_hardening` | `enable_kernel_hardening` | Direct to security role |
| `apparmor` | `enable_apparmor` | Direct to security role |
| `rootkit_detection` | `enable_rkhunter` | Direct to security role |
| `file_integrity` | `enable_aide` | Direct to security role |
| `audit_logging` | `enable_auditd` | Direct to security role |
| `log_monitoring` | `enable_logwatch` | Direct to security role |
| `ssh_2fa_totp/fido2/duo` | `enable_ssh_2fa` | Direct to security role |
| `backups` | `enable_backups` | Direct to security role |
| `usb_restrictions` | `enable_usb_restrictions` | Direct to security role |
| `secure_shm` | `enable_secure_shm` | Direct to security role |
| `lynis` | `enable_lynis` | Direct to security role |
| `disable_ipv6` | `disable_ipv6` | Direct to security role |
| `suricata` | `enable_suricata` | Direct to security role |

**Important**: Variables prefixed with `prompt_` are converted to host variables (without prefix) in Play 1. Security variables are passed directly and used in Play 2 via `vars/default.yml` defaults.

### Known Gotchas

- **ip_forward + Docker**: Kernel hardening sets `net.ipv4.ip_forward=0` which breaks Docker. The security role re-enables it to `1` via `/etc/sysctl.d/99-docker.conf` when both `enable_kernel_hardening` and `install_docker` are true.
- **SSH 2FA enrollment**: The `enable_ssh_2fa` flag installs the PAM module and configures SSH, but each user must manually run `google-authenticator` on the server to complete enrollment. The `nullok` PAM option allows login without 2FA until enrolled.
- **Cache backward compatibility**: When adding new fields to `ProvisioningConfig`, always use `#[serde(default)]` or `#[serde(default = "default_true")]` so old cache files still deserialize.
- **Process cleanup**: Never use `killall` or `pkill` by process name in `cleanup_previous_instances()` — it kills the current process too. Always target specific PIDs.

## Naming Conventions

**Rust:**
- `snake_case` for functions, variables, modules
- `PascalCase` for types, structs, enums
- `SCREAMING_SNAKE_CASE` for constants (in `macos_colors` module)

**Ansible:**
- `snake_case` for all variables
- Role names use `snake_case`
- `prompt_*` prefix for CLI input variables
- `enable_*` / `install_*` prefix for feature flags
