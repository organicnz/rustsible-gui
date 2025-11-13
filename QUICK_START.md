# 🚀 Quick Start Guide

<div align="center">

**Get your Ubuntu server running in minutes with beautiful interfaces**

</div>

---

## ⚡ TL;DR - Just Run This

```bash
# 🖥️  GUI Interface (Recommended - Auto-saves settings!)
./RUN_GUI.sh

# ⌨️  CLI with Checkboxes (Power users)
./RUN_ME.sh

# 📝 Direct Ansible (Traditional)
ansible-playbook playbook.yml
```

---

## 🖥️ GUI Interface

> **The easiest way to configure your server provisioning**

### 🏃 Running the GUI

```bash
./RUN_GUI.sh
```

### ✨ Features

<table>
<tr>
<td width="50%">

**🎯 User Experience**
- ✅ All options on one screen
- ✅ Real GUI checkboxes
- ✅ Auto-save settings (cached!)
- ✅ Live terminal output
- ✅ Beautiful dialogs

</td>
<td width="50%">

**🔌 Connection Settings**
- Server IP address
- SSH username (default: `root`)
- SSH key path (default: `~/.ssh/id_rsa_gitlab`)

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

#### **Security Clusters** (4 clusters)
- 🔐 **System Hardening**: Kernel security, AppArmor, Auto-updates
- 📊 **Monitoring & Detection**: Lynis, AIDE, rkhunter, Log analysis
- 🌐 **Network Security**: IPv6 disable, Network IDS
- 🔑 **Advanced Protection**: 2FA, Automated backups, USB restrictions

### 🎬 Live Output Window

When you click **"Launch Provisioning"**, a terminal window opens showing live ansible-playbook output with beautiful colors:
- 🟢 Green for successful tasks
- 🟡 Yellow for changed tasks
- 🔴 Red for failed tasks

### 🔧 Prerequisites

The GUI requires tkinter, which is automatically installed:

```bash
# Already installed via Homebrew
brew install python-tk@3.13
```

The launcher script automatically uses Python 3.13 with tkinter support.

### 🐛 Troubleshooting

<details>
<summary><b>Problem</b>: <code>ModuleNotFoundError: No module named '_tkinter'</code></summary>

**Solution**: Install tkinter for Homebrew Python:
```bash
brew install python-tk@3.13
```
</details>

<details>
<summary><b>Problem</b>: <code>macOS 26 (2601) or later required</code></summary>

**Solution**: This happens with system Python. The launcher now uses Homebrew Python 3.13 which has full tkinter support.
</details>

---

## ⌨️ CLI Interface (Advanced)

> **Interactive terminal interface with checkbox selection**

### 🏃 Running the CLI

```bash
./RUN_ME.sh
```

### ✨ Features

| Feature | Description |
|---------|-------------|
| 👀 **Preview Screen** | See all questions before answering |
| ☑️ **Checkbox Selection** | Use arrow keys and space bar to select |
| 🔧 **Auto-Setup** | Creates virtual environment and installs dependencies |
| ✅ **Validation** | WordPress requires LEMP (checked automatically) |
| 🎨 **Color-Coded Summary** | Visual feedback with feature breakdowns |
| ⏱️ **Timer** | Shows elapsed time when complete |
| 📊 **Next Steps** | Context-aware recommendations based on selections |

### 📋 Selection Process

```
1️⃣ Preview screen shows all 4 questions
2️⃣ Enter server IP address
3️⃣ Enter SSH username (default: root)
4️⃣ Enter SSH private key path
5️⃣ Select features with checkboxes (↑↓ arrow keys + space)
6️⃣ Review configuration summary with color indicators
7️⃣ Confirm and launch provisioning
```

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
<th align="center">🖥️ GUI</th>
<th align="center">⌨️ CLI</th>
<th align="center">📝 Direct</th>
</tr>
</thead>
<tbody>
<tr>
<td><b>All options visible</b></td>
<td align="center">✅ Yes</td>
<td align="center">❌ No</td>
<td align="center">❌ No</td>
</tr>
<tr>
<td><b>Checkboxes</b></td>
<td align="center">✅ GUI</td>
<td align="center">✅ Terminal</td>
<td align="center">❌ No</td>
</tr>
<tr>
<td><b>Live output window</b></td>
<td align="center">✅ Yes</td>
<td align="center">❌ No</td>
<td align="center">✅ Yes</td>
</tr>
<tr>
<td><b>Auto-save settings</b></td>
<td align="center">✅ Yes</td>
<td align="center">❌ No</td>
<td align="center">❌ No</td>
</tr>
<tr>
<td><b>Setup required</b></td>
<td align="center">None</td>
<td align="center">Auto</td>
<td align="center">None</td>
</tr>
<tr>
<td><b>Best for</b></td>
<td align="center">Visual preference</td>
<td align="center">Power users</td>
<td align="center">Quick runs</td>
</tr>
</tbody>
</table>

---

## 🎯 Recommended Workflow

### 🆕 First Time Setup

```bash
# 1️⃣ Configure your server IP and SSH key
vim vars/default.yml

# 2️⃣ Install required roles
ansible-galaxy install oefenweb.fail2ban

# 3️⃣ Run GUI to select features
./RUN_GUI.sh
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

<table>
<tr>
<td width="50%" valign="top">

### 🖥️ GUI Defaults

- ✅ Fail2ban
- ✅ Docker
- ✅ Swap Memory
- ✅ Cron Jobs
- ✅ System Hardening
- ✅ Monitoring & Detection
- ❌ LEMP Stack
- ❌ WordPress
- ❌ Certbot
- ❌ Dev Tools
- ❌ Network Security
- ❌ Advanced Protection

</td>
<td width="50%" valign="top">

### ⌨️ CLI Defaults

Same as GUI defaults ⬅️

These are sensible defaults for most server setups. You can change them before launching.

</td>
</tr>
</table>

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
