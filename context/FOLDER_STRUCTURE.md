# Ansible Ubuntu Repository - Folder Structure

**Repository**: ansible-ubuntu
**Version**: 2.1.0
**Last Updated**: 2025-11-12
**Architecture**: Modular role-based Ansible playbook

---

## Repository Tree

```
ansible-ubuntu/
├── .cursor/                          # Cursor IDE configuration
│   └── rules/
│       └── ansible-rules.mdc        # Ansible coding standards for Cursor
│
├── context/                          # Project context and documentation
│   ├── ADR.md                       # Architecture Decision Records (795 lines)
│   ├── FEATURES.md                  # Complete feature catalog (503 lines)
│   ├── FOLDER_STRUCTURE.md          # This file - repository structure reference
│   ├── PRD.md                       # Product Requirements Document (767 lines)
│   ├── WORKFLOWS.md                 # Common workflow guides (1,292 lines)
│   └── ultrathink.md                # Detailed analysis and planning (1,183 lines)
│
├── docs/                             # Technical documentation
│   ├── COMPLETION_SUMMARY.md        # Implementation completion summary (414 lines)
│   ├── PLAYBOOK_ENHANCEMENTS.md     # Playbook enhancement details (526 lines)
│   └── TAG_REFERENCE.md             # Comprehensive tag documentation (326 lines)
│
├── roles/                            # Ansible roles (modular components)
│   │
│   ├── base_setup/                  # Core system setup role
│   │   ├── defaults/                # Default variables
│   │   ├── files/                   # Static files
│   │   ├── handlers/                # Service handlers
│   │   ├── meta/                    # Role metadata
│   │   ├── tasks/                   # Role tasks
│   │   │   └── main.yml            # User creation, packages, timezone
│   │   ├── templates/               # Jinja2 templates
│   │   └── vars/                    # Role-specific variables
│   │
│   ├── certbot/                     # SSL/TLS certificate automation
│   │   ├── defaults/
│   │   │   └── main.yml            # Certbot configuration defaults
│   │   └── tasks/
│   │       └── main.yml            # Let's Encrypt certificate management
│   │
│   ├── cron_jobs/                   # Automated system maintenance
│   │   ├── defaults/
│   │   │   └── main.yml            # Cron job configuration
│   │   └── tasks/
│   │       └── main.yml            # Unattended upgrades, cleanup
│   │
│   ├── dev_tools/                   # Development environment setup
│   │   ├── defaults/
│   │   │   └── main.yml            # Development tool defaults
│   │   └── tasks/
│   │       └── main.yml            # Neovim, Node.js, Claude Code
│   │
│   ├── docker/                      # Docker containerization platform
│   │   ├── defaults/                # Docker default variables
│   │   ├── files/                   # Docker-related files
│   │   ├── handlers/
│   │   │   └── main.yml            # Docker service restart handlers
│   │   ├── meta/                    # Docker role metadata
│   │   ├── tasks/
│   │   │   └── main.yml            # Docker CE & Compose installation
│   │   ├── templates/               # Docker daemon.json templates
│   │   └── vars/                    # Docker role variables
│   │
│   ├── lemp/                        # LEMP stack (Nginx, MySQL, PHP)
│   │   ├── defaults/
│   │   │   └── main.yml            # LEMP stack defaults
│   │   ├── handlers/
│   │   │   └── main.yml            # Nginx, MySQL, PHP-FPM handlers
│   │   ├── tasks/
│   │   │   └── main.yml            # Web server stack installation
│   │   └── templates/
│   │       ├── .my.cnf.j2          # MySQL client configuration
│   │       └── wordpress-nginx.conf.j2  # Nginx virtual host template
│   │
│   ├── oefenweb.fail2ban/           # External fail2ban role (community)
│   │   ├── .github/                 # GitHub workflows
│   │   ├── defaults/                # Fail2ban default configuration
│   │   ├── files/                   # Static configuration files
│   │   ├── handlers/                # Fail2ban service handlers
│   │   ├── meta/                    # Role metadata and dependencies
│   │   ├── molecule/                # Testing framework
│   │   ├── tasks/                   # Fail2ban installation tasks
│   │   ├── templates/               # Jail configuration templates
│   │   ├── tests/                   # Role tests
│   │   ├── vars/                    # OS-specific variables
│   │   ├── .ansible-lint            # Linting configuration
│   │   ├── .gitignore              # Git ignore patterns
│   │   ├── .yamllint               # YAML linting rules
│   │   ├── Dockerfile              # Testing container
│   │   ├── LICENSE.txt             # MIT License
│   │   ├── README.md               # Role documentation
│   │   ├── Vagrantfile             # Vagrant testing environment
│   │   └── requirements.yml        # Role dependencies
│   │
│   ├── security/                    # Security hardening role
│   │   ├── defaults/                # Security default settings
│   │   ├── files/                   # Security-related files
│   │   ├── handlers/
│   │   │   └── main.yml            # SSH, fail2ban handlers
│   │   ├── meta/                    # Security role metadata
│   │   ├── tasks/
│   │   │   └── main.yml            # SSH hardening, UFW, PAM
│   │   ├── templates/               # Security configuration templates
│   │   └── vars/                    # Security role variables
│   │
│   ├── swap/                        # Intelligent swap configuration
│   │   ├── defaults/                # Swap defaults (empty - uses logic)
│   │   └── tasks/
│   │       └── main.yml            # Auto-sized swap file creation
│   │
│   └── wordpress/                   # WordPress CMS deployment
│       ├── defaults/
│       │   └── main.yml            # WordPress configuration defaults
│       ├── tasks/
│       │   └── main.yml            # WP installation & database setup
│       └── templates/
│           ├── .htaccess            # Apache compatibility rules
│           ├── .user.ini            # PHP optimization settings
│           ├── robots.txt           # SEO-optimized crawl rules
│           └── wp-config.php.j2     # WordPress configuration template
│
├── scripts/                          # Utility scripts
│   └── setup-vault.sh               # Ansible Vault setup helper
│
├── vars/                             # Playbook-level variables
│   ├── default.yml                  # Central configuration (104 lines)
│   └── vault.yml.template           # Encrypted secrets template
│
├── .gitignore                        # Git ignore patterns
├── CLAUDE.md                         # AI assistant technical reference (472 lines)
├── ansible.cfg                       # Ansible configuration
├── inventory.ini                     # Target server inventory
├── playbook.yml                      # Main playbook (191 lines)
└── readme.md                         # User documentation (673 lines)
```

---

## Directory Purpose Summary

### Configuration Files (Root Level)

| File | Purpose | Key Contents |
|------|---------|--------------|
| `ansible.cfg` | Ansible settings | Inventory path, SSH settings, privilege escalation |
| `inventory.ini` | Target servers | Host definitions, groups, connection details |
| `playbook.yml` | Main playbook | Role execution order, pre/post tasks, tags |
| `vars/default.yml` | Central config | All user-configurable variables |
| `vars/vault.yml.template` | Secrets template | MySQL passwords, API tokens (encrypted) |
| `.gitignore` | Git exclusions | IDE files, temporary files, sensitive data |

### Documentation Hierarchy

```
Documentation Layer 1: User-Facing
├── readme.md                    → Quick start, usage examples, troubleshooting

Documentation Layer 2: Technical Reference
├── CLAUDE.md                    → AI assistant guidance, architecture, commands
├── docs/TAG_REFERENCE.md        → Complete tag system documentation
└── docs/PLAYBOOK_ENHANCEMENTS.md → Technical enhancement details

Documentation Layer 3: Context & Planning
├── context/FEATURES.md          → Feature catalog with status
├── context/ADR.md               → Architecture decisions
├── context/PRD.md               → Product requirements
├── context/WORKFLOWS.md         → Step-by-step workflow guides
└── context/FOLDER_STRUCTURE.md  → This file
```

### Role Organization

#### Core Roles (Always Execute)
```
roles/base_setup/    → Foundation (user, packages, timezone)
roles/swap/          → Memory optimization (auto-sizing)
roles/security/      → Hardening (SSH, UFW, PAM)
roles/oefenweb.fail2ban/ → Intrusion prevention (external role)
```

#### Optional Roles (Conditional)
```
roles/docker/        → Container platform (when: install_docker)
roles/lemp/          → Web server stack (when: install_lemp)
roles/cron_jobs/     → Automation (when: enable_cron_jobs)
roles/dev_tools/     → Development env (when: install_dev_tools)
roles/wordpress/     → CMS deployment (when: install_wordpress)
roles/certbot/       → SSL/TLS certs (when: install_certbot)
```

---

## File Count Statistics

### By Directory Type

```
Total Directories: 64
Total Files: 26 (tracked in git)

Breakdown:
├── Configuration: 6 files
├── Documentation: 11 files
├── Roles: 10 roles
│   ├── Core roles: 4
│   ├── Optional roles: 6
│   └── Role files: Multiple (tasks, defaults, handlers, templates)
└── Scripts: 1 file
```

### By File Type

```
Ansible Files:
├── Playbooks: 1 (playbook.yml)
├── Role tasks: 10 (roles/*/tasks/main.yml)
├── Role defaults: 10 (roles/*/defaults/main.yml)
├── Role handlers: 5 (roles/*/handlers/main.yml)
├── Templates: 9 (.j2 files)
└── Variables: 2 (vars/*.yml)

Documentation:
├── Markdown: 11 (.md files)
├── Total lines: ~7,500+ (documentation only)

Configuration:
├── Ansible config: 1 (ansible.cfg)
├── Inventory: 1 (inventory.ini)
├── Git ignore: 1 (.gitignore)
└── IDE rules: 1 (.cursor/rules/ansible-rules.mdc)
```

---

## Key File Details

### Critical Configuration Files

#### `vars/default.yml` (104 lines)
**Purpose**: Central configuration for all playbook variables
**Contains**:
- Server configuration (IP, user, passwords)
- Security settings (SSH, firewall, fail2ban)
- Feature toggles (install_docker, install_lemp, etc.)
- LEMP stack configuration
- WordPress settings
- Certbot SSL/TLS configuration
- Development tools configuration
- Cron job settings

**Variable Groups**:
```yaml
# Server Configuration
ip_address, added_user, user_password, copy_local_key

# System Packages
sys_packages: [curl, vim, git, ufw, htop, tmux, zip, unzip, build-essential]

# Timezone
system_timezone: America/Los_Angeles

# Security
ssh_port, disable_root_login, password_authentication
ufw_allowed_ports, fail2ban_bantime, fail2ban_findtime, fail2ban_maxretry

# Docker
install_docker, docker_compose_version, docker_users

# LEMP Stack
install_lemp, php_modules

# Swap
enable_swap

# Cron Jobs
enable_cron_jobs, enable_unattended_upgrades, enable_auto_updates, enable_periodic_reboot

# Development Tools
install_dev_tools, install_neovim, install_nodejs, install_claude_code, nodejs_version

# WordPress
install_wordpress, configure_nginx_vhost, full_domain, http_host

# Certbot SSL/TLS
install_certbot, certbot_email, certbot_www_domain, certbot_auto_renew

# Monitoring (future)
install_monitoring
```

#### `playbook.yml` (191 lines)
**Purpose**: Main playbook orchestrating all roles
**Structure**:
```yaml
Pre-tasks:
├── Docker repository cleanup
├── Apt cache update
└── Fail2ban installation

Roles (Execution Order):
1. base_setup
2. swap
3. security
4. oefenweb.fail2ban
5. docker (conditional)
6. lemp (conditional)
7. cron_jobs (conditional)
8. dev_tools (conditional)
9. wordpress (conditional)
10. certbot (conditional)

Post-tasks:
├── Reboot requirement check
└── System reboot (tagged 'never' + 'reboot')
```

### Documentation Files

#### `CLAUDE.md` (472 lines)
Technical reference for AI assistants containing:
- Project overview and capabilities
- Critical configuration files
- Role architecture and dependencies
- Common commands and workflows
- Architecture patterns (idempotency, error handling, handlers)
- Development guidelines
- Variable management
- Testing procedures
- Reboot handling
- New consolidated roles documentation
- Common workflow examples
- Version history

#### `readme.md` (673 lines)
User-facing documentation containing:
- Quick start guide
- Prerequisites and installation
- Configuration examples
- Usage examples with tags
- Role documentation
- Security best practices
- Firewall configuration
- Troubleshooting guide
- Common workflows
- File structure overview
- Available tags reference

#### `context/FEATURES.md` (503 lines)
Complete feature catalog with:
- Feature status legend (Stable, Beta, Optional, Conditional)
- Core features (base setup, security, Docker)
- Optional features (LEMP, WordPress, Certbot, dev tools)
- Operational features (idempotency, tags, error handling)
- Platform support matrix
- Performance characteristics
- Security features summary
- Testing and validation
- Version history
- Future roadmap

#### `context/ADR.md` (795 lines)
Architecture Decision Records documenting:
- ADR-001: Role-based architecture
- ADR-002: UFW for firewall management
- ADR-003: External role for fail2ban
- ADR-004: Idempotency as core principle
- ADR-005: Docker repository conflict resolution
- ADR-006: Variable centralization
- ADR-007: Tag-based execution strategy
- ADR-008: Handlers for service management
- ADR-009: Python 3 standardization
- ADR-010: Security-first design
- ADR-011: Reboot handling strategy
- ADR-012: Error handling with block/rescue

#### `context/WORKFLOWS.md` (1,292 lines)
Step-by-step workflow guides for:
- New server provisioning
- LEMP stack deployment
- WordPress site deployment
- SSL/TLS certificate setup
- Development environment configuration
- Security hardening
- Docker deployment
- System maintenance
- Troubleshooting procedures

---

## Role Internal Structure

### Standard Role Layout

Each role follows Ansible's standard directory structure:

```
role_name/
├── defaults/          # Default variables (lowest precedence)
│   └── main.yml      # role_variable: default_value
│
├── tasks/            # Task definitions
│   └── main.yml      # Main task file (entry point)
│
├── handlers/         # Event handlers (triggered by notify)
│   └── main.yml      # Service restart handlers
│
├── templates/        # Jinja2 templates (.j2 files)
│   └── config.j2     # Dynamic configuration files
│
├── files/            # Static files
│   └── file.conf     # Files copied as-is
│
├── vars/             # Role variables (high precedence)
│   └── main.yml      # Fixed role variables
│
└── meta/             # Role metadata
    └── main.yml      # Dependencies, Galaxy info
```

### Role Complexity

| Role | Task Files | Templates | Handlers | Complexity |
|------|-----------|-----------|----------|------------|
| base_setup | 1 | Multiple | Yes | Medium |
| swap | 1 | 0 | No | Low |
| security | 1 | Multiple | Yes | High |
| oefenweb.fail2ban | Multiple | Yes | Yes | High (external) |
| docker | 1 | Yes | Yes | High |
| lemp | 1 | 2 | Yes | Medium |
| cron_jobs | 1 | 0 | No | Low |
| dev_tools | 1 | 0 | No | Medium |
| wordpress | 1 | 4 | No | Medium |
| certbot | 1 | 0 | No | Medium |

---

## Template Files

### LEMP Role Templates

**`roles/lemp/templates/.my.cnf.j2`**
```jinja2
[client]
user={{ mysql_user }}
password={{ mysql_password }}
```
Purpose: MySQL client authentication configuration

**`roles/lemp/templates/wordpress-nginx.conf.j2`**
```nginx
server {
    listen 80;
    server_name {{ full_domain }};
    root /var/www/{{ full_domain }};
    # ... WordPress-optimized Nginx configuration
}
```
Purpose: Nginx virtual host for WordPress with PHP-FPM integration

### WordPress Role Templates

**`roles/wordpress/templates/wp-config.php.j2`**
- WordPress database connection
- Security salts (auto-generated)
- WordPress debugging settings
- File permissions configuration

**`roles/wordpress/templates/.user.ini`**
- PHP memory limit
- PHP upload limits
- PHP execution time

**`roles/wordpress/templates/robots.txt`**
- SEO-optimized crawling rules
- Disallow patterns for WordPress

**`roles/wordpress/templates/.htaccess`**
- Apache compatibility (if needed)
- Rewrite rules for clean URLs

---

## Version Control

### Git Tracked Files

**Included in repository**:
- All `.yml` and `.yaml` files (playbooks, roles, variables)
- All `.md` files (documentation)
- All `.j2` files (templates)
- Configuration files (ansible.cfg, inventory.ini)
- Scripts (setup-vault.sh)

**Excluded from repository** (via `.gitignore`):
```
.DS_Store
*.pyc
__pycache__/
.venv/
venv/
.vault_pass
*.retry
.history/
.trunk/
.vscode/
.idea/
vars/vault.yml         # Encrypted secrets (use vault.yml.template)
```

---

## Scalability & Extension

### Adding New Roles

To add a new role to this structure:

1. **Create role skeleton**:
   ```bash
   mkdir -p roles/new_role/{defaults,tasks,handlers,templates,files}
   ```

2. **Add tasks**:
   ```bash
   touch roles/new_role/tasks/main.yml
   ```

3. **Add to playbook**:
   ```yaml
   - role: new_role
     tags:
       - new_role
       - custom_tag
     when: install_new_role | default(false) | bool
   ```

4. **Add variables**:
   ```yaml
   # vars/default.yml
   install_new_role: false
   new_role_setting: value
   ```

5. **Document**:
   - Update CLAUDE.md with role description
   - Update readme.md with usage instructions
   - Update context/FEATURES.md with feature details

### Current Capacity

- **Role slots**: Unlimited (modular design)
- **Variable capacity**: ~100 variables in vars/default.yml (organized in sections)
- **Documentation**: ~7,500 lines across 11 files
- **Tags**: 30+ tags (expandable)
- **Supported platforms**: Ubuntu 20.04, 22.04, 24.04 LTS

---

## Repository Metrics

### Size Statistics

```
Total Repository Size: ~15 MB (excluding .git)
├── Roles: ~8 MB (includes oefenweb.fail2ban external role)
├── Documentation: ~200 KB (11 markdown files)
├── Configuration: ~50 KB
└── Templates: ~30 KB
```

### Code Lines

```
Ansible Code (YAML):
├── Playbook: 191 lines
├── Role tasks: ~2,500 lines (estimated across all roles)
├── Variables: 104 lines (vars/default.yml)
└── Templates: ~300 lines (9 template files)

Total Ansible Code: ~3,095 lines

Documentation (Markdown):
├── User docs: 1,145 lines (readme.md + CLAUDE.md)
├── Context docs: 4,425 lines (FEATURES.md, ADR.md, PRD.md, WORKFLOWS.md)
├── Technical docs: 1,266 lines (docs/*.md)
└── Planning docs: 1,183 lines (ultrathink.md)

Total Documentation: ~8,019 lines
```

---

## Navigation Quick Reference

### Common File Locations

**Need to...**
- Configure servers? → `inventory.ini`
- Change settings? → `vars/default.yml`
- Add secrets? → `vars/vault.yml` (create from template)
- Understand architecture? → `CLAUDE.md`
- Quick start? → `readme.md`
- See all features? → `context/FEATURES.md`
- Learn workflows? → `context/WORKFLOWS.md`
- Understand decisions? → `context/ADR.md`
- Find tags? → `docs/TAG_REFERENCE.md`
- Modify roles? → `roles/[role_name]/tasks/main.yml`
- Change templates? → `roles/[role_name]/templates/`

### Command Execution

```bash
# From repository root
ansible-playbook playbook.yml                          # Run all roles
ansible-playbook playbook.yml --tags docker            # Run Docker only
ansible-playbook playbook.yml --check                  # Dry run
ansible-playbook playbook.yml --syntax-check           # Syntax validation
ansible-lint playbook.yml                              # Lint playbook
```

---

**Last Updated**: 2025-11-12
**Repository Version**: 2.1.0
**Status**: Production Ready
**Maintainer**: Infrastructure Team
