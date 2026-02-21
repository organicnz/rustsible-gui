# Operational Workflows

This document provides step-by-step workflows for common server provisioning and maintenance tasks using this Ansible playbook.

---

## Table of Contents

1. [Initial Server Setup](#workflow-1-initial-server-setup)
2. [Adding New Servers to Inventory](#workflow-2-adding-new-servers-to-inventory)
3. [Security Configuration Updates](#workflow-3-security-configuration-updates)
4. [Docker Deployment](#workflow-4-docker-deployment-to-existing-servers)
5. [Package Updates and Patching](#workflow-5-package-updates-and-security-patching)
6. [SSH Port Change](#workflow-6-changing-ssh-port)
7. [Firewall Rule Management](#workflow-7-managing-firewall-rules)
8. [User and SSH Key Management](#workflow-8-user-and-ssh-key-management)
9. [Troubleshooting Failed Runs](#workflow-9-troubleshooting-failed-playbook-runs)
10. [Disaster Recovery](#workflow-10-disaster-recovery-server-rebuild)

---

## Workflow 1: Initial Server Setup

**Scenario**: Deploy a fresh Ubuntu server with full security hardening and Docker.

**Prerequisites**:
- Fresh Ubuntu 20.04/22.04/24.04 server
- Root SSH access (password or key)
- Server has internet connectivity
- Local machine has Ansible 2.9+ installed

### Step 1: Prepare SSH Access

If using SSH keys:
```bash
# Add your SSH key to ssh-agent
eval $(ssh-agent)
ssh-add ~/.ssh/azure_id_rsa.pem

# Test SSH connection
ssh -i ~/.ssh/azure_id_rsa.pem root@152.00.000.00
```

If using password authentication:
```bash
# Ensure you know the root password
# You'll be prompted during playbook execution
```

### Step 2: Configure Inventory

Edit `inventory.ini`:
```ini
[webservers]
# Production server
152.00.000.00 ansible_user=root ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab

[all:vars]
ansible_python_interpreter=/usr/bin/python3
```

### Step 3: Configure Variables

Edit `vars/default.yml`:
```yaml
# Server Configuration
ip_address: "152.00.000.00"
added_user: organic
copy_local_key: "{{ lookup('file', lookup('env','HOME') + '/.ssh/id_rsa_gitlab.pub') }}"

# Security Settings
disable_root_login: true
password_authentication: false
ssh_port: 22

# Docker Installation
install_docker: true
docker_users:
  - organic

# Firewall Ports
ufw_allowed_ports:
  - port: 22
    proto: tcp
  - port: 80
    proto: tcp
  - port: 443
    proto: tcp
```

### Step 4: Validate Configuration

```bash
# Syntax check
ansible-playbook playbook.yml --syntax-check

# Dry run (preview changes without applying)
ansible-playbook playbook.yml -l webservers -u root -k --check

# Review what would change
ansible-playbook playbook.yml -l webservers -u root -k --check --diff
```

### Step 5: Execute Playbook

```bash
# Initial setup as root
ansible-playbook playbook.yml -l webservers -u root -k
```

When prompted, enter root password (if not using SSH key).

### Step 6: Verify Deployment

```bash
# Test SSH access with new user
ssh -i ~/.ssh/id_rsa_gitlab organic@152.00.000.00

# Verify sudo access
sudo whoami  # Should return: root

# Check Docker installation
docker --version
docker-compose --version
sudo systemctl status docker

# Verify firewall
sudo ufw status verbose

# Check fail2ban
sudo fail2ban-client status
```

### Step 7: Optional Reboot

```bash
# Check if reboot is needed
ssh organic@152.00.000.00 "ls -l /var/run/reboot-required"

# If file exists, reboot
ansible-playbook playbook.yml -t reboot -l webservers -u organic --ask-become-pass
```

**Expected Duration**: 8-15 minutes
**Success Indicators**:
- SSH access works with new user and key
- Root login disabled
- UFW firewall active
- Fail2ban running
- Docker installed and running

---

## Workflow 2: Adding New Servers to Inventory

**Scenario**: Add multiple new servers to be managed by the same playbook.

### Step 1: Obtain Server Details

Collect for each server:
- IP address or hostname
- SSH access method (key path)
- Server purpose/environment (production, staging, dev)

### Step 2: Update Inventory

Edit `inventory.ini`:
```ini
[webservers]
# Production servers
prod-web1 ansible_host=152.00.000.00 ansible_user=root ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab
prod-web2 ansible_host=152.53.136.85 ansible_user=root ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab

[staging]
# Staging servers
staging-web1 ansible_host=10.0.1.10 ansible_user=root ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab

[development]
# Development servers
dev-web1 ansible_host=192.168.1.100 ansible_user=root ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab

[all:vars]
ansible_python_interpreter=/usr/bin/python3
```

### Step 3: Create Group Variables (Optional)

Create `group_vars/webservers.yml`:
```yaml
---
# Production-specific settings
install_docker: true
install_lemp: true
disable_root_login: true
password_authentication: false
```

Create `group_vars/staging.yml`:
```yaml
---
# Staging-specific settings
install_docker: true
install_lemp: false
disable_root_login: false  # Keep root access for testing
password_authentication: true
```

### Step 4: Test Connectivity

```bash
# Ping all servers
ansible all -m ping -u root -k

# Ping specific group
ansible webservers -m ping -u root -k

# Gather facts from all servers
ansible all -m setup -u root -k | grep ansible_distribution
```

### Step 5: Provision Servers

```bash
# Provision all production servers
ansible-playbook playbook.yml -l webservers -u root -k

# Provision only staging
ansible-playbook playbook.yml -l staging -u root -k

# Provision specific server
ansible-playbook playbook.yml -l prod-web1 -u root -k
```

### Step 6: Verify Inventory

```bash
# List all hosts
ansible all --list-hosts

# List production hosts
ansible webservers --list-hosts

# Show inventory tree
ansible-inventory --graph
```

**Best Practices**:
- Use descriptive hostnames (environment-purpose-number)
- Group servers by environment and role
- Use group_vars for environment-specific settings
- Document server purpose in inventory comments
- Test connectivity before running full playbook

---

## Workflow 3: Security Configuration Updates

**Scenario**: Update security settings across all servers (SSH hardening, firewall rules, fail2ban).

### Step 1: Review Current Security Settings

```bash
# Check current SSH configuration
ansible webservers -m shell -a "grep -E 'PasswordAuthentication|PermitRootLogin' /etc/ssh/sshd_config" -u organic --become

# Check UFW status
ansible webservers -m shell -a "ufw status numbered" -u organic --become

# Check fail2ban status
ansible webservers -m shell -a "fail2ban-client status sshd" -u organic --become
```

### Step 2: Update Security Variables

Edit `vars/default.yml`:
```yaml
# Harden SSH (disable all password auth)
disable_root_login: true
password_authentication: false
ssh_port: 22  # Or change to non-standard port

# Stricter fail2ban
fail2ban_bantime: 86400    # 24 hours
fail2ban_findtime: 600     # 10 minutes
fail2ban_maxretry: 3       # 3 failed attempts

# Additional firewall ports (if needed)
ufw_allowed_ports:
  - port: 22
    proto: tcp
  - port: 443
    proto: tcp
```

### Step 3: Test Changes with Dry Run

```bash
# Preview security changes
ansible-playbook playbook.yml -t security -l webservers -u organic --ask-become-pass --check --diff
```

### Step 4: Apply Security Updates

```bash
# Apply to one server first
ansible-playbook playbook.yml -t security -l prod-web1 -u organic --ask-become-pass

# Verify SSH still works
ssh organic@prod-web1

# Apply to all servers
ansible-playbook playbook.yml -t security -l webservers -u organic --ask-become-pass
```

### Step 5: Verify Security Configuration

```bash
# Test SSH with password (should fail)
ssh -o PubkeyAuthentication=no organic@prod-web1
# Expected: Permission denied (publickey)

# Test root login (should fail)
ssh root@prod-web1
# Expected: Permission denied

# Verify firewall rules
ansible webservers -m shell -a "ufw status verbose" -u organic --become

# Check fail2ban jails
ansible webservers -m shell -a "fail2ban-client status" -u organic --become
```

### Step 6: Monitor Fail2ban Activity

```bash
# Check banned IPs
ansible webservers -m shell -a "fail2ban-client status sshd" -u organic --become

# View fail2ban log
ansible webservers -m shell -a "tail -n 50 /var/log/fail2ban.log" -u organic --become

# Unban an IP if needed
ansible webservers -m shell -a "fail2ban-client set sshd unbanip 1.2.3.4" -u organic --become
```

**Safety Tips**:
- Always test on one server before fleet-wide rollout
- Keep a root console session open during SSH changes
- Document current SSH port before changing it
- Whitelist your IP in fail2ban before applying
- Have out-of-band access (cloud console, IPMI) ready

---

## Workflow 4: Docker Deployment to Existing Servers

**Scenario**: Install Docker on servers that were provisioned without it.

### Step 1: Verify Current Docker Status

```bash
# Check if Docker is installed
ansible webservers -m shell -a "docker --version" -u organic --become
# Expected: Command not found (if not installed)

# Check for existing Docker installations
ansible webservers -m shell -a "dpkg -l | grep docker" -u organic --become
```

### Step 2: Update Variables

Edit `vars/default.yml`:
```yaml
# Enable Docker installation
install_docker: true

# Specify Docker Compose version (or use "latest")
docker_compose_version: "latest"

# Add users to docker group
docker_users:
  - organic
  - deploy  # Add other users if needed
```

### Step 3: Dry Run Docker Installation

```bash
# Preview Docker installation tasks
ansible-playbook playbook.yml -t docker -l webservers -u organic --ask-become-pass --check
```

### Step 4: Install Docker

```bash
# Install on one server first (test)
ansible-playbook playbook.yml -t docker -l prod-web1 -u organic --ask-become-pass

# Verify installation on test server
ssh organic@prod-web1 "docker run hello-world"

# If successful, install on all servers
ansible-playbook playbook.yml -t docker -l webservers -u organic --ask-become-pass
```

### Step 5: Verify Docker Installation

```bash
# Check Docker version
ansible webservers -m shell -a "docker --version" -u organic --become

# Check Docker Compose version
ansible webservers -m shell -a "docker-compose --version" -u organic --become

# Verify Docker daemon status
ansible webservers -m shell -a "systemctl status docker" -u organic --become

# Test Docker without sudo
ansible webservers -m shell -a "docker info" -u organic

# Check Docker daemon configuration
ansible webservers -m shell -a "cat /etc/docker/daemon.json" -u organic --become
```

### Step 6: Optional - Configure Docker Daemon

If you need custom Docker daemon settings, edit daemon.json template in role:
```json
{
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  },
  "live-restore": true,
  "userland-proxy": false,
  "no-new-privileges": true
}
```

Then apply:
```bash
ansible-playbook playbook.yml -t docker-daemon -l webservers -u organic --ask-become-pass
```

**Troubleshooting**:
- If repository conflicts occur, playbook auto-cleans and retries
- If docker group changes don't apply, users need to log out and back in
- Check `/var/log/ansible.log` for detailed error messages
- For Ubuntu 24.04+, ensure python3-docker or docker-py installed

---

## Workflow 5: Package Updates and Security Patching

**Scenario**: Apply security updates and package upgrades to all servers.

### Step 1: Check for Available Updates

```bash
# Update package cache and check for upgrades
ansible webservers -m shell -a "apt-get update && apt list --upgradable" -u organic --become

# Check for security updates only
ansible webservers -m shell -a "unattended-upgrades --dry-run" -u organic --become
```

### Step 2: Dry Run Package Updates

```bash
# Preview package updates
ansible-playbook playbook.yml -t packages,upgrade -l webservers -u organic --ask-become-pass --check

# Show what would be upgraded
ansible-playbook playbook.yml -t upgrade -l webservers -u organic --ask-become-pass --check --diff
```

### Step 3: Apply Updates (Staged Rollout)

```bash
# Update one server first (canary)
ansible-playbook playbook.yml -t packages,upgrade -l prod-web1 -u organic --ask-become-pass

# Verify canary server
ssh organic@prod-web1 "uptime && docker ps"

# Update remaining production servers
ansible-playbook playbook.yml -t packages,upgrade -l 'webservers:!prod-web1' -u organic --ask-become-pass
```

### Step 4: Check for Required Reboots

```bash
# Check if any server needs reboot
ansible webservers -m shell -a "ls -l /var/run/reboot-required" -u organic --become

# Check which packages require reboot
ansible webservers -m shell -a "cat /var/run/reboot-required.pkgs" -u organic --become
```

### Step 5: Schedule and Execute Reboots

```bash
# Reboot one server at a time
ansible-playbook playbook.yml -t reboot -l prod-web1 -u organic --ask-become-pass

# Wait for server to come back online (automatic in playbook)
# Verify services after reboot
ansible prod-web1 -m shell -a "systemctl is-active docker nginx mysql" -u organic --become

# Continue with remaining servers
ansible-playbook playbook.yml -t reboot -l prod-web2 -u organic --ask-become-pass
```

### Step 6: Verify System Health After Updates

```bash
# Check system uptime
ansible webservers -m shell -a "uptime" -u organic

# Verify critical services
ansible webservers -m shell -a "systemctl status docker sshd fail2ban" -u organic --become

# Check for errors in system logs
ansible webservers -m shell -a "journalctl -p err -b" -u organic --become

# Verify Docker containers are running (if applicable)
ansible webservers -m shell -a "docker ps" -u organic
```

**Best Practices**:
- Always test on staging/dev servers first
- Update during maintenance window
- Have rollback plan ready
- Monitor service health after updates
- Stagger reboots to maintain service availability
- Document update outcomes and any issues

**Alternative: Automated Updates**

To enable unattended security upgrades:
```yaml
# In vars/default.yml
enable_unattended_upgrades: true
enable_auto_updates: true
```

Then apply:
```bash
ansible-playbook playbook.yml -t cron -l webservers -u organic --ask-become-pass
```

---

## Workflow 6: Changing SSH Port

**Scenario**: Change SSH port from default 22 to non-standard port 2222 for security obscurity.

### Step 1: Plan the Change

**Pre-flight checklist**:
- [ ] Have console access to servers (cloud provider console, IPMI, etc.)
- [ ] Test SSH key access is working
- [ ] Document current SSH port
- [ ] Verify no automation depends on port 22
- [ ] Schedule change during maintenance window

### Step 2: Update Variables

Edit `vars/default.yml`:
```yaml
# Change SSH port
ssh_port: 2222

# Update firewall to allow new port
ufw_allowed_ports:
  - port: 2222  # New SSH port
    proto: tcp
  - port: 80
    proto: tcp
  - port: 443
    proto: tcp
```

### Step 3: Test on Single Server

```bash
# Apply changes to test server
ansible-playbook playbook.yml -t ssh-port,ssh-allow -l dev-web1 -u organic --ask-become-pass

# Verify SSH works on new port
ssh -p 2222 organic@dev-web1

# Verify old port is closed
ssh -p 22 organic@dev-web1
# Expected: Connection refused after timeout
```

### Step 4: Update Inventory for New Port

Edit `inventory.ini`:
```ini
[webservers]
prod-web1 ansible_host=152.00.000.00 ansible_user=organic ansible_port=2222 ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab
prod-web2 ansible_host=152.53.136.85 ansible_user=organic ansible_port=2222 ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab

[all:vars]
ansible_python_interpreter=/usr/bin/python3
```

### Step 5: Apply to Production Servers

```bash
# Before changing port, ensure we can still connect on port 22
ansible webservers -m ping -u organic --ask-become-pass

# Apply SSH port change (one server at a time for safety)
ansible-playbook playbook.yml -t ssh-port,ssh-allow -l prod-web1 -u organic --ask-become-pass

# Test connection on new port
ssh -p 2222 organic@prod-web1

# Continue with remaining servers
ansible-playbook playbook.yml -t ssh-port,ssh-allow -l webservers -u organic --ask-become-pass
```

### Step 6: Update SSH Config (Optional)

Update `~/.ssh/config` on your local machine:
```
Host prod-web1
    HostName 152.00.000.00
    Port 2222
    User organic
    IdentityFile ~/.ssh/id_rsa_gitlab

Host prod-web2
    HostName 152.53.136.85
    Port 2222
    User organic
    IdentityFile ~/.ssh/id_rsa_gitlab
```

Now you can connect with:
```bash
ssh prod-web1  # Automatically uses port 2222
```

### Step 7: Verify and Document

```bash
# Verify SSH configuration
ansible webservers -m shell -a "grep '^Port' /etc/ssh/sshd_config" -u organic --become

# Verify firewall rules
ansible webservers -m shell -a "ufw status numbered | grep 2222" -u organic --become

# Test connectivity
ansible webservers -m ping -u organic
```

Document the change:
- Update runbooks and documentation
- Notify team members of new SSH port
- Update monitoring tools
- Update SSH jump host configurations

**Recovery Procedure (if locked out)**:
1. Access server via cloud provider console
2. Edit `/etc/ssh/sshd_config` and change `Port` back to 22
3. Run `systemctl restart ssh`
4. Re-run playbook with correct configuration

---

## Workflow 7: Managing Firewall Rules

**Scenario**: Add, modify, or remove firewall rules for new applications or services.

### Step 1: Identify Required Ports

**Example scenarios**:
- Application server on port 8080
- Database access on port 3306
- Monitoring on port 9090
- Custom protocol on UDP port 5000

### Step 2: Update Firewall Configuration

Edit `vars/default.yml`:
```yaml
ufw_allowed_ports:
  - port: "{{ ssh_port }}"
    proto: tcp
  - port: 80
    proto: tcp
  - port: 443
    proto: tcp
  - port: 8080      # Application server
    proto: tcp
  - port: 3306      # MySQL (for specific IPs only)
    proto: tcp
  - port: 9090      # Prometheus
    proto: tcp
  - port: 5000      # Custom UDP service
    proto: udp
```

### Step 3: Preview Changes

```bash
# Dry run firewall changes
ansible-playbook playbook.yml -t custom-ports -l webservers -u organic --ask-become-pass --check
```

### Step 4: Apply Firewall Changes

```bash
# Apply to one server first
ansible-playbook playbook.yml -t custom-ports -l prod-web1 -u organic --ask-become-pass

# Verify port is open
nc -zv prod-web1 8080
# Or from remote
curl http://prod-web1:8080

# Apply to all servers
ansible-playbook playbook.yml -t custom-ports -l webservers -u organic --ask-become-pass
```

### Step 5: Verify Firewall Rules

```bash
# List all UFW rules
ansible webservers -m shell -a "ufw status numbered" -u organic --become

# Check specific port
ansible webservers -m shell -a "ufw status | grep 8080" -u organic --become

# Test port accessibility from outside
for server in prod-web1 prod-web2; do
  nc -zv $server 8080 && echo "$server:8080 is open"
done
```

### Advanced Firewall Management

**Allow specific IP ranges**:
```bash
# Allow MySQL access only from app servers (10.0.1.0/24)
ansible webservers -m shell -a "ufw allow from 10.0.1.0/24 to any port 3306 proto tcp" -u organic --become
```

**Delete a firewall rule**:
```bash
# List rules with numbers
ansible webservers -m shell -a "ufw status numbered" -u organic --become

# Delete rule by number (e.g., rule 5)
ansible webservers -m shell -a "ufw delete 5" -u organic --become
```

**Reset firewall completely**:
```bash
# WARNING: This will remove all rules!
ansible webservers -m shell -a "ufw --force reset" -u organic --become

# Re-run security role to restore rules
ansible-playbook playbook.yml -t firewall -l webservers -u organic --ask-become-pass
```

**Temporary rule testing**:
```bash
# Add temporary rule manually
ansible webservers -m shell -a "ufw allow 9999/tcp" -u organic --become

# Test your application

# Remove temporary rule
ansible webservers -m shell -a "ufw delete allow 9999/tcp" -u organic --become
```

---

## Workflow 8: User and SSH Key Management

**Scenario**: Add new team member, update SSH keys, or modify sudo permissions.

### Step 1: Add New User

Create `group_vars/all.yml` or update `vars/default.yml`:
```yaml
additional_users:
  - username: deploy
    ssh_key: "{{ lookup('file', '/home/admin/.ssh/deploy_id_rsa.pub') }}"
    sudo: true
  - username: developer
    ssh_key: "{{ lookup('file', '/home/admin/.ssh/developer_id_rsa.pub') }}"
    sudo: false
```

### Step 2: Create User Management Tasks

Create custom playbook `add-user.yml`:
```yaml
---
- hosts: all
  become: true
  vars_files:
    - vars/default.yml
  tasks:
    - name: Create additional users
      ansible.builtin.user:
        name: "{{ item.username }}"
        state: present
        create_home: true
        shell: /bin/bash
        groups: "{{ 'wheel' if item.sudo else '' }}"
      loop: "{{ additional_users }}"

    - name: Deploy SSH keys for users
      ansible.builtin.authorized_key:
        user: "{{ item.username }}"
        state: present
        key: "{{ item.ssh_key }}"
      loop: "{{ additional_users }}"

    - name: Configure sudo for privileged users
      ansible.builtin.lineinfile:
        path: /etc/sudoers
        regexp: "^{{ item.username }} ALL="
        line: "{{ item.username }} ALL=(ALL) NOPASSWD: ALL"
        state: present
        validate: '/usr/sbin/visudo -cf %s'
      when: item.sudo
      loop: "{{ additional_users }}"
```

### Step 3: Execute User Addition

```bash
# Add users to all servers
ansible-playbook add-user.yml -l webservers -u organic --ask-become-pass

# Verify user creation
ansible webservers -m shell -a "id deploy" -u organic --become

# Test SSH access
ssh deploy@prod-web1
```

### Step 4: Update Existing User's SSH Key

**Scenario**: SSH key compromised or rotated

Edit `vars/default.yml`:
```yaml
# Update the SSH key path
copy_local_key: "{{ lookup('file', lookup('env','HOME') + '/.ssh/NEW_id_rsa_gitlab.pub') }}"
```

Apply the change:
```bash
# Update SSH key
ansible-playbook playbook.yml -t ssh-keys -l webservers -u organic --ask-become-pass

# Verify new key works
ssh -i ~/.ssh/NEW_id_rsa_gitlab organic@prod-web1
```

### Step 5: Remove User Access

```bash
# Remove user from servers
ansible webservers -m user -a "name=developer state=absent remove=yes" -u organic --become

# Verify removal
ansible webservers -m shell -a "id developer" -u organic --become
# Expected: id: 'developer': no such user
```

### Step 6: Add User to Docker Group

```bash
# Add existing user to docker group
ansible webservers -m user -a "name=deploy groups=docker append=yes" -u organic --become

# User must log out and back in for group membership to take effect
ansible webservers -m shell -a "id deploy" -u organic --become
# Should show docker in groups list

# Test Docker access
ssh deploy@prod-web1 "docker ps"
```

**SSH Key Rotation Workflow**:

1. Generate new SSH key pair
2. Update vars/default.yml with new public key
3. Run playbook with `-t ssh-keys`
4. Test new key access
5. Remove old key from authorized_keys
6. Update automation tools with new private key

---

## Workflow 9: Troubleshooting Failed Playbook Runs

**Scenario**: Playbook execution failed partway through.

### Step 1: Identify the Failure

```bash
# Run playbook with verbose output
ansible-playbook playbook.yml -l webservers -u organic --ask-become-pass -vv

# Look for FAILED tasks in output
# Example:
# TASK [docker : Docker Prerequisites | Install required packages] ****
# fatal: [prod-web1]: FAILED! => {"changed": false, "msg": "..."}
```

### Step 2: Common Failure Scenarios

**Scenario A: SSH Connection Failure**

Symptoms:
```
UNREACHABLE! => {"changed": false, "msg": "Failed to connect to the host via ssh"}
```

Troubleshooting:
```bash
# Test SSH connectivity manually
ssh organic@prod-web1

# Check SSH key in agent
ssh-add -l

# Add key if missing
ssh-add ~/.ssh/id_rsa_gitlab

# Verify inventory configuration
ansible webservers --list-hosts
ansible webservers -m ping -u organic
```

**Scenario B: Package Installation Failure**

Symptoms:
```
FAILED! => {"msg": "Unable to locate package docker-ce"}
```

Troubleshooting:
```bash
# SSH into server and check manually
ssh organic@prod-web1

# Update apt cache
sudo apt-get update

# Check for repository issues
sudo apt-cache policy docker-ce

# Look for held packages
sudo dpkg --get-selections | grep hold

# Clean Docker repositories and retry
ansible-playbook playbook.yml -t docker-cleanup,docker-repo -l prod-web1 -u organic --ask-become-pass
```

**Scenario C: Permission Denied**

Symptoms:
```
FAILED! => {"msg": "Missing sudo password"}
```

Troubleshooting:
```bash
# Ensure --ask-become-pass is used
ansible-playbook playbook.yml -l webservers -u organic --ask-become-pass

# Verify sudo access
ssh organic@prod-web1 "sudo whoami"

# Check sudoers configuration
ssh organic@prod-web1 "sudo cat /etc/sudoers | grep organic"
```

**Scenario D: Docker Repository Conflict**

Symptoms:
```
FAILED! => {"msg": "GPG error: The following signatures couldn't be verified"}
```

Troubleshooting:
```bash
# Run Docker cleanup tasks
ansible-playbook playbook.yml -t docker-cleanup -l prod-web1 -u organic --ask-become-pass

# Manual cleanup (if needed)
ssh organic@prod-web1
sudo rm -f /etc/apt/sources.list.d/*docker*.list
sudo rm -f /etc/apt/keyrings/docker*
sudo rm -f /etc/apt/trusted.gpg.d/docker*
sudo apt-get update

# Re-run Docker installation
ansible-playbook playbook.yml -t docker -l prod-web1 -u organic --ask-become-pass
```

### Step 3: Safe Retry Strategy

```bash
# Retry only the failed role
ansible-playbook playbook.yml -t docker -l prod-web1 -u organic --ask-become-pass

# If still failing, use verbose mode
ansible-playbook playbook.yml -t docker -l prod-web1 -u organic --ask-become-pass -vvv

# Check playbook with --start-at-task
ansible-playbook playbook.yml --start-at-task="Docker Installation | Install Docker Engine" -l prod-web1 -u organic --ask-become-pass
```

### Step 4: Check Server State

```bash
# Check disk space
ansible prod-web1 -m shell -a "df -h" -u organic --become

# Check memory
ansible prod-web1 -m shell -a "free -h" -u organic --become

# Check running services
ansible prod-web1 -m shell -a "systemctl list-units --failed" -u organic --become

# Check recent system logs
ansible prod-web1 -m shell -a "journalctl -p err -n 50" -u organic --become
```

### Step 5: Idempotency Check

```bash
# Run playbook again - should succeed if system is in partial state
ansible-playbook playbook.yml -l prod-web1 -u organic --ask-become-pass

# Verify no unwanted changes
ansible-playbook playbook.yml -l prod-web1 -u organic --ask-become-pass --check
# Should show minimal or no changes
```

**Debug Techniques**:

1. **Increase verbosity**: `-v`, `-vv`, `-vvv`, `-vvvv`
2. **Enable debug**: Set `ANSIBLE_DEBUG=1` environment variable
3. **Check logs**: `/var/log/ansible.log` (if configured)
4. **Run tasks manually**: SSH to server and replicate task
5. **Check Ansible facts**: `ansible prod-web1 -m setup | grep ansible_distribution`

---

## Workflow 10: Disaster Recovery - Server Rebuild

**Scenario**: Production server failed catastrophically, need rapid rebuild.

### Step 1: Provision New Server

**Cloud provider steps** (e.g., Azure, AWS, DigitalOcean):
1. Launch new Ubuntu 20.04/22.04/24.04 instance
2. Same or larger size as failed server
3. Note new IP address
4. Attach appropriate security groups/firewall rules
5. Ensure SSH access enabled

### Step 2: Update Inventory

Edit `inventory.ini`:
```ini
[webservers]
# Old server (failed)
# prod-web1 ansible_host=152.00.000.00 ansible_user=root ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab

# New replacement server
prod-web1-new ansible_host=152.53.137.100 ansible_user=root ansible_ssh_private_key_file=~/.ssh/id_rsa_gitlab
```

### Step 3: Verify Connectivity

```bash
# Test SSH to new server
ssh -i ~/.ssh/id_rsa_gitlab root@152.53.137.100

# Test Ansible connectivity
ansible prod-web1-new -m ping -u root -k
```

### Step 4: Rapid Provisioning

```bash
# Run full playbook on new server
ansible-playbook playbook.yml -l prod-web1-new -u root -k

# Monitor execution (8-15 minutes)
# Watch for any failures and address immediately
```

### Step 5: Restore Application Data

```bash
# SSH into new server
ssh organic@prod-web1-new

# Restore from backups (example with rsync)
sudo rsync -avz backup-server:/backups/prod-web1/latest/ /var/www/html/

# Or restore Docker volumes
docker volume create myapp_data
sudo tar -xzf /backups/myapp_data.tar.gz -C /var/lib/docker/volumes/myapp_data/_data/

# Restore database
mysql -u root -p myapp < /backups/myapp_db.sql
```

### Step 6: Start Services

```bash
# Start Docker containers
cd /opt/myapp
docker-compose up -d

# Verify services
docker ps
curl http://localhost:8080/health

# Check logs
docker-compose logs -f --tail=100
```

### Step 7: Update DNS and Networking

**DNS update**:
```bash
# Update A record
# example.com A 152.53.137.100

# Verify DNS propagation
dig example.com
nslookup example.com
```

**Load balancer update** (if applicable):
```bash
# Remove old server from load balancer
# Add new server to load balancer pool
# Verify health checks passing
```

### Step 8: Verify Functionality

```bash
# Run smoke tests
curl -I https://example.com
curl -X POST https://example.com/api/health

# Check application logs
docker logs myapp_web_1

# Monitor system resources
ssh organic@prod-web1-new "htop"
```

### Step 9: Post-Recovery Tasks

```bash
# Update monitoring
# - Update Prometheus targets
# - Update Grafana dashboards
# - Update alerting rules

# Update documentation
# - Update inventory records
# - Document incident and recovery steps
# - Update runbooks

# Verify backups
# - Ensure new server is in backup rotation
# - Test restore from new server
```

### Step 10: Decommission Old Server

```bash
# Remove from inventory
# Comment out or delete from inventory.ini

# Remove from cloud provider
# Delete instance (after confirming new server is stable)

# Update DNS records
# Remove any lingering A/AAAA records for old IP

# Clean up monitoring
# Remove old server from Prometheus, Grafana, etc.
```

**Recovery Time Objectives**:
- New server provisioning: 5-10 minutes (cloud provider)
- Ansible playbook execution: 8-15 minutes
- Data restoration: 10-60 minutes (depends on data size)
- DNS propagation: 5-30 minutes (depends on TTL)
- **Total RTO**: 30-120 minutes

**Best Practices**:
- Maintain recent backups (daily/hourly)
- Document backup locations and restore procedures
- Test disaster recovery process quarterly
- Keep inventory.ini in version control
- Have runbooks for critical applications
- Monitor backup success/failures

---

## Appendix: Quick Reference Commands

### Common Playbook Executions

```bash
# Full initial setup
ansible-playbook playbook.yml -l webservers -u root -k

# Security only
ansible-playbook playbook.yml -t security -l webservers -u organic --ask-become-pass

# Docker only
ansible-playbook playbook.yml -t docker -l webservers -u organic --ask-become-pass

# Package updates
ansible-playbook playbook.yml -t packages,upgrade -l webservers -u organic --ask-become-pass

# Reboot if needed
ansible-playbook playbook.yml -t reboot -l webservers -u organic --ask-become-pass

# Dry run (preview changes)
ansible-playbook playbook.yml -l webservers -u organic --ask-become-pass --check
```

### Ad-hoc Commands

```bash
# Ping all servers
ansible all -m ping -u organic

# Check disk space
ansible all -m shell -a "df -h" -u organic --become

# Restart service
ansible webservers -m service -a "name=docker state=restarted" -u organic --become

# Check uptime
ansible all -m shell -a "uptime" -u organic

# Gather facts
ansible all -m setup -u organic | grep ansible_distribution
```

### Debugging

```bash
# Verbose output (levels 1-4)
ansible-playbook playbook.yml -vvv

# Syntax check
ansible-playbook playbook.yml --syntax-check

# List hosts
ansible webservers --list-hosts

# Check connectivity
ansible all -m ping

# Show inventory
ansible-inventory --graph
```

---

**Last Updated**: 2025-11-12
**Maintainer**: Infrastructure Team
**Feedback**: Submit issues or improvements via pull request
