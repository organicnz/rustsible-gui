# 🖥️ Ubuntu Server Provisioning - GUI Documentation

This directory contains documentation for both GUI implementations:

1. **Pure Rust GUI (egui)** ⭐ **RECOMMENDED** - See `../gui-egui/README.md`
   - ~2MB single binary
   - Blazing fast immediate mode rendering
   - Perfect for real-time log streaming
   - No Node.js required

2. **Tauri GUI** (Legacy) - This document
   - ~3-5MB binary
   - Svelte + Rust
   - Requires Node.js for development

---

# 🖥️ Tauri GUI (Legacy)

A cross-platform desktop application for Ubuntu server provisioning built with **Tauri** (Rust) and **Svelte**.

> **Note**: The Pure Rust GUI (egui) is now recommended for better performance and smaller bundle size.

## ✨ Features

- 🎨 **Modern Dark UI** - Beautiful, responsive interface with smooth animations
- 💾 **Auto-Save** - Settings automatically cached between sessions
- 📊 **Real-Time Output** - Live streaming of Ansible provisioning output
- ⚡ **Native Performance** - Rust backend for fast, efficient execution
- 🔐 **SSH Key Auth** - Secure key-based authentication
- 🖥️ **Cross-Platform** - Works on macOS, Linux, and Windows
- 🎯 **All-in-One View** - All configuration options visible at once
- 🛡️ **Security Focused** - 7 security options + 8 core features

## 🚀 Quick Start

### Development Mode

```bash
# Install dependencies (first time only)
npm install

# Run in development mode
./RUN_GUI.sh

# Or manually
npm run tauri dev
```

### Production Build

```bash
# Build for your platform
./BUILD.sh

# Or manually
npm run tauri build

# Binaries will be in: src-tauri/target/release/bundle/
```

## 📋 Prerequisites

### System Requirements

- **Node.js** 18+ and npm
- **Rust** 1.70+ (install from https://rustup.rs)
- **System dependencies** (platform-specific):

#### macOS
```bash
# Xcode Command Line Tools
xcode-select --install
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

#### Windows
- Install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)

### Ansible Requirements

The GUI requires Ansible to be installed on your system:

```bash
# macOS
brew install ansible

# Ubuntu/Debian
sudo apt install ansible

# Or via pip
pip3 install ansible
```

## 🎯 Usage

### Connection Setup

1. **IP Address** - Target server IP (e.g., `192.168.1.100`)
2. **SSH User** - Username with sudo privileges (default: `root`)
3. **SSH Key Path** - Path to your private key (supports `~` for home directory)
4. **Hostname** - Optional server hostname

### Feature Selection

#### Core Features
- 🛡️ **Fail2ban** - Intrusion prevention system
- 🐳 **Docker** - Container platform + Docker Compose
- 💾 **Swap** - Auto-sized swap memory
- 🌐 **LEMP** - Nginx, MySQL, PHP stack
- ⚙️ **Dev Tools** - Neovim, Node.js, Claude Code
- 📝 **WordPress** - CMS (requires LEMP)
- 🔒 **Certbot** - SSL/TLS certificates
- ⏰ **Cron Jobs** - Automated maintenance

#### Security Options
- 🔐 **System Hardening** - Kernel hardening, secure shared memory
- 🛡️ **AppArmor** - Mandatory access control
- 🔍 **Rootkit Detection** - Daily malware scans (rkhunter)
- 📁 **File Integrity** - Unauthorized change tracking (AIDE)
- 📊 **Audit Logging** - System call auditing (auditd)
- 📋 **Log Monitoring** - Daily log analysis (Logwatch)
- 🚀 **Advanced Protection** - 2FA, backups, USB restrictions

#### Maintenance
- 🔄 **Periodic Reboot** - Scheduled system reboots
- ⏰ **Reboot Schedule** - Choose frequency (daily, every 6/12/24 hours)

### Running Provisioning

1. Configure all settings in the GUI
2. Click **🚀 Launch Provisioning**
3. Watch real-time output in the log window
4. Wait for completion (✅ Success or ❌ Error)

Settings are automatically saved and restored on next launch.

## 🏗️ Architecture

### Tech Stack

- **Frontend**: Svelte 5 + TypeScript + Vite
- **Backend**: Rust + Tauri 2
- **Styling**: Custom CSS with dark theme
- **IPC**: Tauri commands for Rust ↔ JavaScript communication

### Project Structure

```
gui-tauri/
├── src/                    # Svelte frontend
│   ├── routes/
│   │   └── +page.svelte   # Main UI component
│   └── app.html           # HTML template
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   └── lib.rs         # Tauri commands
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── package.json           # Node dependencies
├── RUN_GUI.sh            # Development launcher
└── BUILD.sh              # Production builder
```

### Tauri Commands

The Rust backend exposes these commands to the frontend:

- `load_cache()` - Load saved configuration
- `save_cache(config)` - Save configuration to disk
- `launch_provisioning(config)` - Execute Ansible playbook
- `validate_ip(ip)` - Validate IP address format

### Event Streaming

Real-time output is streamed via Tauri events:

- `provisioning-output` - Standard output lines
- `provisioning-error` - Error output lines
- `provisioning-complete` - Completion status

## 🔧 Development

### Install Dependencies

```bash
npm install
```

### Run Development Server

```bash
npm run tauri dev
```

This starts:
1. Vite dev server (frontend hot reload)
2. Tauri window with Rust backend

### Build for Production

```bash
npm run tauri build
```

Outputs:
- **macOS**: `.dmg` and `.app` in `src-tauri/target/release/bundle/`
- **Linux**: `.deb`, `.AppImage` in `src-tauri/target/release/bundle/`
- **Windows**: `.msi`, `.exe` in `src-tauri/target/release/bundle/`

### Type Checking

```bash
npm run check
```

### Format Code

```bash
# Rust
cd src-tauri
cargo fmt

# Svelte/TypeScript
npm run format
```

## 🐛 Troubleshooting

### "ansible-playbook not found"

Ensure Ansible is installed and in your PATH:
```bash
which ansible-playbook
ansible-playbook --version
```

### "SSH key not found"

Verify your SSH key path:
```bash
ls -la ~/.ssh/
# Common keys: id_rsa, id_ed25519, id_rsa_gitlab
```

### "Failed to spawn ansible-playbook"

Check that the playbook exists:
```bash
ls -la ../playbook.yml
```

The GUI looks for `playbook.yml` in the parent directory.

### Port Already in Use

If you see "Port 1420 is already in use":
```bash
# The RUN_GUI.sh script automatically kills port conflicts
# But you can also manually clean up:
./KILL_PORTS.sh
```

### Build Errors

Clear caches and rebuild:
```bash
# Clean Rust build
cd src-tauri
cargo clean

# Clean Node modules
cd ..
rm -rf node_modules
npm install

# Rebuild
npm run tauri build
```

### WebView Issues (Linux)

Install WebKit2GTK:
```bash
sudo apt install libwebkit2gtk-4.0-dev
```

## 📝 Configuration Cache

Settings are saved to:
- **macOS/Linux**: `~/.ansible_provisioning_cache.json`
- **Windows**: `%USERPROFILE%\.ansible_provisioning_cache.json`

To reset settings, delete this file.

## 🔒 Security Notes

- SSH keys are never stored in the cache
- Only the key path is saved
- All provisioning runs locally (no cloud services)
- Ansible output is streamed but not logged to disk
- Configuration cache is stored in plain text (contains no secrets)

## 🚀 Performance

- **Startup**: < 1 second (native Rust)
- **Memory**: ~50-100 MB (vs 200-500 MB for Electron)
- **Binary Size**: ~10-20 MB (vs 100+ MB for Electron)
- **CPU**: Minimal idle usage

## 📦 Distribution

### macOS

```bash
# Build
npm run tauri build

# Sign (optional)
codesign --force --deep --sign - "src-tauri/target/release/bundle/macos/gui-tauri.app"

# Create DMG
# Already created at: src-tauri/target/release/bundle/dmg/gui-tauri_*.dmg
```

### Linux

```bash
# Build
npm run tauri build

# Outputs:
# - .deb package (Debian/Ubuntu)
# - .AppImage (universal)
```

### Windows

```bash
# Build
npm run tauri build

# Outputs:
# - .msi installer
# - .exe portable
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

MIT License - see parent project for details

## 🙏 Credits

Built with:
- [Tauri](https://tauri.app/) - Desktop app framework
- [Svelte](https://svelte.dev/) - Reactive UI framework
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Ansible](https://www.ansible.com/) - Infrastructure automation

---

**Need help?** Open an issue in the main repository.
