# Feature Catalog

This document provides a comprehensive catalog of all features and capabilities in the Ansible Ubuntu Server Provisioning playbook.

## Feature Status Legend

- **Stable**: Production-ready, fully tested
- **Beta**: Functional but may have edge cases
- **Optional**: Disabled by default, enable via configuration
- **Conditional**: Activated based on variables or system state

---

## Core Features

### Base System Setup

**Status**: Stable | **Role**: base_setup | **Tags**: base, setup

#### User Management
- **Non-root user creation** with sudo privileges
- **SSH key-based authentication** deployment
- **Password configuration** with SHA-512 hashing
- **Wheel group** for sudo access management
- **Passwordless sudo** for automation support

#### System Configuration
- **Timezone configuration** (default: America/Los_Angeles)
- **System availability checks** with connection timeout handling
- **Cloud-init wait** support for cloud environments (optional)

#### Package Management
- **Essential package installation** (curl, vim, git, ufw, htop, tmux, etc.)
- **Full system package upgrades** with latest security patches
- **APT cache management** with configurable validity period
- **Orphaned package cleanup** (autoremove, autoclean)
- **Ubuntu version detection** and end-of-life handling

#### Supported Ubuntu Versions
- Ubuntu 20.04 LTS (Focal Fossa)
- Ubuntu 22.04 LTS (Jammy Jellyfish)
- Ubuntu 23.10 (Mantic Minotaur)
- Ubuntu 24.04 LTS (Noble Numbat)

---

### Security Hardening

**Status**: Stable | **Role**: security | **Tags**: security, ssh, firewall, hardening

#### SSH Hardening
- **Password authentication disable** (force SSH key-only access)
- **Root login disable** (prevent direct root SSH access)
- **Custom SSH port** support (security through obscurity)
- **SSH configuration validation** before applying changes

#### Firewall (UFW)
- **Default-deny incoming policy** with explicit allow rules
- **SSH port protection** (auto-configured based on ssh_port variable)
- **Configurable port allowlist** (HTTP, HTTPS, custom applications)
- **Protocol-specific rules** (TCP/UDP per port)
- **Automatic firewall activation** with safe SSH preservation

#### Intrusion Prevention (Fail2ban)
- **Automated IP banning** for failed SSH authentication attempts
- **Configurable ban parameters**:
  - Ban time: 24 hours (default)
  - Find time: 10 minutes (default)
  - Max retry: 3 attempts (default)
- **IP whitelist support** (localhost, server IP, custom addresses)
- **SSH and SSH-DDOS jail** protection
- **Custom jail configuration** via jail.local template

#### System Hardening
- **Strong password policies** via PAM (12 char minimum, 3 char difference)
- **Cron directory permission hardening** (700 root-only)
- **File permission enforcement** for security-critical directories

#### DDOS Protection
**Status**: Optional (disabled by default)
- HTTP GET flood detection and mitigation
- Configurable request thresholds
- Web server log parsing (Apache/Nginx)

---

### Docker Container Platform

**Status**: Stable | **Role**: docker | **Tags**: docker | **Conditional**: install_docker=true

#### Docker Engine
- **Docker CE** (Community Edition) installation
- **Docker CLI** tools and commands
- **containerd.io** runtime
- **Automatic version selection** (latest stable)
- **Repository conflict resolution** (handles existing installations)

#### Docker Compose
- **Docker Compose V2** installation
- **Version control** (latest or specific version)
- **Binary installation** to /usr/local/bin
- **Executable permissions** configuration

#### Security Configuration
- **JSON file logging** with rotation (10MB max, 3 files)
- **Live restore** enabled (containers survive daemon restart)
- **Userland proxy disabled** (iptables-based port forwarding)
- **No-new-privileges** security flag
- **Docker daemon hardening** via daemon.json

#### User Management
- **Docker group creation** and management
- **User addition to docker group** (configurable list)
- **Non-root container execution** support

#### Repository Management
- **Modern GPG key handling** (/etc/apt/keyrings/*.asc)
- **Signed-by repository configuration** (no deprecated apt-key)
- **Multi-location cleanup** (removes old GPG keys and repos)
- **Automatic cache refresh** after repository changes

#### Python Integration
- **Docker Python SDK** installation (python3-docker or docker-py)
- **Ubuntu 24.04+ compatibility** (--break-system-packages flag)
- **Ansible module support** for docker_container, docker_image, etc.

#### Installation Verification
- **Docker info health check** (verify daemon accessibility)
- **Version display** in playbook output
- **Service status validation**

---

### LEMP Stack (Web Server)

**Status**: Beta | **Role**: lemp | **Tags**: lemp, web | **Conditional**: install_lemp=true

#### Components
- **Nginx** web server
- **MySQL/MariaDB** database server
- **PHP-FPM** processor
- **PHP modules** (mysql, curl, gd, intl, mbstring, soap, xml, zip)

#### Configuration
- **Virtual host support** (configurable via variables)
- **PHP-FPM pool configuration**
- **Database initialization** with secure passwords
- **Nginx server blocks** for domain hosting

---

### Swap Memory Management

**Status**: Stable | **Role**: swap | **Tags**: swap, system | **Conditional**: enable_swap=true

#### Features
- **Automatic swap sizing** based on available RAM
- **Swap file creation** and configuration
- **fstab integration** for persistent mounting
- **Swappiness configuration** for optimal performance
- **Idempotent execution** (safe to run repeatedly)

---

### Automated Maintenance (Cron Jobs)

**Status**: Stable | **Role**: cron_jobs | **Tags**: cron, automation | **Conditional**: enable_cron_jobs=true

#### Features
- **Unattended security upgrades** (automatic security patches)
- **Automatic package updates** (configurable frequency)
- **Periodic system reboots** (optional, disabled by default)
- **Log rotation** and cleanup
- **System health monitoring**

#### Safety Features
- **Warning comments** for dangerous operations
- **Configurable schedules** via variables
- **Email notification** support (when configured)

---

### Development Tools

**Status**: Beta | **Role**: dev_tools | **Tags**: dev, development | **Conditional**: install_dev_tools=true

#### Tools
- **Neovim** text editor (latest version)
- **Node.js** (configurable version, default: v22)
- **npm** package manager
- **Claude Code CLI** (optional)
- **Development dependencies** (build-essential, python3-dev)

#### Configuration
- **User-level tool installation**
- **PATH configuration** for installed tools
- **Version management** support

---

### WordPress CMS Platform

**Status**: Beta | **Role**: wordpress | **Tags**: wordpress, web, cms | **Conditional**: install_wordpress=true

#### Features
- **WordPress core installation** (latest version)
- **MySQL database creation** and configuration
- **Nginx virtual host** configuration
- **PHP-FPM integration**
- **File permission hardening**
- **wp-config.php** generation with secure keys

#### Requirements
- Requires LEMP stack (install_lemp=true)
- Domain name configuration
- MySQL credentials (from vault)

---

### SSL/TLS Certificate Management

**Status**: Beta | **Role**: certbot | **Tags**: certbot, ssl, security | **Conditional**: install_certbot=true

#### Features
- **Certbot** installation and configuration
- **Let's Encrypt** certificate automation
- **Automatic certificate renewal** (cron-based)
- **Nginx integration** for SSL configuration
- **WWW subdomain** support (optional)
- **Email notifications** for expiration warnings

#### Requirements
- Requires Nginx web server
- Valid domain name with DNS configured
- Port 80 and 443 accessible

---

## Operational Features

### Idempotency and Safety

**Status**: Stable | **All Roles**

- **Repeatable execution** without side effects
- **State checking** before modifications
- **Safe defaults** for all operations
- **Change detection** and accurate reporting
- **Handler-based service restarts** (only on config changes)

### Tag-Based Execution

**Status**: Stable | **All Roles**

- **Granular task selection** via tags
- **Role-level execution** (base, security, docker, etc.)
- **Category-level execution** (ssh, firewall, packages)
- **Task-level precision** (ssh-port, password-disable)
- **Tag hierarchy** (parent tags include child tasks)
- **Tag exclusion** support (--skip-tags)

### Error Handling and Recovery

**Status**: Stable | **Critical Roles**

- **Block/rescue patterns** for critical operations
- **Helpful error messages** with context
- **Manual recovery instructions** in rescue blocks
- **Graceful degradation** where appropriate
- **Failed task reporting** with actionable guidance

### Reboot Management

**Status**: Stable | **Post-tasks**

- **Automatic reboot detection** (/var/run/reboot-required)
- **Opt-in reboot** via tag (never by default)
- **Configurable timeouts** (300s reboot, 30s post-reboot delay)
- **Connection validation** after reboot
- **Status reporting** (indicates when reboot needed)

### Inventory and Variable Management

**Status**: Stable | **Configuration**

- **Centralized variable file** (vars/default.yml)
- **Inventory-based host management** (inventory.ini)
- **Group variables** support (group_vars)
- **Host-specific variables** support (host_vars)
- **Ansible Vault** integration for sensitive data
- **Environment variable lookups** for secrets

---

## Platform Support

### Operating Systems

**Fully Supported**:
- Ubuntu Server 20.04 LTS (amd64)
- Ubuntu Server 22.04 LTS (amd64)
- Ubuntu Server 24.04 LTS (amd64)

**Partially Tested**:
- Ubuntu 23.10 (amd64)

**Not Supported**:
- Debian (may work with modifications)
- CentOS/RHEL (different package manager and paths)
- Arch Linux (different package manager)

### Cloud Platforms

**Tested On**:
- Azure Virtual Machines
- DigitalOcean Droplets
- AWS EC2 Instances
- Bare metal servers

**Cloud-Init Support**:
- Optional cloud-init wait task
- Compatible with cloud provider metadata
- Works with cloud-provided SSH keys

---

## Monitoring and Observability

### Playbook Output

- **Task categorization** with clear prefixes (Docker |, SSH |, etc.)
- **Color-coded results** (changed, ok, failed)
- **Verbose mode support** (-v, -vv, -vvv, -vvvv)
- **Change summary** at playbook completion
- **System information** display (hostname, distribution, version)

### Verification Tasks

- **Docker installation verification** (docker info check)
- **Service status checks** (docker, ssh, fail2ban)
- **Health check tags** (verify, health-check)
- **Configuration validation** (sudoers, sshd_config)

---

## Security Features Summary

| Feature | Default State | Hardening Level |
|---------|---------------|-----------------|
| SSH Password Auth | Disabled | High |
| Root SSH Login | Disabled | High |
| UFW Firewall | Enabled (deny incoming) | High |
| Fail2ban | Enabled | High |
| Strong Passwords | Enforced (PAM) | Medium |
| Cron Permissions | Restricted (700) | Medium |
| Docker Daemon Security | Hardened | Medium |
| Swap Encryption | Not configured | N/A |
| Automatic Updates | Optional | Medium |

---

## Performance Characteristics

### Execution Time (Approximate)

**Full Playbook Run (Fresh Server)**:
- Base Setup: 3-5 minutes
- Security Hardening: 2-3 minutes
- Docker Installation: 3-5 minutes
- LEMP Stack: 5-7 minutes (if enabled)
- **Total**: 8-20 minutes (depending on options)

**Incremental Updates**:
- Security-only: 30-60 seconds
- Package updates: 2-5 minutes
- Docker-only: 1-2 minutes

### Resource Requirements

**Target Server Minimum**:
- CPU: 1 core
- RAM: 1GB (2GB recommended with Docker)
- Disk: 10GB (20GB+ recommended for Docker)
- Network: Stable internet connection

**Control Node (Where Ansible Runs)**:
- Any system with Ansible 2.9+ installed
- Python 3.6+
- SSH client

---

## Testing and Validation

### Included Test Modes

- **Syntax check** (--syntax-check)
- **Dry run mode** (--check)
- **Diff display** (--diff)
- **Verbose output** (-v through -vvvv)
- **Tag-based testing** (test specific components)

### Lint Support

- Compatible with ansible-lint
- Follows Ansible best practices
- YAML syntax validation
- Role metadata validation

---

## Documentation Features

- **Comprehensive CLAUDE.md** (project overview and guidelines)
- **TAG_REFERENCE.md** (complete tag documentation)
- **Architecture Decision Records** (ADR.md)
- **Product Requirements** (PRD.md)
- **Workflow Guides** (WORKFLOWS.md)
- **Inline task comments** (explain purpose and reasoning)
- **Variable documentation** (in vars/default.yml)

---

## Future Roadmap

### Planned Features

**Short-term** (Next 1-3 months):
- Advanced monitoring stack (Prometheus, Grafana)
- Log aggregation (ELK/Loki)
- Automated backups
- Multi-server orchestration
- Database clustering support

**Medium-term** (3-6 months):
- Kubernetes cluster setup
- CI/CD pipeline integration
- Infrastructure as Code testing (Molecule)
- Windows WSL2 support
- ARM64 architecture support

**Long-term** (6-12 months):
- Multi-cloud deployment
- Auto-scaling configuration
- Disaster recovery automation
- Compliance frameworks (CIS, PCI-DSS)
- Ansible Collection packaging

### Community Contributions

**Accepting PRs for**:
- Additional Linux distribution support
- New optional roles
- Security enhancements
- Documentation improvements
- Bug fixes and optimizations

---

## Version History

### Current Version: 2.1.0

**Major Releases**:

**v2.1.0** (2025-11-12):
- Added comprehensive documentation (ADR, FEATURES, PRD, WORKFLOWS)
- Enhanced role modularity (swap, lemp, dev_tools, cron_jobs)
- Improved tag granularity
- Ubuntu 24.04 LTS support

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

---

## Feature Request Process

To request a new feature:

1. **Check existing features** in this document
2. **Search GitHub issues** for similar requests
3. **Open a feature request issue** with:
   - Clear use case description
   - Expected behavior
   - Benefits to users
   - Potential implementation approach
4. **Discuss in issue** before submitting PR

---

**Last Updated**: 2025-11-12
**Maintainer**: Infrastructure Team
**Status**: Living document - updated with each release
