# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a comprehensive Ansible playbook repository for automated Ubuntu server provisioning with a full LEMP stack, WordPress deployment, SSL/TLS automation, Docker containers, development tools, and enterprise-grade security hardening. It uses a modular role-based architecture for complete server provisioning from bare metal to production-ready web applications.

**Key Capabilities:**
- Base system setup with security hardening and fail2ban intrusion prevention
- Docker containerization platform with Compose support
- Complete LEMP stack (Linux, Nginx, MySQL, PHP-FPM)
- Production-ready WordPress deployment with optimized configuration
- Automated SSL/TLS certificate management with Let's Encrypt
- Development tools (Neovim with kickstart, Node.js, Claude Code)
- Intelligent swap configuration based on available RAM
- Automated system maintenance with cron jobs and unattended upgrades

## Critical Configuration Files

### Inventory and Variables
- `inventory.ini`: Defines target servers under `[webservers]` group with SSH connection details
- `vars/default.yml`: Central configuration for all playbook variables (user, security settings, packages, Docker, etc.)
- `ansible.cfg`: Sets `inventory.ini` as default inventory and disables host key checking

### Variable Structure
The `vars/default.yml` file contains:
- **Server Configuration**: `ip_address`, `added_user`, `user_password`, `copy_local_key`
- **System Packages**: `sys_packages` list (curl, vim, git, ufw, htop, tmux, etc.)
- **Timezone**: `system_timezone` (default: America/Los_Angeles)
- **Security**: `ssh_port`, `disable_root_login`, `password_authentication`
- **Firewall**: `ufw_allowed_ports` (list of port/proto dicts)
- **Fail2ban**: `fail2ban_bantime`, `fail2ban_findtime`, `fail2ban_maxretry`
- **Docker**: `install_docker`, `docker_users`, `docker_compose_version`
- **LEMP Stack**: `install_lemp`, `php_modules` list
- **Swap**: `enable_swap` (auto-sizes based on RAM)
- **Cron Jobs**: `enable_cron_jobs`, `enable_unattended_upgrades`, `enable_auto_updates`, `enable_periodic_reboot`
- **Development Tools**: `install_dev_tools`, `install_neovim`, `install_nodejs`, `install_claude_code`, `nodejs_version`
- **Monitoring**: `install_monitoring` (future feature)
- **WordPress**: `install_wordpress`, `configure_nginx_vhost`, `full_domain`, `http_host`, MySQL credentials
- **Certbot SSL/TLS**: `install_certbot`, `certbot_email`, `certbot_www_domain`, `certbot_auto_renew`

## Role Architecture

The playbook uses 10 roles executed in specific order:

### Core Roles (Always Execute)
1. **base_setup**: User creation, sudo configuration, package installation, timezone setup
2. **swap**: Intelligent swap file creation with automatic sizing based on RAM
3. **security**: SSH hardening, UFW firewall, system hardening, optional DDOS protection
4. **oefenweb.fail2ban**: External role for fail2ban intrusion prevention (community role)

### Optional Roles (Conditional Execution)
5. **docker**: Docker CE and Docker Compose installation with security configurations
   - Conditional: `when: install_docker | bool`

6. **lemp**: Nginx, MySQL/MariaDB, PHP-FPM with security hardening
   - Conditional: `when: install_lemp | default(false) | bool`
   - Installs complete web server stack with PHP modules

7. **cron_jobs**: Automated system maintenance and security updates
   - Conditional: `when: enable_cron_jobs | default(true) | bool`
   - Includes unattended upgrades, system cleanup, optional periodic reboots

8. **dev_tools**: Development environment (Neovim, Node.js, Claude Code)
   - Conditional: `when: install_dev_tools | default(false) | bool`
   - Configurable Node.js version, optional Claude Code CLI

9. **wordpress**: Production-ready WordPress deployment with database setup
   - Conditional: `when: install_wordpress | default(false) | bool`
   - Requires LEMP stack to be installed first
   - Creates database, configures wp-config.php, sets security permissions

10. **certbot**: Automated SSL/TLS certificates with Let's Encrypt
    - Conditional: `when: install_certbot | default(false) | bool`
    - Requires Nginx to be installed
    - Includes automatic certificate renewal

### Role Dependencies
- **security** depends on **base_setup** (user must exist before SSH hardening)
- **wordpress** depends on **lemp** (requires MySQL and Nginx)
- **certbot** depends on **lemp** or Nginx (requires web server for certificate validation)
- **swap** executes early to ensure adequate memory for subsequent operations
- All roles use variables from `vars/default.yml` for consistent configuration

### Pre-tasks and Post-tasks
- **Pre-tasks**: Clean Docker repository conflicts, update apt cache, ensure fail2ban is installed
- **Post-tasks**: Check for required reboots (checks `/var/run/reboot-required`), reboot is tagged with `never` and `reboot`

## Common Commands

### Running the Playbook

Initial setup (as root):
```bash
ansible-playbook playbook.yml -l webservers -i inventory.ini -u root -k
```

Subsequent runs (as non-root user):
```bash
ansible-playbook playbook.yml -l webservers -i inventory.ini -u organic --ask-become-pass
```

### Tag-based Execution

Run only security tasks:
```bash
ansible-playbook playbook.yml -t security -l webservers -i inventory.ini -u organic --ask-become-pass
```

Run only Docker installation:
```bash
ansible-playbook playbook.yml -t docker -l webservers -i inventory.ini -u organic --ask-become-pass
```

Available tags: `base`, `setup`, `security`, `ssh`, `firewall`, `fail2ban`, `docker`, `lemp`, `web`, `nginx`, `mysql`, `php`, `swap`, `cron`, `automation`, `dev`, `development`, `neovim`, `nodejs`, `claude`, `wordpress`, `cms`, `certbot`, `ssl`, `packages`, `user`, `system`, `reboot`

### Testing and Validation

Syntax check:
```bash
ansible-playbook playbook.yml --syntax-check
```

Dry run (check mode):
```bash
ansible-playbook playbook.yml -l webservers -i inventory.ini -u organic --ask-become-pass --check
```

Lint playbook (requires ansible-lint):
```bash
ansible-lint playbook.yml
```

### SSH Troubleshooting

Add SSH keys to agent:
```bash
eval 'ssh-agent' && ssh-add ~/.ssh/azure_id_rsa.pem
eval 'ssh-agent' && ssh-add ~/.ssh/id_rsa_gitlab
```

## Architecture Patterns

### Idempotency
All tasks are designed to be idempotent. The playbook can be run multiple times without side effects.

### Error Handling
- Docker role uses `block:`/`rescue:` pattern for installation failures
- Security role uses `rescue:` for system hardening failures
- Failed tasks display helpful debug messages

### Handlers
Handlers are used for service restarts to avoid unnecessary service disruptions:
- SSH configuration changes trigger `restart ssh` handler
- Fail2ban changes trigger `restart fail2ban` handler
- Docker daemon config changes trigger `restart docker` handler

### Repository Conflict Resolution
The playbook includes pre-tasks and tasks to handle Docker repository conflicts:
- Cleans existing Docker GPG keys from multiple locations
- Removes old repository files
- Uses `signed-by` option in apt repository configuration
- Places GPG key in `/etc/apt/keyrings/docker.asc`

### Security Hardening
Security role implements:
- SSH hardening: Disables password auth and root login based on variables
- UFW firewall: Default deny with explicit allow rules
- Password policies: PAM configuration for strong passwords
- Proper file permissions on cron directories
- Docker daemon security: Disables userland proxy, enables no-new-privileges

### User Management Pattern
Creates non-root user with:
- SSH key-based authentication
- Passwordless sudo via wheel group
- Addition to Docker group (if Docker is installed)
- Password hash stored securely

## Development Guidelines

### Ansible Best Practices (from .cursor/rules)
- Follow idempotent design principles
- Use `group_vars` and `host_vars` for environment-specific configs
- Validate with `ansible-lint` before running
- Use handlers for service restarts
- Apply variables securely with Ansible Vault for sensitive data
- Use `block:` and `rescue:` for error handling
- Implement tags for flexible execution
- Use Jinja2 templates for dynamic configurations

### Variable Management
- All configurable settings should be in `vars/default.yml`
- Sensitive data (passwords, keys) should use `lookup()` from local files
- Use descriptive variable names following snake_case convention
- Document variable purpose in comments

### Naming Conventions
- Use snake_case for: variables, file names, directory structures
- Use descriptive task names starting with action verbs
- Tag tasks appropriately for selective execution

### Testing Changes
Always test changes in this order:
1. Syntax check: `ansible-playbook playbook.yml --syntax-check`
2. Dry run: `--check` mode
3. Limited scope: Use tags to run only modified sections
4. Full run on test server before production

## Python Dependencies

The playbook requires Python 3 on target hosts:
- `ansible_python_interpreter=/usr/bin/python3` set in inventory
- Docker role installs `python3-pip` and `python3-docker`
- Uses `--break-system-packages` flag for Ubuntu 24.04+ when installing Docker Python module

## Reboot Handling

The playbook checks for `/var/run/reboot-required` and can reboot if needed:
- Reboot task is tagged with `never` and `reboot`
- To allow reboots, add `-t reboot` to the ansible-playbook command
- Reboot timeout: 300 seconds, post-reboot delay: 30 seconds

## New Consolidated Roles (v2.1.0)

### LEMP Stack Role (`lemp`)
Provides complete web server infrastructure with Nginx, MySQL/MariaDB, and PHP-FPM.

**Key Features:**
- Nginx with security-hardened default configuration
- MySQL/MariaDB with secure initial setup
- PHP-FPM with common modules (mysql, curl, gd, intl, mbstring, soap, xml, zip)
- Virtual host configuration via templates
- Service management with handlers

**Configuration Variables:**
- `install_lemp`: Enable/disable LEMP stack installation
- `php_modules`: List of PHP modules to install
- `full_domain`: Domain name for virtual host configuration
- MySQL credentials stored in `vars/vault.yml`

**Templates:**
- `wordpress-nginx.conf.j2`: Nginx virtual host for WordPress
- `.my.cnf.j2`: MySQL client configuration

### Swap Configuration Role (`swap`)
Automatically configures optimal swap space based on available system RAM.

**Intelligent Sizing:**
- RAM < 2GB: Swap = 2x RAM
- RAM 2-4GB: Swap = 1.5x RAM
- RAM 4-8GB: Swap = 1x RAM
- RAM > 8GB: Swap = 1x RAM

**Features:**
- Creates `/swapfile` with appropriate permissions (600)
- Configures swappiness and cache pressure for optimal performance
- Persistent configuration via `/etc/fstab`
- Idempotent execution (checks existing swap before creating)

### WordPress Role (`wordpress`)
Production-ready WordPress deployment with security-hardened configuration.

**Features:**
- Downloads latest WordPress core
- Creates MySQL database and dedicated user
- Generates wp-config.php with unique security salts
- Sets security-hardened file permissions (750 for directories, 640 for files)
- Nginx virtual host configuration (when `configure_nginx_vhost: true`)
- SEO-optimized robots.txt
- PHP performance optimization with .user.ini

**Configuration Variables:**
- `install_wordpress`: Enable WordPress deployment
- `configure_nginx_vhost`: Create Nginx virtual host
- `full_domain`: Primary domain name
- `http_host`: HTTP host (usually same as full_domain)
- MySQL credentials: `mysql_db`, `mysql_user`, `mysql_password`, `mysql_root_password`

**Templates:**
- `wp-config.php.j2`: WordPress configuration with security salts
- Nginx virtual host shared with LEMP role

**Requirements:**
- LEMP stack must be installed first (`install_lemp: true`)
- MySQL credentials must be provided (preferably via Ansible Vault)
- Domain DNS must point to server IP

### Certbot SSL/TLS Role (`certbot`)
Automated Let's Encrypt SSL/TLS certificate management with auto-renewal.

**Features:**
- Installs Certbot with Nginx plugin
- Obtains SSL/TLS certificates for domain and www subdomain
- Configures automatic HTTPS redirect in Nginx
- Sets up systemd timer for automatic renewal (twice daily)
- Email notifications for certificate expiration

**Configuration Variables:**
- `install_certbot`: Enable Certbot installation and certificate generation
- `certbot_email`: Email for Let's Encrypt notifications
- `certbot_www_domain`: Include www subdomain in certificate
- `certbot_auto_renew`: Enable automatic renewal (recommended: true)
- `full_domain`: Primary domain for certificate

**Certificate Renewal:**
- Systemd timer runs twice daily
- Certificates auto-renew 30 days before expiration
- Nginx automatically reloads after successful renewal

**Requirements:**
- Nginx must be installed and running
- Domain DNS must point to server IP
- Ports 80 and 443 must be accessible from the internet
- Valid email address required for Let's Encrypt notifications

**Rate Limits:**
- Let's Encrypt allows 5 certificates per week per domain
- Failed validation attempts count against rate limits

### Cron Jobs Role (`cron_jobs`)
Automated system maintenance and security update management.

**Features:**
- Unattended security upgrades (configurable)
- Automatic package updates (configurable)
- System cleanup and log rotation
- Optional periodic system reboots (DANGEROUS - disabled by default)

**Configuration Variables:**
- `enable_cron_jobs`: Enable/disable cron job management
- `enable_unattended_upgrades`: Enable automatic security updates
- `enable_auto_updates`: Enable automatic package updates
- `enable_periodic_reboot`: **WARNING**: Reboots server every 6 hours if enabled!

**Cron Schedule:**
- Security updates: Every 48 hours
- System cleanup: Daily
- Periodic reboot: Every 6 hours (if enabled - NOT RECOMMENDED for production)

**Safety Notes:**
- `enable_periodic_reboot` should almost always be `false`
- Automatic reboots can cause service disruption
- Use only in development or testing environments

### Development Tools Role (`dev_tools`)
Installs modern development environment for server-side development.

**Tools Installed:**
- **Neovim**: Latest version with kickstart.nvim configuration
- **Node.js**: Configurable version via NodeSource (default: v22 LTS)
- **npm**: Included with Node.js
- **Claude Code CLI**: Optional Anthropic AI coding assistant
- **Build essentials**: Compilers and build tools

**Configuration Variables:**
- `install_dev_tools`: Enable/disable development tools installation
- `install_neovim`: Include Neovim in installation
- `install_nodejs`: Include Node.js in installation
- `install_claude_code`: Include Claude Code CLI (optional)
- `nodejs_version`: Node.js major version (default: "22")

**Neovim Configuration:**
- Installs kickstart.nvim for sensible defaults
- LSP, Treesitter, and Telescope pre-configured
- Optimized for Lua and TypeScript/JavaScript development

**Use Cases:**
- Server-side Node.js development
- Infrastructure as Code editing
- Remote development workflows
- DevOps automation scripting

## Common Workflow Examples

### Complete WordPress Stack Deployment
```bash
# 1. Configure variables in vars/default.yml:
#    install_lemp: true
#    install_wordpress: true
#    configure_nginx_vhost: true
#    install_certbot: true
#    full_domain: example.com
#    certbot_email: admin@example.com

# 2. Create MySQL credentials in vars/vault.yml:
ansible-vault create vars/vault.yml
# Add: mysql_root_password, mysql_user, mysql_password, mysql_db

# 3. Deploy complete stack:
ansible-playbook playbook.yml --ask-vault-pass

# 4. Complete WordPress installation:
# Visit: https://example.com/wp-admin/install.php
```

### Development Server Setup
```bash
# Configure for development:
# install_docker: true
# install_dev_tools: true
# install_neovim: true
# install_nodejs: true
# nodejs_version: "22"

# Deploy development environment:
ansible-playbook playbook.yml --tags "docker,dev"
```

### Add LEMP to Existing Server
```bash
# Set install_lemp: true in vars/default.yml

# Install only LEMP stack:
ansible-playbook playbook.yml --tags lemp

# Verify services:
ansible all -m shell -a "systemctl status nginx mysql php*-fpm"
```

### SSL Certificate Installation
```bash
# Prerequisites:
# - LEMP stack installed
# - Domain DNS pointing to server
# - Ports 80 and 443 accessible

# Set in vars/default.yml:
# install_certbot: true
# certbot_email: admin@example.com
# full_domain: example.com

# Install SSL certificate:
ansible-playbook playbook.yml --tags certbot

# Verify certificate:
ssh user@server "sudo certbot certificates"
```

## Documentation Resources

This repository includes comprehensive documentation:

- **CLAUDE.md** (this file): Technical reference for Claude Code AI assistant
- **README.md**: User-facing documentation with quick start guide
- **FEATURES.md**: Complete feature catalog with status and capabilities
- **ADR.md**: Architecture Decision Records documenting key design choices
- **WORKFLOWS.md**: Step-by-step workflow guides for common scenarios
- **PRD.md**: Product Requirements Document with project vision

## Version History

**v2.1.0** (2025-11-12):
- Consolidated functionality from ansible-stack-ubuntu
- Added 6 new roles: swap, lemp, wordpress, certbot, cron_jobs, dev_tools
- Enhanced documentation with FEATURES.md, ADR.md, WORKFLOWS.md
- Expanded vars/default.yml with comprehensive configuration options
- Improved tag granularity for surgical task execution
- Ubuntu 24.04 LTS full support

**v2.0.0** (2024-10-15):
- Docker repository conflict resolution
- Block/rescue error handling
- Modern GPG key management
- Python 3 standardization

**v1.0.0** (2024-09-01):
- Initial stable release
- Core roles: base_setup, security, docker
- Fail2ban integration
- Tag-based execution
