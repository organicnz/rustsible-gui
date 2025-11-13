# Quick Start Guide

## TL;DR - Just Run This

```bash
# GUI Interface (Recommended)
./RUN_GUI.sh

# CLI with Checkboxes
./RUN_ME.sh

# Direct Ansible
ansible-playbook playbook.yml
```

## GUI Interface

The GUI provides the easiest way to configure your server provisioning:

### Running the GUI

```bash
./RUN_GUI.sh
```

### Features

- **All options on one screen** - No sequential prompts
- **Real GUI checkboxes** - Visual, clickable interface
- **Auto-save settings** - Your configuration is cached automatically
- **Connection fields**:
  - Server IP address
  - SSH username (default: root)
  - SSH private key path (default: ~/.ssh/id_rsa_gitlab)

- **8 Core Features**:
  - Fail2ban Intrusion Prevention
  - Docker & Docker Compose
  - LEMP Stack (Nginx, MySQL, PHP)
  - Swap Memory Configuration
  - Automated Cron Jobs
  - Development Tools (Neovim, Node.js, Claude Code)
  - WordPress CMS
  - Certbot SSL/TLS Certificates

- **4 Security Clusters**:
  - System Hardening (Kernel, AppArmor, Auto-updates)
  - Monitoring & Detection (Lynis, AIDE, rkhunter, Logs)
  - Network Security (IPv6 disable, IDS)
  - Advanced Protection (2FA, Backups, USB)

### Live Output

When you click "Launch Provisioning", a terminal window opens showing live ansible-playbook output!

### Prerequisites

The GUI requires tkinter, which is now automatically installed:

```bash
# Already installed via Homebrew
brew install python-tk@3.13
```

The launcher script automatically uses Python 3.13 with tkinter support.

### Troubleshooting

**Problem**: `ModuleNotFoundError: No module named '_tkinter'`

**Solution**: Install tkinter for Homebrew Python:
```bash
brew install python-tk@3.13
```

**Problem**: `macOS 26 (2601) or later required`

**Solution**: This happens with system Python. The launcher now uses Homebrew Python 3.13 which has full tkinter support.

## CLI Interface (Advanced)

Interactive terminal interface with checkbox selection:

```bash
./RUN_ME.sh
```

### Features

- **Preview screen** - See all questions before answering
- **Checkbox selection** - Use arrow keys and space bar
- **Auto-setup** - Creates virtual environment and installs dependencies
- **Validation** - WordPress requires LEMP
- **Color-coded summary** - Visual feedback with feature breakdowns

### Selection Process

1. Preview screen shows all 4 questions
2. Enter server IP address
3. Enter SSH username (default: root)
4. Enter SSH private key path
5. Select features with checkboxes (arrow keys + space)
6. Review configuration summary
7. Confirm and launch

## Direct Ansible Playbook

Standard Ansible with beautified prompts:

```bash
ansible-playbook playbook.yml
```

Sequential prompts with color-coded output.

## Comparison

| Feature | GUI | CLI | Direct |
|---------|-----|-----|--------|
| All options visible | ✅ | ❌ | ❌ |
| Checkboxes | ✅ GUI | ✅ Terminal | ❌ |
| Live output window | ✅ | ❌ | ✅ |
| Setup required | None | Auto | None |
| Best for | Visual preference | Power users | Quick runs |

## Recommended Workflow

### First Time Setup
```bash
# 1. Configure your server IP and SSH key
vim vars/default.yml

# 2. Install required roles
ansible-galaxy install oefenweb.fail2ban

# 3. Run GUI to select features
./RUN_GUI.sh
```

### Quick Updates
```bash
# Run specific role only
ansible-playbook playbook.yml --tags docker

# Run with tags
ansible-playbook playbook.yml --tags "security,firewall"
```

## What Gets Selected by Default?

### GUI Defaults
- Fail2ban: ✅ Enabled
- Docker: ✅ Enabled
- Swap: ✅ Enabled
- Cron Jobs: ✅ Enabled
- System Hardening: ✅ Enabled
- Monitoring & Detection: ✅ Enabled

### CLI Defaults
Same as GUI defaults

These are sensible defaults for most server setups. You can change them before launching.

## After Provisioning

### Check Services
```bash
ssh user@server

# Check Docker
docker --version
docker-compose --version

# Check fail2ban
sudo fail2ban-client status

# Check firewall
sudo ufw status

# Check swap
swapon -s
```

### WordPress Setup (if enabled)
1. Visit `http://your-ip/wp-admin/install.php`
2. Complete WordPress installation
3. Enable Nginx vhost: Set `configure_nginx_vhost: true` in vars/default.yml
4. Install SSL: Enable `install_certbot: true`

## Need Help?

- Read full README.md for detailed documentation
- Check ansible.log for playbook output
- Use `--check` flag for dry runs
- Use `--tags` for selective execution
