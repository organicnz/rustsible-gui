# Ansible Playbook Tag Reference Guide

This document provides a comprehensive reference for all available tags to enable granular, selective task execution.

## Quick Start Examples

### Run only SSH hardening tasks
```bash
ansible-playbook playbook.yml -t ssh -l webservers -u organic --ask-become-pass
```

### Change SSH port only (no auth changes)
```bash
ansible-playbook playbook.yml -t ssh-port -l webservers -u organic --ask-become-pass
```

### Install Docker dependencies without installing Docker
```bash
ansible-playbook playbook.yml -t docker-deps -l webservers -u organic --ask-become-pass
```

### Update packages only (no full system upgrade)
```bash
ansible-playbook playbook.yml -t packages -l webservers -u organic --ask-become-pass
```

---

## Base Setup Role Tags

| Tag | Description | Use Case |
|-----|-------------|----------|
| `connection` | System availability checks | Verify system is reachable |
| `cloud-init` | Cloud initialization | Wait for cloud-init on cloud VMs |
| `timezone` | Timezone configuration | Set system timezone |
| `user` | All user management tasks | Create users and configure access |
| `create-user` | User creation only | Add new user account |
| `sudo` | Sudo configuration | Configure passwordless sudo |
| `ssh-keys` | SSH key deployment | Deploy public keys for users |
| `password` | Password configuration | Set user passwords |
| `groups` | Group management | Create and manage groups |
| `packages` | Package operations | Install/update packages |
| `apt-cache` | APT cache refresh | Update package lists |
| `install` | Package installation | Install new packages |
| `upgrade` | System upgrades | Upgrade existing packages |
| `cleanup` | Package cleanup | Remove orphaned packages |
| `maintenance` | System maintenance | Cleanup and optimization |
| `version-check` | Version verification | Check Ubuntu version |
| `eol` | EOL cleanup | Remove end-of-life warnings |

### Examples

**Create user and deploy SSH key only:**
```bash
ansible-playbook playbook.yml -t create-user,ssh-keys -l webservers -u root -k
```

**Update package cache and install packages (no upgrades):**
```bash
ansible-playbook playbook.yml -t apt-cache,install -l webservers -u organic --ask-become-pass
```

**Timezone configuration only:**
```bash
ansible-playbook playbook.yml -t timezone -l webservers -u organic --ask-become-pass
```

---

## Security Role Tags

| Tag | Description | Use Case |
|-----|-------------|----------|
| `ssh` | All SSH hardening | Complete SSH security config |
| `ssh-port` | SSH port changes only | Change SSH port |
| `ssh-auth` | SSH authentication config | Disable passwords/root login |
| `password-disable` | Disable password auth | Force key-based auth |
| `root-disable` | Disable root login | Prevent direct root access |
| `firewall` | All firewall tasks | Complete UFW setup |
| `ufw` | UFW-specific tasks | UFW configuration |
| `ssh-allow` | Allow SSH in firewall | Permit SSH connections |
| `custom-ports` | Allow custom ports | Open additional ports |
| `enable-firewall` | Enable UFW | Activate firewall |
| `fail2ban` | Fail2ban configuration | Intrusion prevention |
| `jail-config` | Jail configuration | Custom fail2ban rules |
| `hardening` | System hardening | OS-level security |
| `system-hardening` | All hardening tasks | Complete system hardening |
| `pam` | PAM configuration | Password policies |
| `password-policy` | Password requirements | Strong password rules |
| `permissions` | File permissions | Secure file/directory perms |
| `cron-security` | Cron permissions | Secure cron directories |
| `ddos` | DDOS protection | HTTP flood protection |
| `web-protection` | Web security | Web server protection |

### Examples

**Configure firewall without SSH changes:**
```bash
ansible-playbook playbook.yml -t firewall -l webservers -u organic --ask-become-pass
```

**Disable SSH password authentication only:**
```bash
ansible-playbook playbook.yml -t password-disable -l webservers -u organic --ask-become-pass
```

**Apply all hardening except DDOS protection:**
```bash
ansible-playbook playbook.yml -t hardening --skip-tags ddos -l webservers -u organic --ask-become-pass
```

**Change SSH port and update firewall rule:**
```bash
ansible-playbook playbook.yml -t ssh-port,ssh-allow -l webservers -u organic --ask-become-pass
```

---

## Docker Role Tags

| Tag | Description | Use Case |
|-----|-------------|----------|
| `docker` | All Docker tasks | Complete Docker setup |
| `docker-deps` | Docker dependencies | Install prerequisites only |
| `docker-cleanup` | Repository cleanup | Clean existing configs |
| `repo-cleanup` | Remove old repos | Delete conflicting repos |
| `docker-repo` | Repository setup | Configure Docker repo |
| `gpg-setup` | GPG key management | Setup repository keys |
| `gpg-key` | Download GPG key | Fetch Docker GPG key |
| `cache-cleanup` | APT cache clean | Clear cached packages |
| `apt-update` | Update package cache | Refresh package lists |
| `apt-source` | Add APT source | Add Docker repository |
| `docker-install` | Install Docker | Install Docker packages |
| `docker-compose` | Docker Compose | Install Compose |
| `compose-install` | Compose installation | Download Compose binary |
| `docker-service` | Service management | Start/enable Docker |
| `service-management` | Service control | Manage Docker daemon |
| `docker-python` | Python Docker SDK | Install Python module |
| `python-modules` | Python packages | Python dependencies |
| `pip-install` | Pip installation | Install via pip |
| `docker-users` | User management | Add users to docker group |
| `user-groups` | Group membership | Manage group membership |
| `docker-security` | Security config | Apply security settings |
| `docker-daemon` | Daemon config | Configure daemon.json |
| `verify` | Verification | Health checks |
| `health-check` | Health validation | Verify installation |

### Examples

**Install Docker dependencies and repository only (no Docker installation):**
```bash
ansible-playbook playbook.yml -t docker-deps,docker-repo -l webservers -u organic --ask-become-pass
```

**Install Docker Compose only (assumes Docker already installed):**
```bash
ansible-playbook playbook.yml -t docker-compose -l webservers -u organic --ask-become-pass
```

**Add users to docker group:**
```bash
ansible-playbook playbook.yml -t docker-users -l webservers -u organic --ask-become-pass
```

**Update Docker security configuration only:**
```bash
ansible-playbook playbook.yml -t docker-security -l webservers -u organic --ask-become-pass
```

**Clean up Docker repositories and reinstall:**
```bash
ansible-playbook playbook.yml -t docker-cleanup,docker-repo,docker-install -l webservers -u organic --ask-become-pass
```

**Verify Docker installation:**
```bash
ansible-playbook playbook.yml -t verify -l webservers -u organic --ask-become-pass
```

---

## Main Playbook Role Tags

| Tag | Description | Roles/Tasks Included |
|-----|-------------|---------------------|
| `base` | Base system setup | base_setup role |
| `setup` | Initial setup | base_setup role |
| `swap` | Swap configuration | swap role |
| `system` | System tasks | base_setup, swap |
| `security` | Security hardening | security, fail2ban roles |
| `docker` | Docker installation | docker role |
| `lemp` | LEMP stack | lemp role |
| `web` | Web server | lemp role |
| `cron` | Cron jobs | cron_jobs role |
| `automation` | Automation tasks | cron_jobs role |
| `dev` | Development tools | dev_tools role |
| `development` | Dev environment | dev_tools role |
| `reboot` | System reboot | Reboot task (with 'never') |

---

## Advanced Usage Patterns

### Multiple Tag Selection

Run user creation and Docker installation:
```bash
ansible-playbook playbook.yml -t user,docker -l webservers -u root -k
```

### Tag Exclusion

Run all security tasks except fail2ban:
```bash
ansible-playbook playbook.yml -t security --skip-tags fail2ban -l webservers -u organic --ask-become-pass
```

### Role-Specific Tag Filtering

Run only SSH tasks from security role:
```bash
ansible-playbook playbook.yml --tags "security,ssh" -l webservers -u organic --ask-become-pass
```

### Dry Run with Tags

Test what would happen without making changes:
```bash
ansible-playbook playbook.yml -t docker --check -l webservers -u organic --ask-become-pass
```

### Verbose Output with Tag Selection

Debug specific tasks with verbose output:
```bash
ansible-playbook playbook.yml -t docker-repo -vvv -l webservers -u organic --ask-become-pass
```

---

## Common Workflows

### Initial Server Setup
```bash
# Step 1: Base setup as root
ansible-playbook playbook.yml -t base -l webservers -u root -k

# Step 2: Security hardening as new user
ansible-playbook playbook.yml -t security -l webservers -u organic --ask-become-pass

# Step 3: Install Docker
ansible-playbook playbook.yml -t docker -l webservers -u organic --ask-become-pass
```

### Security Updates Only
```bash
# Update packages and apply security patches
ansible-playbook playbook.yml -t packages,upgrade -l webservers -u organic --ask-become-pass
```

### Troubleshooting Docker Installation
```bash
# Clean and reinstall Docker
ansible-playbook playbook.yml -t docker-cleanup,docker -l webservers -u organic --ask-become-pass
```

### User Management
```bash
# Add new user and deploy SSH keys
ansible-playbook playbook.yml -t create-user,ssh-keys -l webservers -u organic --ask-become-pass
```

### Firewall Reconfiguration
```bash
# Update firewall rules only
ansible-playbook playbook.yml -t custom-ports -l webservers -u organic --ask-become-pass
```

---

## Task Naming Convention

All tasks follow a consistent naming pattern for easy identification:

```
"Category | Action Description"
```

Examples:
- `Docker Prerequisites | Install required system packages`
- `SSH Security | Configure SSH daemon for maximum security`
- `User Management | Create non-root user with sudo privileges`
- `Firewall | Allow SSH on port 22`

This makes task output easy to read and understand during playbook execution.

---

## Best Practices

1. **Test with --check first**: Always use `--check` mode to preview changes
2. **Use specific tags**: Prefer specific tags over broad ones for precision
3. **Combine related tags**: Group related tasks for efficient execution
4. **Document custom runs**: Keep notes of tag combinations for complex scenarios
5. **Verify after changes**: Use `-t verify` or `-t health-check` after installations
6. **Skip unnecessary tasks**: Use `--skip-tags` to exclude unwanted operations

---

## Tag Hierarchy

Tags are organized hierarchically. Running a parent tag executes all child tasks:

```
docker (all Docker tasks)
├── docker-deps (dependencies)
├── docker-repo (repository)
│   ├── gpg-setup
│   └── apt-source
├── docker-install (installation)
├── docker-security (security)
│   └── docker-daemon
└── verify (verification)
```

This allows both broad and granular control over task execution.
