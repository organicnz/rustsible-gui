# 🚀 Quick Start Guide

<div align="center">

**Get your Ubuntu server running in minutes with beautiful interfaces**

</div>

---

## ⚡ TL;DR - Just Run This

```bash
# 🖥️ Pure Rust GUI (Recommended - Blazing Fast, ~2MB!)
./launch-gui
# Or: make gui

# 📝 Direct Ansible (Traditional)
ansible-playbook playbook.yml
```

---

## 🖥️ Pure Rust GUI (egui) ⭐ Recommended

> **Native desktop app - Blazing fast with real-time streaming**

### 🏃 Running the GUI

```bash
# From project root
./launch-gui

# Or using make
make gui

# Or build production version
make gui-build
# Binary at: gui-egui/target/release/ansible-provisioning-gui
```

### ✨ Features

<table>
<tr>
<td width="50%">

**🎯 User Experience**
- ✅ All options on one screen
- ✅ Modern dark-themed UI
- ✅ Auto-save settings (cached!)
- ✅ Real-time output streaming
- ✅ **Blazing fast** - immediate mode rendering
- ✅ **Tiny binary** - ~2MB (vs 80-150MB Electron)
- ✅ Cross-platform (macOS/Linux/Win)
- ✅ **No Node.js required** - pure Rust only

</td>
<td width="50%">

**🔌 Connection Settings**
- Server IP address
- SSH username (default: `root`)
- SSH key path (default: `~/.ssh/id_rsa`)
- Optional hostname

</td>
</tr>
</table>

### 📦 Available Features

#### **Core Features** (8 components)
- 🛡️ Fail2ban Intrusion Prevention
- 🐳 Docker & Docker Compose
- 🌐 LEMP Stack (Nginx, MySQL, PHP)
- 💾 Swap Memory Configuration
- ⏰ Automated Cron Jobs
- ⚙️ Development Tools (Neovim, Node.js, Claude Code)
- 📝 WordPress CMS
- 🔒 Certbot SSL/TLS Certificates

#### **Security Options** (7 features)
- 🔐 System Hardening
- 🛡️ AppArmor Enforcement
- 🔍 Rootkit Detection (rkhunter)
- 📁 File Integrity Monitoring (AIDE)
- 📊 Audit Logging (auditd)
- 📋 Log Monitoring (Logwatch)
- 🚀 Advanced Protection (2FA, Backups, USB restrictions)

### 🎬 Real-Time Output Streaming

When you click **"🚀 Launch Provisioning"**, the GUI shows live ansible-playbook output:
- 📊 Real-time streaming with immediate mode rendering
- 📜 Scrollable output log
- ✅ Success/error messages
- 🔄 Progress tracking

### 🔧 Prerequisites

```bash
# Install Rust (first time only)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or on macOS
brew install rust
```

The launcher script handles building automatically!

### 🐛 Troubleshooting

<details>
<summary><b>Problem</b>: <code>cargo not found</code></summary>

**Solution**: Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```
</details>

<details>
<summary><b>Problem</b>: <code>ansible-playbook not found</code></summary>

**Solution**: Install Ansible:
```bash
brew install ansible
```
</details>

<details>
<summary><b>Problem</b>: Build errors on Linux</summary>

**Solution**: Install development dependencies:
```bash
sudo apt install build-essential libgtk-3-dev
```
</details>

---

## 📝 Direct Ansible Playbook

> **Standard Ansible with beautified prompts**

```bash
ansible-playbook playbook.yml
```

Sequential prompts with color-coded output and visual indicators.

---

## 📊 Interface Comparison

<table>
<thead>
<tr>
<th>Feature</th>
<th align="center">🖥️ Pure Rust GUI</th>
<th align="center">📝 Direct Playbook</th>
</tr>
</thead>
<tbody>
<tr>
<td><b>All options visible</b></td>
<td align="center">✅ Yes</td>
<td align="center">❌ Sequential</td>
</tr>
<tr>
<td><b>Modern UI</b></td>
<td align="center">✅ Dark theme</td>
<td align="center">❌ Terminal only</td>
</tr>
<tr>
<td><b>Real-time output</b></td>
<td align="center">✅ Immediate mode</td>
<td align="center">✅ Terminal</td>
</tr>
<tr>
<td><b>Auto-save settings</b></td>
<td align="center">✅ Yes</td>
<td align="center">❌ No</td>
</tr>
<tr>
<td><b>Performance</b></td>
<td align="center">⚡⚡ Blazing (egui)</td>
<td align="center">⚡ Fast</td>
</tr>
<tr>
<td><b>Bundle Size</b></td>
<td align="center">📦 ~2MB</td>
<td align="center">⚡ N/A</td>
</tr>
<tr>
<td><b>Memory Usage</b></td>
<td align="center">💚 Low (~30MB)</td>
<td align="center">💚 Minimal</td>
</tr>
<tr>
<td><b>Setup required</b></td>
<td align="center">⚡ None (just Rust)</td>
<td align="center">⚡ None</td>
</tr>
<tr>
<td><b>Best for</b></td>
<td align="center">👍 Everyone</td>
<td align="center">🚀 Quick runs / CI/CD</td>
</tr>
</tbody>
</table>

---

## 🎯 Recommended Workflow

### 🆕 First Time Setup

```bash
# 1️⃣ Install required roles
ansible-galaxy install oefenweb.fail2ban

# 2️⃣ Run Pure Rust GUI to configure and provision
./launch-gui
# Or: make gui
```

### ⚡ Quick Updates

```bash
# Run specific role only
ansible-playbook playbook.yml --tags docker

# Run multiple tags
ansible-playbook playbook.yml --tags "security,firewall"

# Run with reboot
ansible-playbook playbook.yml -t reboot
```

---

## ✅ Default Selections

### 🖥️ Pure Rust GUI Defaults

**Core Features:**
- ✅ Fail2ban
- ✅ Docker
- ✅ Swap Memory
- ✅ Cron Jobs
- ✅ Dev Tools
- ❌ LEMP Stack
- ❌ WordPress
- ❌ Certbot

**Security Options:**
- ❌ All disabled by default (enable as needed)

These are sensible defaults for most server setups. You can change them in the GUI before launching.

---

## 🎉 After Provisioning

### ✅ Check Services

```bash
# SSH into your server
ssh user@server

# 🐳 Check Docker
docker --version
docker-compose --version

# 🛡️ Check fail2ban
sudo fail2ban-client status

# 🔥 Check firewall
sudo ufw status

# 💾 Check swap
swapon -s
```

### 📝 WordPress Setup (if enabled)

1. Visit `http://your-ip/wp-admin/install.php`
2. Complete WordPress installation wizard
3. Enable Nginx vhost: Set `configure_nginx_vhost: true` in `vars/default.yml`
4. Install SSL: Enable `install_certbot: true`

---

## 🆘 Need Help?

<table>
<tr>
<td>

**📚 Documentation**
- Read full [README.md](readme.md) for detailed documentation
- Check `ansible.log` for playbook output
- Review role-specific documentation in `roles/*/README.md`

</td>
<td>

**🐛 Debugging**
- Use `--check` flag for dry runs
- Use `--tags` for selective execution
- Use `-vv` or `-vvv` for verbose output
- Check [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md)

</td>
</tr>
</table>

---

<div align="center">

**🎉 Happy Provisioning! 🎉**

*Made with ❤️ by the Ubuntu Server Automation Team*

</div>
