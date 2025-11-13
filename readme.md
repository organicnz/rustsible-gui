# 🖥️ Ubuntu Server Ansible Playbook

<div align="center">

**🚀 Automated Ubuntu Server Provisioning with Beautiful Interfaces**

[![Ansible](https://img.shields.io/badge/Ansible-2.9+-red.svg)](https://www.ansible.com/)
[![Ubuntu](https://img.shields.io/badge/Ubuntu-20.04%20|%2022.04%20|%2024.04-orange.svg)](https://ubuntu.com/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

*A comprehensive, modular Ansible playbook for Ubuntu server configuration with security hardening, optional LEMP stack, Docker, development tools, and automated system optimization.*

**✨ Features Three Beautiful Interfaces: GUI, CLI, and Direct Playbook**

</div>

---

## ✨ Features

<table>
<tr>
<td width="50%">

### 🎯 Core Features
- ✅ **Base System Setup**
  User management, SSH keys, essential packages
- ✅ **Timezone Configuration**
  Auto-configured (default: America/Los_Angeles)
- ✅ **Security Hardening**
  SSH hardening, UFW firewall, fail2ban
- ✅ **Intelligent Swap**
  Auto-sizes based on available RAM
- ✅ **Automated Maintenance**
  Unattended security upgrades, system updates

</td>
<td width="50%">

### 🔧 Optional Features
- 🐳 **Docker**
  Complete Docker & Docker Compose
- 🌐 **LEMP Stack**
  Nginx, MySQL/MariaDB, PHP-FPM
- 📝 **WordPress**
  Production-ready CMS deployment
- 🔒 **SSL/TLS**
  Let's Encrypt via Certbot
- 💻 **Development Tools**
  Neovim, Node.js 22 LTS, Claude Code
- ⏰ **Cron Jobs**
  Automated maintenance & cleanup
- 🛡️ **Advanced Security**
  14 features in 4 clusters

</td>
</tr>
</table>

## Quick Start

### Prerequisites

1. **Ansible installed** on your local machine:
   ```bash
   # macOS
   brew install ansible

   # Ubuntu/Debian
   sudo apt update && sudo apt install ansible
   ```

2. **SSH access** to your Ubuntu server with a user that has sudo privileges

3. **SSH key** configured on the server

### Beautified Interactive Experience

This playbook features a modern, beautified interactive experience with:
- Color-coded output with visual indicators (✓, ✗, ●, ○)
- Professional box-drawing characters for headers
- Clear feature selection summary
- Real-time status updates during provisioning

See `docs/BEAUTIFIED_PROMPTS_GUIDE.md` for details on the beautification enhancements.

### Installation

1. Clone this repository:
   ```bash
   git clone <your-repo-url>
   cd ansible-ubuntu
   ```

2. Install required Ansible roles:
   ```bash
   ansible-galaxy install oefenweb.fail2ban
   ```

3. Configure your inventory:
   ```bash
   # Edit inventory.ini with your server details
   vim inventory.ini
   ```

4. Configure variables:
   ```bash
   # Edit vars/default.yml
   vim vars/default.yml
   ```

5. Set up vault for sensitive data (optional but recommended):
   ```bash
   # Create encrypted vault file
   ansible-vault create vars/vault.yml
   ```

6. Run the playbook:
   ```bash
   # Interactive mode with beautified prompts
   ansible-playbook playbook.yml

   # See demo of beautified output
   bash docs/demo_beautified_output.sh

   # Dry run first (check mode)
   ansible-playbook playbook.yml --check

   # With vault password
   ansible-playbook playbook.yml --ask-vault-pass
   ```

   **Note**: The playbook will interactively prompt for connection info and feature selection with a beautified, color-coded interface.

### Three Ways to Run the Provisioning

This playbook offers three different interfaces to suit your preference:

#### 1. Direct Playbook (Basic Interactive)
Standard Ansible with beautified color-coded prompts:
```bash
ansible-playbook playbook.yml
```

#### 2. CLI with Checkbox Selection (Advanced)
Interactive terminal interface with checkbox selection:
```bash
# Quick launcher (auto-installs dependencies)
./RUN_ME.sh

# Or run directly
python3 run_interactive.py
```

Features:
- Automatic virtual environment setup
- Preview of all questions before answering
- Checkbox selection for features
- Security clusters (4 groups: System Hardening, Monitoring, Network Security, Advanced Protection)
- Validation (WordPress requires LEMP)
- Color-coded summary with feature breakdowns

#### 3. GUI Interface (Visual)
Full graphical interface with tkinter:
```bash
# Quick launcher
./RUN_GUI.sh

# Or run directly
python3 run_gui.py
```

Features:
- All options visible on one screen
- Real GUI checkboxes
- Auto-save settings (cached between sessions)
- Scrollable interface
- Connection info fields (IP, username, SSH key)
- 8 core features + 4 security clusters
- Live terminal output window during provisioning
- Visual progress tracking
- Modern, beautiful interface with hover effects

**Recommended**: Use the GUI (`./RUN_GUI.sh`) for the best experience, especially when selecting from many options.

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
<td><strong>Visual Interface</strong></td>
<td align="center">✅ Beautiful</td>
<td align="center">✅ Checkboxes</td>
<td align="center">❌ Text only</td>
</tr>
<tr>
<td><strong>All Options Visible</strong></td>
<td align="center">✅ Yes</td>
<td align="center">⚠️ Sequential</td>
<td align="center">❌ No</td>
</tr>
<tr>
<td><strong>Auto-Save Settings</strong></td>
<td align="center">✅ Yes</td>
<td align="center">❌ No</td>
<td align="center">❌ No</td>
</tr>
<tr>
<td><strong>Live Output Window</strong></td>
<td align="center">✅ Dark theme</td>
<td align="center">✅ Terminal</td>
<td align="center">✅ Terminal</td>
</tr>
<tr>
<td><strong>Setup Required</strong></td>
<td align="center">⚡ None</td>
<td align="center">⚡ Auto-install</td>
<td align="center">⚡ None</td>
</tr>
<tr>
<td><strong>Best For</strong></td>
<td align="center">👍 Everyone</td>
<td align="center">🔧 Power users</td>
<td align="center">🚀 Quick runs</td>
</tr>
</tbody>
</table>

---

## Configuration

### Inventory Setup

Edit `inventory.ini`:

```ini
[webservers]
152.53.136.84 ansible_user=root ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab

[all:vars]
ansible_python_interpreter=/usr/bin/python3
```

### Variable Configuration

All configuration is done in `vars/default.yml`:

#### Server Configuration
```yaml
ip_address: "152.53.136.84"
added_user: organic
system_timezone: America/Los_Angeles
```

#### Security Settings
```yaml
ssh_port: 22
disable_root_login: true      # IMPORTANT: Disable root login
password_authentication: false # IMPORTANT: Use SSH keys only

# Fail2ban settings
fail2ban_bantime: 86400   # 24 hours
fail2ban_findtime: 600    # 10 minutes
fail2ban_maxretry: 3
```

#### Feature Toggles
```yaml
# Docker
install_docker: true

# LEMP Stack (Nginx, MySQL, PHP)
install_lemp: false  # Set to true to install

# Swap Configuration
enable_swap: true

# Automated Cron Jobs
enable_cron_jobs: true
enable_unattended_upgrades: true
enable_auto_updates: true
enable_periodic_reboot: false  # WARNING: Reboots every 6 hours!

# Development Tools
install_dev_tools: false  # Set to true for Neovim, Node.js, etc.
install_neovim: true
install_nodejs: true
install_claude_code: false
nodejs_version: "22"
```

## Usage Examples

### Run Complete Setup

**Interactive mode** (will prompt for server IP):
```bash
ansible-playbook playbook.yml
```

**Non-interactive mode** (provide IP via command line):
```bash
ansible-playbook playbook.yml -e "ip_address=152.53.136.84"
```

### Run Specific Roles Only

```bash
# Only base setup
ansible-playbook playbook.yml --tags base

# Only security hardening
ansible-playbook playbook.yml --tags security

# Only Docker installation
ansible-playbook playbook.yml --tags docker

# Only LEMP stack
ansible-playbook playbook.yml --tags lemp

# Only swap configuration
ansible-playbook playbook.yml --tags swap

# Only development tools
ansible-playbook playbook.yml --tags dev

# Only WordPress deployment
ansible-playbook playbook.yml --tags wordpress

# Only SSL certificate installation
ansible-playbook playbook.yml --tags certbot

# Complete LEMP + WordPress + SSL stack
ansible-playbook playbook.yml --tags "lemp,wordpress,certbot"

# Multiple tags
ansible-playbook playbook.yml --tags "base,security,docker"
```

### Skip Specific Roles

```bash
# Skip Docker installation
ansible-playbook playbook.yml --skip-tags docker

# Skip LEMP stack
ansible-playbook playbook.yml --skip-tags lemp
```

### Dry Run (Check Mode)

```bash
# Test what would change without making changes
ansible-playbook playbook.yml --check

# See differences
ansible-playbook playbook.yml --check --diff
```

### With Vault Password

```bash
# Interactive password prompt
ansible-playbook playbook.yml --ask-vault-pass

# Password file
ansible-playbook playbook.yml --vault-password-file ~/.vault_pass.txt
```

## Role Documentation

### Base Setup (`base_setup`)
- Creates user accounts with sudo privileges
- Configures SSH keys
- Sets system timezone
- Installs essential packages
- Upgrades all packages

**Tags**: `base`, `setup`, `system`, `timezone`

### Swap Configuration (`swap`)
- Automatically calculates optimal swap size based on RAM:
  - <2GB RAM: 2x RAM
  - 2-4GB RAM: 1.5x RAM
  - 4-8GB RAM: 1x RAM
  - 8GB+ RAM: 1x RAM
- Optimizes swappiness and cache pressure

**Tags**: `swap`, `system`

**Variables**:
```yaml
enable_swap: true
```

### Security (`security`)
- SSH hardening (disable root, disable passwords)
- UFW firewall configuration
- Fail2ban protection
- System hardening (password policies, permissions)

**Tags**: `security`, `ssh`, `firewall`, `fail2ban`

**Variables**:
```yaml
disable_root_login: true
password_authentication: false
ssh_port: 22
```

### Docker (`docker`)
- Installs Docker Engine
- Installs Docker Compose
- Adds users to docker group
- Starts and enables Docker service

**Tags**: `docker`

**Variables**:
```yaml
install_docker: true
docker_users:
  - organic
```

### LEMP Stack (`lemp`)
- Nginx web server
- MySQL/MariaDB database
- PHP-FPM with common modules
- Automatic service startup

**Tags**: `lemp`, `web`, `nginx`, `mysql`, `php`

**Variables**:
```yaml
install_lemp: false  # Set to true
php_modules:
  - php-fpm
  - php-mysql
  - php-curl
  - php-gd
  # ... more modules
```

### Cron Jobs (`cron_jobs`)
- Automated security updates every 48 hours
- System cleanup and maintenance
- Optional periodic reboots (disabled by default)

**Tags**: `cron`, `automation`

**Variables**:
```yaml
enable_cron_jobs: true
enable_unattended_upgrades: true
enable_auto_updates: true
enable_periodic_reboot: false  # WARNING!
```

### Development Tools (`dev_tools`)
- Neovim with kickstart.nvim configuration
- Node.js 22 LTS
- Optional Claude Code CLI

**Tags**: `dev`, `development`, `neovim`, `nodejs`, `claude`

**Variables**:
```yaml
install_dev_tools: false  # Set to true
install_neovim: true
install_nodejs: true
install_claude_code: false
nodejs_version: "22"
```

### WordPress (`wordpress`)
- Production-ready WordPress installation
- Automated database and user creation
- Security-hardened file permissions (750/640)
- Auto-generated WordPress security salts
- Optimized wp-config.php configuration
- SEO-optimized robots.txt
- PHP optimization with .user.ini
- Clean URL configuration

**Tags**: `wordpress`, `web`, `cms`, `wordpress-db`, `wordpress-config`

**Requirements**:
- LEMP stack must be installed first (`install_lemp: true`)
- MySQL root credentials in vault
- Domain name configured

**Variables**:
```yaml
install_wordpress: false      # Set to true to deploy WordPress
configure_nginx_vhost: false  # Set to true for Nginx virtual host
full_domain: example.com      # Your domain name

# In vault.yml:
mysql_db: wordpress_db
mysql_user: wordpress_user
mysql_password: secure_password
mysql_root_password: secure_root_password
```

**Usage**:
```bash
# Full WordPress stack deployment
ansible-playbook playbook.yml -e "install_lemp=true install_wordpress=true configure_nginx_vhost=true"

# Only WordPress deployment (LEMP already installed)
ansible-playbook playbook.yml --tags wordpress

# Fix WordPress permissions only
ansible-playbook playbook.yml --tags wordpress-perms
```

**After deployment**:
1. Configure Nginx virtual host: `configure_nginx_vhost: true`
2. Install SSL certificate: `install_certbot: true`
3. Visit `http://your-domain.com/wp-admin/install.php` to complete setup

### Certbot SSL/TLS (`certbot`)
- Automated Let's Encrypt certificate installation
- Nginx plugin for seamless integration
- Automatic HTTPS redirect
- Auto-renewal with systemd timer (twice daily)
- Multi-domain support (primary + www)

**Tags**: `certbot`, `ssl`, `security`, `certbot-install`, `certbot-cert`

**Requirements**:
- Nginx must be installed and running
- Domain DNS must point to server IP
- Ports 80 and 443 must be accessible
- Valid email address required

**Variables**:
```yaml
install_certbot: false                    # Set to true to install SSL
certbot_email: admin@example.com          # Email for Let's Encrypt notifications
certbot_www_domain: true                  # Include www subdomain
certbot_auto_renew: true                  # Enable auto-renewal (recommended)
```

**Usage**:
```bash
# Install Certbot and generate certificate
ansible-playbook playbook.yml --tags certbot

# Only install Certbot (no cert generation)
ansible-playbook playbook.yml --tags certbot-install

# Force certificate renewal
ansible-playbook playbook.yml --tags certbot-cert -e "force_renew=true"

# Check certificate status
sudo certbot certificates

# Test auto-renewal
sudo certbot renew --dry-run
```

**Important**:
- Ensure DNS points to your server before running
- Let's Encrypt has rate limits (5 certs/week per domain)
- Certificates auto-renew 30 days before expiration
- HTTPS redirect is configured automatically

## Security Best Practices

### Before First Run

1. **Generate SSH keys** if you don't have them:
   ```bash
   ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
   ```

2. **Copy SSH key to server**:
   ```bash
   ssh-copy-id -i ~/.ssh/id_rsa_gitlab.pub root@152.53.136.84
   ```

3. **Test SSH key authentication**:
   ```bash
   ssh -i ~/.ssh/id_rsa_gitlab root@152.53.136.84
   ```

### Important Security Variables

**⚠️ These defaults are set for security:**

```yaml
disable_root_login: true        # Prevents root SSH access
password_authentication: false  # Requires SSH keys only
```

**Before enabling these security features**, ensure:
1. Your SSH key is properly configured on the server
2. You can log in with the non-root user (`added_user`)
3. The non-root user has sudo privileges

**If you get locked out**, you'll need console access to recover!

### Vault Usage for Sensitive Data

Store sensitive data in `vars/vault.yml`:

```bash
# Create vault
ansible-vault create vars/vault.yml

# Edit vault
ansible-vault edit vars/vault.yml

# View vault
ansible-vault view vars/vault.yml
```

Store these in vault:
- MySQL passwords
- Database credentials
- API tokens
- GitHub/GitLab PATs

## Firewall Configuration

Default allowed ports:
- **22** (SSH) - or custom port from `ssh_port`
- **80** (HTTP)
- **443** (HTTPS)

To allow additional ports, edit `vars/default.yml`:

```yaml
ufw_allowed_ports:
  - port: "{{ ssh_port }}"
    proto: tcp
  - port: 80
    proto: tcp
  - port: 443
    proto: tcp
  - port: 8080    # Add custom port
    proto: tcp
```

## Troubleshooting

### SSH Connection Issues

**Problem**: Can't connect after running playbook

**Solution**:
1. Check if you set `disable_root_login: true` but don't have the user account set up
2. Verify SSH key is correct in inventory
3. Try connecting with verbose mode: `ssh -vvv user@host`

### Apt Lock Issues

**Problem**: "Could not get lock /var/lib/dpkg/lock"

**Solution**:
```bash
# Wait for other package managers to finish, or:
sudo rm /var/lib/apt/lists/lock
sudo rm /var/cache/apt/archives/lock
sudo rm /var/lib/dpkg/lock*
sudo dpkg --configure -a
```

### Fail2ban Not Starting

**Problem**: Fail2ban service fails to start

**Solution**:
```bash
# Check logs
sudo journalctl -u fail2ban -n 50

# Test configuration
sudo fail2ban-client -t

# Restart service
sudo systemctl restart fail2ban
```

### Docker Permission Denied

**Problem**: "Permission denied" when running docker commands

**Solution**:
```bash
# Log out and log back in for group changes to take effect
# Or manually reload:
newgrp docker
```

### Swap Not Created

**Problem**: Swap file not created or wrong size

**Solution**:
```bash
# Check available disk space
df -h

# Manually check swap
sudo swapon -s

# Re-run swap configuration
ansible-playbook playbook.yml --tags swap
```

## File Structure

```
ansible-ubuntu/
├── playbook.yml              # Main playbook
├── inventory.ini             # Server inventory
├── ansible.cfg              # Ansible configuration
├── README.md                # This file
├── vars/
│   ├── default.yml          # Main variables
│   └── vault.yml            # Encrypted sensitive data (create this)
└── roles/
    ├── base_setup/          # Base system configuration
    ├── security/            # Security hardening
    ├── docker/              # Docker installation
    ├── swap/                # Swap configuration
    ├── lemp/                # LEMP stack
    ├── cron_jobs/           # Automated tasks
    ├── dev_tools/           # Development tools
    └── oefenweb.fail2ban/   # External fail2ban role
```

## Available Tags

| Tag | Description |
|-----|-------------|
| `base` | Base system setup |
| `setup` | Base system setup |
| `system` | System configuration |
| `timezone` | Timezone configuration |
| `swap` | Swap configuration |
| `security` | Security hardening |
| `ssh` | SSH configuration |
| `firewall` | UFW firewall |
| `fail2ban` | Fail2ban configuration |
| `docker` | Docker installation |
| `lemp` | LEMP stack |
| `web` | Web server components |
| `nginx` | Nginx only |
| `mysql` | MySQL only |
| `php` | PHP only |
| `cron` | Cron jobs |
| `automation` | Automated tasks |
| `dev` | Development tools |
| `development` | Development tools |
| `neovim` | Neovim only |
| `nodejs` | Node.js only |
| `claude` | Claude Code only |
| `reboot` | Reboot tasks |

## Common Workflows

### New Server Setup (Full Installation)

```bash
# 1. Configure inventory and variables
vim inventory.ini
vim vars/default.yml

# 2. Test connection
ansible all -m ping

# 3. Dry run
ansible-playbook playbook.yml --check

# 4. Run playbook
ansible-playbook playbook.yml

# 5. Reboot if required
ansible-playbook playbook.yml --tags reboot
```

### Adding LEMP Stack to Existing Server

```bash
# 1. Enable LEMP in vars/default.yml
# Set: install_lemp: true

# 2. Run only LEMP installation
ansible-playbook playbook.yml --tags lemp

# 3. Verify services
ansible all -m shell -a "systemctl status nginx mysql php*-fpm"
```

### Updating All Packages

```bash
# Run system updates only
ansible-playbook playbook.yml --tags packages
```

### Security Hardening Only

```bash
ansible-playbook playbook.yml --tags security
```

## Contributing

Feel free to submit issues or pull requests to improve this playbook.

## License

MIT License

## Author

Created and maintained for Ubuntu server deployments.

## Support

For issues, questions, or contributions, please open an issue in the repository.
