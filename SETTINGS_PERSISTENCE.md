# Settings Persistence Guide

## Overview

The Rustsible GUI automatically saves all your settings locally, so you never have to re-enter them.

## Cache Location

Settings are stored in: `~/.ansible_provisioning_cache.json`

**Important:** This file is stored in your home directory (outside the git repository) and contains sensitive information including SSH keys, passphrases, and passwords.

## What Gets Saved

All 70+ settings are automatically persisted:

### Connection Settings
- IP address / hostname
- SSH username
- Connection password (if used)
- SSH key path
- SSH key passphrase
- Server hostname/alias

### User Creation
- Create user flag
- Username
- User password

### Features
- Docker
- LEMP stack
- WordPress
- Certbot SSL
- Development tools

### Developer Tools (28 tools)
- Neovim, Micro, Zsh, Fish, Starship, Tmux
- Node.js, Claude Code, Gemini, Kiro, GitHub CLI
- btop, htop, ripgrep, fd, fzf, zoxide, direnv
- ranger, duf, ncdu, lnav, tldr, lazygit, uv
- jq, gping, nmap, autossh, bat, eza

### Security Settings (17 options)
- Fail2ban, AppArmor, Rootkit detection
- File integrity (AIDE), Audit logging
- 2FA options (TOTP, FIDO2, Duo)
- Backups, USB restrictions
- IPv6 disable, Suricata IDS
- System hardening options

### Maintenance
- Swap configuration
- Cron jobs
- Periodic reboot settings
- Reboot schedule

## How It Works

Settings are saved automatically when you:
1. Type in any text field
2. Check/uncheck any checkbox
3. Select from dropdowns
4. Browse for files
5. Switch between tabs

No manual save button needed!

## Security Considerations

⚠️ **The cache file contains sensitive data:**
- SSH key passphrases
- User passwords
- Connection passwords
- Server IP addresses

### Recommendations:

1. **Use disk encryption** (FileVault on macOS, LUKS on Linux)
2. **Encrypt your home directory**
3. **Set stricter file permissions:**
   ```bash
   chmod 600 ~/.ansible_provisioning_cache.json
   ```
4. **Never commit this file to git** (it's in your home directory, so it won't be)
5. **Don't share this file** with others

## Usage

### First Time Setup
1. Open the GUI: `make gui`
2. Fill in your connection details
3. Select your desired features
4. Close the GUI

### Subsequent Uses
1. Open the GUI: `make gui`
2. All your settings are automatically loaded!
3. Make any changes you need
4. Changes are saved automatically

## Troubleshooting

### Settings Not Loading

1. **Check if cache file exists:**
   ```bash
   ls -la ~/.ansible_provisioning_cache.json
   ```

2. **Verify JSON is valid:**
   ```bash
   python3 -m json.tool ~/.ansible_provisioning_cache.json > /dev/null && echo "Valid" || echo "Invalid"
   ```

3. **Check file permissions:**
   ```bash
   ls -l ~/.ansible_provisioning_cache.json
   ```
   Should be readable by you.

4. **Start fresh (delete cache):**
   ```bash
   rm ~/.ansible_provisioning_cache.json
   make gui
   ```

### Settings Not Saving

1. **Check home directory is writable:**
   ```bash
   touch ~/test_write && rm ~/test_write && echo "OK" || echo "FAIL"
   ```

2. **Check disk space:**
   ```bash
   df -h ~
   ```

3. **Check for errors in GUI output**

## Example Cache Structure

```json
{
  "ip_address": "192.168.1.100",
  "ssh_user": "ubuntu",
  "ssh_key_path": "~/.ssh/id_rsa",
  "ssh_key_passphrase": "your-passphrase-here",
  "hostname": "myserver",
  "create_user": true,
  "added_user": "myuser",
  "docker": true,
  "lemp": false,
  "devtools": true,
  "install_neovim": true,
  "fail2ban": true,
  "swap": true,
  "cron_jobs": true,
  ...
}
```

## Git Security

The cache file is **NOT** tracked by git because:
1. It's stored in your home directory (`~/.ansible_provisioning_cache.json`)
2. It's outside the repository
3. Even if copied to the repo, `.gitignore` excludes `*_cache.json` patterns

## Implementation Details

Auto-save is implemented in:
- `render_connection()` - Connection settings
- `render_features()` - Feature selections
- `render_security()` - Security options
- `render_maintenance()` - Maintenance settings

Each function checks for changes and calls `save_cache()` automatically.

## Benefits

✅ Never re-enter connection details
✅ Never re-enter SSH passphrases
✅ Never re-configure feature selections
✅ Consistent settings across sessions
✅ Quick provisioning workflow
✅ No manual save/load needed

## Notes

- Cache is created on first use
- Cache is updated on every change
- Cache is loaded on GUI startup
- Cache persists across reboots
- Cache is user-specific (stored in home directory)
