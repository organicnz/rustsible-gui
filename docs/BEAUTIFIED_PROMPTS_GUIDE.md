# Beautified Interactive Prompts Guide

## Overview

This guide explains the beautification enhancements made to the Ansible playbook's interactive prompts and output displays.

## Understanding vars_prompt Limitations

### What vars_prompt CANNOT Do

Ansible's `vars_prompt` has several inherent limitations:

1. **No Native Checkboxes**: Only supports text input - no GUI elements, checkboxes, or radio buttons
2. **No Color in Prompts**: ANSI color codes in prompt text are displayed as raw escape sequences
3. **No Interactive Selection**: No arrow key navigation, multi-select, or visual feedback during prompting
4. **Linear Input Only**: Each prompt must be answered sequentially; cannot navigate back
5. **No Real-time Preview**: Cannot show a live preview of selections as you type

### What You CAN Do

You can beautify the **output after prompts** using:
- Unicode symbols (✓, ✗, ●, ○, ▸, ⚠, etc.)
- ANSI color codes in debug messages
- Box-drawing characters for borders
- Conditional styling based on user selections
- Formatted summary displays

## Visual Elements Used

### Unicode Symbols

| Symbol | Usage | Description |
|--------|-------|-------------|
| ✓ | Enabled features | Green checkmark for selected items |
| ✗ | Disabled features | Red X for unselected items |
| ● | Active status | Solid circle for enabled state |
| ○ | Inactive status | Hollow circle for disabled state |
| ▸ | List items | Right-pointing arrow for details |
| ⚠ | Warnings | Warning triangle for alerts |

### Box-Drawing Characters

```
┌─────────────────┐  Single-line box
│  Content here   │
└─────────────────┘

╔═════════════════╗  Double-line box (headers)
║  Important!     ║
╚═════════════════╝
```

### ANSI Color Codes

```
\033[1;32m  = Bold Green (success, enabled)
\033[1;31m  = Bold Red (error, disabled)
\033[1;33m  = Bold Yellow (warning, info)
\033[1;36m  = Bold Cyan (labels, highlights)
\033[1;90m  = Bold Gray (disabled text)
\033[0m     = Reset formatting
```

## Example Output

### Phase 1: Connection Information Display

```
┌─────────────────────────────────────────────────────────────┐
│           CONNECTION INFORMATION                            │
└─────────────────────────────────────────────────────────────┘
  Target Server: 192.168.1.100
  SSH User:      root
  SSH Key:       ~/.ssh/id_rsa_gitlab
```

### Phase 2: Selected Features Summary

```
┌─────────────────────────────────────────────────────────────┐
│           SELECTED FEATURES                                 │
└─────────────────────────────────────────────────────────────┘
  ✓ Fail2ban Intrusion Prevention
  ✓ Docker & Docker Compose
  ✗ LEMP Stack (Nginx, MySQL, PHP)
  ✓ Swap Memory Configuration
  ✓ Automated Cron Jobs
  ✗ Development Tools
  ✗ WordPress CMS
  ✗ Certbot SSL/TLS Certificates

  ▸ Total features enabled: 4
```

### Phase 3: Provisioning Start

```
╔═════════════════════════════════════════════════════════════╗
║         UBUNTU SERVER PROVISIONING STARTED                  ║
╚═════════════════════════════════════════════════════════════╝
  ▸ Target Host:  192.168.1.100
  ▸ OS:           Ubuntu 22.04
  ▸ Architecture: x86_64

┌─────────────────────────────────────────────────────────────┐
│           ACTIVE FEATURES FOR THIS RUN                      │
└─────────────────────────────────────────────────────────────┘
  ● ENABLED   Fail2ban Intrusion Prevention
  ● ENABLED   Docker & Docker Compose
  ○ DISABLED  LEMP Stack
  ● ENABLED   Swap Memory
  ● ENABLED   Cron Jobs
  ○ DISABLED  Development Tools
  ○ DISABLED  WordPress
  ○ DISABLED  Certbot
```

### Phase 4: Completion Status

```
╔═════════════════════════════════════════════════════════════╗
║         PROVISIONING COMPLETED SUCCESSFULLY                 ║
╚═════════════════════════════════════════════════════════════╝
✓  System Status: No reboot required
```

Or with reboot required:

```
╔═════════════════════════════════════════════════════════════╗
║         PROVISIONING COMPLETED SUCCESSFULLY                 ║
╚═════════════════════════════════════════════════════════════╝
⚠  WARNING: Reboot is REQUIRED (kernel or system updates)
  ▸ Run with tag -t reboot to reboot now
```

## Color Scheme

The beautified output uses a consistent color scheme:

- **Green (✓, ●)**: Success, enabled features, positive status
- **Red (✗)**: Disabled features, errors
- **Yellow (⚠, ▸)**: Warnings, information, counts
- **Cyan (▸)**: Labels, field names, highlights
- **Gray (○)**: Disabled/inactive status

## Implementation Details

### Jinja2 Conditional Rendering

The feature display uses Jinja2 conditionals to show different symbols/colors:

```yaml
- name: Display selected features with visual indicators
  debug:
    msg: "{{ item }}"
  loop:
    - "  {{ '\033[1;32m✓\033[0m' if (prompt_enable_fail2ban | lower in ['yes', 'y', 'true']) else '\033[1;31m✗\033[0m' }} Fail2ban"
```

### Dynamic Feature Counting

The total enabled features count is calculated dynamically:

```yaml
vars:
  feature_count: "{{ [prompt_enable_fail2ban, prompt_install_docker, ...] | select('match', '^(yes|y|true)$') | list | length }}"
```

## Alternative Approaches

### 1. External Prompting Script

For true checkbox/menu functionality, create a separate Python/Bash script:

```python
#!/usr/bin/env python3
import inquirer

questions = [
    inquirer.Checkbox('features',
                     message="Select features to enable",
                     choices=[
                         'Fail2ban',
                         'Docker',
                         'LEMP Stack',
                         'Swap Memory',
                         'Cron Jobs',
                         'Dev Tools',
                         'WordPress',
                         'Certbot'
                     ])
]

answers = inquirer.prompt(questions)
# Generate ansible extra vars
for feature in answers['features']:
    print(f"enable_{feature.lower().replace(' ', '_')}=yes")
```

Then call it before ansible-playbook:

```bash
extra_vars=$(./prompt_features.py)
ansible-playbook playbook.yml -e "$extra_vars"
```

### 2. Ansible Tower/AWX Surveys

If using Ansible Tower or AWX, you can create visual surveys with:
- Dropdown menus
- Checkboxes
- Radio buttons
- Text fields with validation
- Multi-select lists

### 3. Interactive Shell Wrapper

Create a bash script with `dialog` or `whiptail`:

```bash
#!/bin/bash
features=$(whiptail --checklist "Select features:" 20 60 8 \
    "Fail2ban" "Intrusion Prevention" ON \
    "Docker" "Container Platform" ON \
    "LEMP" "Web Stack" OFF \
    "Swap" "Memory Management" ON \
    "Cron" "Automation" ON \
    "DevTools" "Development" OFF \
    "WordPress" "CMS" OFF \
    "Certbot" "SSL/TLS" OFF \
    3>&1 1>&2 2>&3)

# Convert to ansible vars
# ... process $features and call ansible-playbook
```

## Testing Your Beautified Output

To see the beautified output in action:

```bash
# Run the playbook normally
ansible-playbook playbook.yml

# The prompts will appear (uncolored, sequential)
# After all prompts, you'll see the beautified summary with colors and symbols
```

To test without connecting to a server, you can add `--check` mode:

```bash
ansible-playbook playbook.yml --check
```

## Terminal Compatibility

The beautification works in most modern terminals:

- **macOS**: Terminal.app, iTerm2 (full support)
- **Linux**: gnome-terminal, konsole, xterm (full support)
- **Windows**: Windows Terminal (full support), PowerShell (partial), CMD (no colors)
- **SSH**: Works over SSH connections with terminal support

## Customization Examples

### Change Color Scheme

Replace color codes in the playbook:

```yaml
# From green to blue for enabled features
'\033[1;32m'  # Green
# to
'\033[1;34m'  # Blue
```

### Use Different Symbols

```yaml
# From checkmarks to arrows
'✓'  # Checkmark
# to
'▶'  # Right arrow
```

### Add More Details

```yaml
- name: Display feature details
  debug:
    msg:
      - "  ✓ Docker & Docker Compose"
      - "    └─ Version: {{ docker_version | default('latest') }}"
      - "    └─ Users: {{ docker_users | join(', ') }}"
```

## Best Practices

1. **Keep Prompts Simple**: Use plain text in `vars_prompt` (no colors)
2. **Beautify Output**: Apply colors/symbols in debug tasks after prompts
3. **Test Terminal Support**: Verify output in your target terminals
4. **Provide Alternatives**: Include plain text fallback for CI/CD environments
5. **Document Color Meanings**: Help users understand the visual indicators
6. **Use Consistent Symbols**: Maintain symbol meanings across the playbook
7. **Avoid Overuse**: Don't overload with too many colors/symbols

## Environment Variable for Color Control

To disable colors (for CI/CD or log files):

```bash
export ANSIBLE_NOCOLOR=1
ansible-playbook playbook.yml
```

Or use the `--nocolor` flag:

```bash
ansible-playbook playbook.yml --nocolor
```

## Future Enhancements

Potential improvements for even better UX:

1. **Progress Bars**: Show role execution progress
2. **Time Estimates**: Display estimated completion time
3. **Role Summaries**: Show detailed success/failure for each role
4. **Diff Previews**: Show configuration changes before applying
5. **Interactive Confirmation**: Ask for confirmation before destructive changes
6. **Log Beautification**: Color-code different log levels in verbose output

## Conclusion

While Ansible's `vars_prompt` is limited to basic text input, the beautified output provides:

- Clear visual feedback on selections
- Professional appearance
- Easy status identification
- Improved user experience
- Better readability in logs

The key is accepting the limitation of prompts themselves and focusing on making the output after prompts as clear and beautiful as possible.
