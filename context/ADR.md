# Architecture Decision Records (ADR)

This document captures the key architectural decisions made in the development of this Ansible playbook repository for automated Ubuntu server provisioning and security hardening.

## ADR Format

Each decision record follows this structure:
- **Context**: The situation and forces at play
- **Decision**: The architectural decision made
- **Consequences**: The resulting context, both positive and negative

---

## ADR-001: Role-Based Architecture

**Date**: 2024-08-31
**Status**: Accepted

### Context

When designing an Ansible playbook for server provisioning, there are multiple organizational approaches:
1. Monolithic playbook with all tasks in a single file
2. Task-based organization with separate task files
3. Role-based modular architecture
4. Collection-based package organization

The playbook needed to:
- Support multiple server types and configurations
- Enable selective execution of specific functionality
- Facilitate code reuse across different environments
- Allow team members to work on different components independently
- Scale from single server to multiple server configurations

### Decision

We adopted a **role-based modular architecture** with clear separation of concerns:
- **base_setup**: User management, packages, timezone
- **security**: SSH hardening, firewall, system hardening
- **docker**: Container runtime installation and configuration
- **swap**: Memory swap configuration
- **lemp**: Web server stack (optional)
- **dev_tools**: Development tooling (optional)
- **cron_jobs**: Automated maintenance (optional)

Each role is self-contained with its own tasks, handlers, templates, and defaults.

### Consequences

**Positive:**
- Clear separation of concerns makes debugging easier
- Roles can be developed and tested independently
- New team members can quickly understand the system
- Selective execution via role tags enables targeted updates
- Roles can be extracted to Ansible Galaxy for reuse
- Testing can be scoped to individual roles
- Documentation is naturally organized by role
- Code reuse across different playbooks and environments

**Negative:**
- More directory structure and files to navigate
- Slight learning curve for developers unfamiliar with Ansible roles
- Need to manage dependencies between roles
- More complex than a single-file playbook for simple use cases

**Mitigation:**
- Comprehensive CLAUDE.md documentation explains role architecture
- TAG_REFERENCE.md provides granular tag-based execution guide
- Clear role dependencies documented in playbook.yml
- Role execution order explicitly defined in main playbook

---

## ADR-002: UFW for Firewall Management

**Date**: 2024-09-01
**Status**: Accepted

### Context

Ubuntu servers require firewall configuration to control network access. Primary options considered:
1. **UFW** (Uncomplicated Firewall): Ubuntu's default, iptables frontend
2. **firewalld**: Used primarily on RHEL/CentOS systems
3. **iptables**: Low-level netfilter interface
4. **nftables**: Modern replacement for iptables

Key requirements:
- Simple configuration and management
- Native Ubuntu integration
- Easy-to-read syntax
- Support for common use cases (SSH, HTTP, HTTPS)
- Ansible module availability
- Default-deny incoming policy

### Decision

We selected **UFW (Uncomplicated Firewall)** as the firewall solution, configured via the `community.general.ufw` Ansible module.

Configuration approach:
- Default-deny policy for incoming traffic
- Explicit allow rules for required services
- Port definitions in `vars/default.yml` for easy customization
- Firewall enabled as part of security hardening role

### Consequences

**Positive:**
- Simple, readable configuration syntax
- Native Ubuntu integration (pre-installed on most Ubuntu versions)
- Excellent Ansible module support (`community.general.ufw`)
- Easy to understand for team members
- Straightforward rule management
- Good logging capabilities
- Minimal learning curve

**Negative:**
- Less granular control than direct iptables
- Limited advanced features compared to firewalld
- Not portable to non-Debian/Ubuntu systems
- Some edge cases require iptables knowledge

**Mitigation:**
- Document port configuration in vars/default.yml with comments
- Provide examples in TAG_REFERENCE.md for firewall-specific runs
- Include SSH port validation to prevent lockouts
- Use tags for granular firewall management

---

## ADR-003: External Role for Fail2ban

**Date**: 2024-09-01
**Status**: Accepted

### Context

Fail2ban intrusion prevention requires complex configuration including:
- Service detection and log parsing
- Ban/unban actions
- IP whitelist management
- Multiple jail configurations
- Service-specific filters

Options considered:
1. Write custom fail2ban role from scratch
2. Use community-maintained external role
3. Simple template-based configuration
4. Skip fail2ban entirely and rely only on firewall

### Decision

We adopted the **oefenweb.fail2ban** external role from Ansible Galaxy, supplemented with custom jail.local configuration via the security role.

Integration approach:
- External role handles core fail2ban installation and service management
- Custom variables passed from vars/default.yml
- Security role adds custom jail.local for additional configuration
- IP whitelist includes localhost, default IPv4, and custom addresses

### Consequences

**Positive:**
- Leverages well-tested, community-maintained code
- Reduces maintenance burden
- Handles edge cases and OS version differences
- Regular updates from community
- Saves development time
- Best practices built-in

**Negative:**
- External dependency on Ansible Galaxy
- Less control over implementation details
- Need to understand external role's variable structure
- Potential breaking changes in role updates
- Additional complexity in dependency management

**Mitigation:**
- Pin role version in requirements.yml (when needed)
- Custom jail.local allows for site-specific overrides
- Document fail2ban variables in vars/default.yml
- Test fail2ban functionality after role updates

---

## ADR-004: Idempotency as a Core Design Principle

**Date**: 2024-09-01
**Status**: Accepted

### Context

Ansible playbooks can be run in two ways:
1. **Imperative**: Execute actions regardless of current state
2. **Idempotent**: Check state and only make necessary changes

Running the playbook multiple times could result in:
- Duplicate user accounts or groups
- Repeated package installations
- Accumulating firewall rules
- Service restarts even when configuration unchanged
- Unnecessary system modifications

### Decision

All tasks in the playbook are designed to be **fully idempotent** following these principles:
- Use Ansible modules' built-in state management
- Check before modifying (stat, register, when conditions)
- Use handlers for service restarts (only trigger on change)
- Leverage `changed_when` and `failed_when` for accurate reporting
- Design tasks to be safely repeatable

### Consequences

**Positive:**
- Playbook can be run repeatedly without side effects
- Safe to re-run after failures or interruptions
- Easier to test changes incrementally
- Predictable behavior regardless of initial system state
- Supports configuration drift correction
- Enables continuous compliance enforcement
- Better Ansible reporting (shows what actually changed)

**Negative:**
- Requires more careful task design
- Some tasks need additional logic for state checking
- Slightly more verbose playbook code
- Need to understand Ansible's change detection

**Mitigation:**
- Use built-in Ansible modules (they're idempotent by default)
- Document non-obvious idempotency patterns in task comments
- Test playbook runs multiple times during development
- Use --check mode to validate idempotency

---

## ADR-005: Docker Repository Conflict Resolution

**Date**: 2024-10-15
**Status**: Accepted

### Context

Docker installation on Ubuntu frequently encounters repository conflicts:
- Multiple GPG key locations (deprecated vs. current)
- Old repository configurations from manual installations
- Conflicting sources.list entries
- apt-key deprecation warnings
- GPG key format changes (binary .gpg vs. ASCII .asc)

These conflicts cause:
- Playbook failures during Docker installation
- Inconsistent package sources
- apt update errors
- GPG verification failures

### Decision

Implement a **comprehensive cleanup and modern repository setup** pattern:

**Pre-tasks** (playbook level):
- Detect existing Docker repository configurations
- Clean conflicting repository files
- Remove Docker entries from main sources.list
- Clear apt cache

**Docker role cleanup** (role level):
- Remove GPG keys from all possible locations
- Delete all Docker .list files in sources.list.d
- Use modern /etc/apt/keyrings directory
- Download GPG key in ASCII armored format (.asc)
- Add repository with signed-by directive

### Consequences

**Positive:**
- Eliminates repository conflict errors
- Uses modern apt repository configuration (signed-by)
- Avoids deprecated apt-key warnings
- Works reliably on Ubuntu 20.04, 22.04, 24.04+
- Handles both fresh installations and existing Docker setups
- Self-healing on subsequent runs

**Negative:**
- Additional cleanup tasks increase playbook complexity
- More tasks to maintain and test
- Removes manually configured Docker repositories
- Requires knowledge of apt repository evolution

**Mitigation:**
- Comprehensive comments explain each cleanup step
- Block/rescue error handling for graceful failure recovery
- Manual recovery instructions in error messages
- TAG_REFERENCE.md includes Docker troubleshooting workflows

---

## ADR-006: Variable Centralization in vars/default.yml

**Date**: 2024-09-01
**Status**: Accepted

### Context

Ansible supports multiple variable definition locations:
1. Role defaults (roles/*/defaults/main.yml)
2. Inventory variables (inventory.ini, host_vars, group_vars)
3. Playbook variables (vars section or vars_files)
4. Command-line variables (-e key=value)
5. Role variables (roles/*/vars/main.yml)

Managing configuration across multiple files can lead to:
- Difficulty finding where values are defined
- Variable precedence confusion
- Duplication across roles
- Inconsistent configurations

### Decision

All user-configurable variables are centralized in **vars/default.yml** and loaded at playbook level:

```yaml
vars_files:
  - vars/default.yml
```

Variable organization:
- Server configuration (IP, user, passwords)
- System packages list
- Security settings (SSH, firewall, fail2ban)
- Docker configuration
- Feature flags (install_docker, install_lemp, etc.)
- Optional component settings

Sensitive data pattern:
```yaml
user_password: "{{ lookup('file', lookup('env','HOME') + '/.ssh/password') }}"
copy_local_key: "{{ lookup('file', lookup('env','HOME') + '/.ssh/id_rsa_gitlab.pub') }}"
```

### Consequences

**Positive:**
- Single source of truth for all configurations
- Easy to locate and modify settings
- Clear variable precedence (playbook vars override role defaults)
- Enables environment-specific configurations (dev/staging/prod)
- Facilitates code review (all changes visible in one file)
- Supports Ansible Vault for sensitive data

**Negative:**
- All roles depend on playbook-level variables
- Roles lose some portability (can't run standalone easily)
- Large vars file can become unwieldy with many settings
- Need to understand all variables before customization

**Mitigation:**
- Extensive comments in vars/default.yml explain each variable
- Logical grouping of related variables (# Server, # Security, etc.)
- CLAUDE.md documents variable structure and usage
- Template file (vars/vault.yml.template) for sensitive data

---

## ADR-007: Tag-Based Execution Strategy

**Date**: 2024-09-01
**Status**: Accepted

### Context

Server provisioning scenarios vary widely:
- Initial fresh server setup (run everything)
- Security updates only (packages, upgrades)
- Docker installation on existing servers
- SSH configuration changes without other modifications
- Selective component updates (firewall rules, user management)

Running the entire playbook for small changes:
- Takes unnecessary time
- Increases risk of unintended changes
- Makes testing specific components difficult
- Complicates troubleshooting

### Decision

Implement **comprehensive tag-based execution** at three granularity levels:

1. **Role-level tags**: Run entire roles
   - `base`, `security`, `docker`, `swap`, `lemp`, `cron`, `dev`

2. **Category tags**: Run related tasks across roles
   - `ssh`, `firewall`, `fail2ban`, `packages`, `user`

3. **Granular tags**: Run specific task subsets
   - `ssh-port`, `password-disable`, `docker-deps`, `apt-cache`

Special tags:
- `always`: Tasks that run regardless of tag selection
- `never`: Tasks that require explicit tag (e.g., `reboot`)

### Consequences

**Positive:**
- Surgical precision in task execution
- Faster development and testing iterations
- Reduced risk when making small changes
- Better troubleshooting capabilities
- Enables targeted documentation (TAG_REFERENCE.md)
- Supports complex workflows (step-by-step setup)

**Negative:**
- Requires careful tag design and maintenance
- More complex playbook structure
- Need to document all tag combinations
- Risk of missing dependencies between tasks

**Mitigation:**
- TAG_REFERENCE.md provides comprehensive tag documentation
- Hierarchical tag design (parent tags include child tasks)
- Test common tag combinations during development
- WORKFLOWS.md demonstrates real-world tag usage

---

## ADR-008: Handlers for Service Management

**Date**: 2024-09-01
**Status**: Accepted

### Context

Configuration changes often require service restarts:
- SSH config changes need SSH restart
- Docker daemon config needs Docker restart
- Fail2ban config needs fail2ban restart

Options for triggering restarts:
1. Restart service after every config task
2. Use handlers triggered by config changes
3. Restart services at end of playbook
4. Manual restart after playbook completion

### Decision

Use **Ansible handlers** for all service restart operations:

Pattern:
```yaml
- name: "SSH Security | Disable password authentication"
  ansible.builtin.lineinfile:
    path: /etc/ssh/sshd_config
    regexp: '^#?PasswordAuthentication'
    line: 'PasswordAuthentication no'
  notify: restart ssh
```

Handler definition (in role/handlers/main.yml):
```yaml
- name: restart ssh
  ansible.builtin.service:
    name: ssh
    state: restarted
```

### Consequences

**Positive:**
- Services restart only when configuration actually changes
- Multiple config changes trigger only one restart
- Idempotent behavior (no restart if config unchanged)
- Cleaner task output (restarts grouped at end)
- Better performance (fewer service disruptions)
- Clear change notification in playbook output

**Negative:**
- Handlers run at end of play (not immediately after task)
- Requires understanding of handler execution order
- Slightly more complex than direct service restart
- Handler failures can be harder to debug

**Mitigation:**
- Use `meta: flush_handlers` when immediate restart needed
- Document handler behavior in role documentation
- Test handler execution during playbook development
- Clear handler names match service names

---

## ADR-009: Python 3 Standardization

**Date**: 2024-09-01
**Status**: Accepted

### Context

Ansible requires Python on target hosts. Ubuntu systems may have:
- Python 2.7 (deprecated, removed in Ubuntu 20.04+)
- Python 3.x (default on modern Ubuntu)
- Both Python 2 and Python 3 installed
- `/usr/bin/python` symlink pointing to different versions

Ansible needs to know which Python interpreter to use for:
- Running modules on target hosts
- Docker Python module (docker-py)
- Package management operations

### Decision

Standardize on **Python 3** for all target hosts:

Inventory configuration:
```ini
[all:vars]
ansible_python_interpreter=/usr/bin/python3
```

Docker role Python handling:
- Install python3-pip for package management
- Use python3-docker from apt when available
- Fallback to pip with --break-system-packages for Ubuntu 24.04+

### Consequences

**Positive:**
- Consistent Python environment across all servers
- Supports modern Ubuntu releases (20.04, 22.04, 24.04+)
- Avoids Python 2 deprecation issues
- Clear dependency declaration in inventory
- Works with Ubuntu's default Python installation

**Negative:**
- Won't work on very old systems with only Python 2
- Requires Python 3 to be installed before running playbook
- Ubuntu 24.04+ requires special handling (PEP 668)

**Mitigation:**
- Document Python 3 requirement in CLAUDE.md
- base_setup role can install Python 3 if missing
- Use --break-system-packages flag for Ubuntu 24.04+
- Test on multiple Ubuntu versions (20.04, 22.04, 24.04)

---

## ADR-010: Security-First Design

**Date**: 2024-09-01
**Status**: Accepted

### Context

Server security requires multiple layers of protection:
- SSH access control
- Firewall configuration
- Intrusion prevention
- System hardening
- Password policies

Options for security implementation:
1. Minimal security (firewall only)
2. Moderate security (firewall + SSH hardening)
3. Comprehensive security (all layers)
4. Deferred security (set up later manually)

### Decision

Implement **security-first comprehensive hardening** as a core role:

Security layers:
1. **SSH Hardening**
   - Disable password authentication (key-based only)
   - Disable root login
   - Optional non-standard port

2. **Firewall (UFW)**
   - Default-deny incoming policy
   - Explicit allow for required services
   - Configurable ports via variables

3. **Fail2ban**
   - Automatic IP banning for failed SSH attempts
   - DDOS protection (optional)
   - Whitelist for known IPs

4. **System Hardening**
   - Strong password policies (PAM)
   - Restricted cron directory permissions
   - Docker daemon security settings

Security role executes early in playbook (after base_setup, before Docker).

### Consequences

**Positive:**
- Secure-by-default server configuration
- Defense in depth (multiple security layers)
- Prevents common attack vectors (brute force, unauthorized access)
- Compliance-ready for security audits
- Reduces attack surface significantly
- Industry best practices built-in

**Negative:**
- More complex initial setup
- Risk of SSH lockout if misconfigured
- Stricter requirements for users
- Potential incompatibility with some legacy systems
- Requires understanding of security concepts

**Mitigation:**
- Comprehensive testing before production deployment
- SSH key validation before disabling password auth
- Firewall allows SSH before enabling default-deny
- CLAUDE.md documents security settings and rationale
- WORKFLOWS.md includes security configuration workflow
- Tag-based execution for incremental security hardening

---

## ADR-011: Reboot Handling Strategy

**Date**: 2024-10-15
**Status**: Accepted

### Context

System updates, especially kernel updates, often require reboots to take effect:
- Kernel security patches
- Core system library updates
- Critical security updates

Options for handling reboots:
1. Always reboot after playbook execution
2. Never reboot (leave to manual intervention)
3. Conditional reboot based on system state
4. Prompt user for reboot decision

Ubuntu creates `/var/run/reboot-required` file when reboot is needed.

### Decision

Implement **conditional reboot with explicit opt-in** via tags:

Pattern:
```yaml
- name: Check if reboot is required
  ansible.builtin.stat:
    path: /var/run/reboot-required
  register: reboot_required_file
  tags: always

- name: Reboot if required
  reboot:
    msg: Reboot initiated by Ansible due to kernel updates
    reboot_timeout: 300
    post_reboot_delay: 30
  when: reboot_required_file.stat.exists
  tags:
    - never
    - reboot
```

Reboot behavior:
- Check is always performed (tag: always)
- Reboot task tagged with 'never' (requires explicit -t reboot)
- User controls reboot via tag selection
- Graceful reboot with proper timeout handling

### Consequences

**Positive:**
- Safe default (no unexpected reboots)
- User maintains control over reboot timing
- Detects when reboot is actually needed
- Proper wait for system to come back online
- Clear communication via reboot message
- Supports automation (can add -t reboot to scripts)

**Negative:**
- System may be running with outdated kernel
- Users might forget to reboot when needed
- Requires understanding of 'never' tag semantics
- Two-step process (run playbook, then reboot if needed)

**Mitigation:**
- Document reboot handling in CLAUDE.md
- WORKFLOWS.md includes reboot procedures
- Playbook output shows if reboot is required
- TAG_REFERENCE.md explains reboot tag usage
- Clear reboot message indicates why rebooting

---

## ADR-012: Error Handling with Block/Rescue

**Date**: 2024-10-15
**Status**: Accepted

### Context

Ansible playbook execution can fail at various points:
- Network issues during package installation
- Repository conflicts during Docker setup
- File permission errors during configuration
- Service restart failures

Options for error handling:
1. Let tasks fail and stop execution
2. Use `ignore_errors: yes` (masks all errors)
3. Use `failed_when` for conditional failures
4. Use `block:` and `rescue:` for structured error handling

### Decision

Implement **block/rescue pattern** for critical and error-prone sections:

Pattern:
```yaml
- name: "Docker Installation | Complete Docker setup"
  ansible.builtin.block:
    # Normal tasks here
    - name: Install Docker packages
      ansible.builtin.apt:
        pkg:
          - docker-ce
          - docker-ce-cli
    # ... more tasks
  rescue:
    - name: "Error Handler | Display failure message"
      ansible.builtin.debug:
        msg: Failed to install Docker. Please check errors above.

    - name: "Error Handler | Manual recovery instructions"
      ansible.builtin.debug:
        msg: |
          Steps to recover:
          1. Clean repository files
          2. Re-run playbook with -t docker
  tags:
    - docker
```

Applied to:
- Docker role (repository conflicts)
- Security role (system hardening)
- Any role with complex external dependencies

### Consequences

**Positive:**
- Graceful error handling with helpful messages
- Recovery instructions provided to user
- Playbook doesn't leave system in broken state
- Clear error context for troubleshooting
- Can perform cleanup in rescue block
- Better user experience during failures

**Negative:**
- More verbose playbook code
- Requires careful design of rescue tasks
- Can hide underlying errors if not implemented carefully
- Need to decide which errors to catch vs. propagate

**Mitigation:**
- Always display original error messages in rescue
- Provide specific, actionable recovery instructions
- Use rescue for known failure scenarios only
- Test rescue blocks by intentionally causing failures
- Document known failure modes in role documentation

---

## Summary of Architectural Principles

The architectural decisions in this playbook repository are guided by these core principles:

1. **Modularity**: Role-based architecture for separation of concerns
2. **Idempotency**: Safe to run repeatedly without side effects
3. **Security-First**: Comprehensive hardening by default
4. **Flexibility**: Tag-based execution for surgical precision
5. **Reliability**: Block/rescue error handling with helpful recovery guidance
6. **Maintainability**: Centralized variables and clear documentation
7. **Modern Standards**: Python 3, modern apt repository config, current security practices
8. **User Control**: Explicit opt-in for destructive operations (reboots, etc.)

These decisions enable:
- Rapid server provisioning with security built-in
- Safe incremental updates and configuration changes
- Easy troubleshooting and recovery from failures
- Clear understanding of system state and changes
- Scalability from single server to fleet management

---

**Last Updated**: 2025-11-12
**Status**: Living document - updated as new architectural decisions are made
