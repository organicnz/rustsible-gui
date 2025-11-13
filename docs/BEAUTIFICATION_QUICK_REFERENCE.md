# Beautification Quick Reference

## Symbol Cheat Sheet

```
✓  Green checkmark    = Feature selected/enabled
✗  Red X              = Feature not selected/disabled
●  Solid circle       = Active/enabled status
○  Hollow circle      = Inactive/disabled status
▸  Right arrow        = Detail/info marker
⚠  Warning triangle   = Warning/attention needed
```

## Color Code Reference

```yaml
# In Jinja2 templates, wrap text with these codes:

\033[1;32m TEXT \033[0m  # Bold Green (success, enabled)
\033[1;31m TEXT \033[0m  # Bold Red (error, failure)
\033[1;33m TEXT \033[0m  # Bold Yellow (warning, info)
\033[1;36m TEXT \033[0m  # Bold Cyan (labels, highlights)
\033[1;90m TEXT \033[0m  # Bold Gray (disabled)
\033[1m    TEXT \033[0m  # Bold White
\033[0m                  # Reset to default
```

## Box Drawing Characters

```
Single-line boxes:
┌ ┐ └ ┘ ─ │ ├ ┤ ┬ ┴ ┼

Double-line boxes:
╔ ╗ ╚ ╝ ═ ║ ╠ ╣ ╦ ╩ ╬

Example usage:
┌─────────────────┐
│  Content        │
└─────────────────┘

╔═════════════════╗
║  Header         ║
╚═════════════════╝
```

## Common Patterns

### Conditional Symbol Display

```yaml
# Show checkmark if enabled, X if disabled
"{{ '\033[1;32m✓\033[0m' if (variable | bool) else '\033[1;31m✗\033[0m' }}"

# Show green circle if enabled, gray circle if disabled
"{{ '\033[1;32m●\033[0m \033[1;32mENABLED\033[0m' if (variable | bool) else '\033[1;90m○\033[0m \033[1;90mDISABLED\033[0m' }}"
```

### Colored Labels

```yaml
# Cyan label with yellow value
"  \033[1;36m▸\033[0m Label: \033[1;33m{{ value }}\033[0m"

# Green success message
"  \033[1;32m✓\033[0m Success: \033[1;32m{{ message }}\033[0m"

# Yellow warning
"  \033[1;33m⚠\033[0m Warning: \033[1;33m{{ warning_text }}\033[0m"
```

### Header Boxes

```yaml
# Single-line header
msg:
  - ""
  - "┌─────────────────────────────────────────┐"
  - "│  SECTION TITLE                          │"
  - "└─────────────────────────────────────────┘"

# Double-line header (emphasis)
msg:
  - ""
  - "╔═════════════════════════════════════════╗"
  - "║  IMPORTANT SECTION                      ║"
  - "╚═════════════════════════════════════════╝"
```

### Status Messages

```yaml
# Success
"{{ '\033[1;32m✓\033[0m  Status: \033[1;32mCompleted successfully\033[0m' }}"

# Warning
"{{ '\033[1;33m⚠\033[0m  Warning: \033[1;33mAction required\033[0m' }}"

# Error
"{{ '\033[1;31m✗\033[0m  Error: \033[1;31mOperation failed\033[0m' }}"

# Info
"{{ '\033[1;36m▸\033[0m  Info: \033[1;36mAdditional details\033[0m' }}"
```

## Testing Colors

To test if your terminal supports colors:

```bash
# Test basic colors
echo -e "\033[1;32mGreen\033[0m \033[1;31mRed\033[0m \033[1;33mYellow\033[0m \033[1;36mCyan\033[0m"

# Test symbols
echo -e "✓ ✗ ● ○ ▸ ⚠"

# Test boxes
echo "┌──────┐"
echo "│ Test │"
echo "└──────┘"
```

## Disabling Colors

When colors are not desired (CI/CD, log files):

```bash
# Environment variable
export ANSIBLE_NOCOLOR=1

# Or command-line flag
ansible-playbook playbook.yml --nocolor
```

## Copy-Paste Templates

### Feature List Display

```yaml
- name: Display features
  debug:
    msg: "{{ item }}"
  loop:
    - "  {{ '\033[1;32m✓\033[0m' if feature1 else '\033[1;31m✗\033[0m' }} Feature 1"
    - "  {{ '\033[1;32m✓\033[0m' if feature2 else '\033[1;31m✗\033[0m' }} Feature 2"
```

### Status Display

```yaml
- name: Show status
  debug:
    msg: "{{ item }}"
  loop:
    - "  {{ '\033[1;32m●\033[0m \033[1;32mENABLED\033[0m' if enabled else '\033[1;90m○\033[0m \033[1;90mDISABLED\033[0m' }}  Service Name"
```

### Info Block

```yaml
- name: Display info
  debug:
    msg:
      - "  \033[1;36m▸\033[0m Server: \033[1;33m{{ ansible_hostname }}\033[0m"
      - "  \033[1;36m▸\033[0m OS: \033[1;33m{{ ansible_distribution }}\033[0m"
```

## Best Practices

1. Always reset colors with `\033[0m` after colored text
2. Use bold (`\033[1;XXm`) for better visibility
3. Keep box widths consistent (60-65 characters)
4. Use single-line boxes for sections, double for major headers
5. Limit colors to 3-4 per screen for readability
6. Test output in multiple terminals
7. Provide plain-text alternatives for scripted execution

## Unicode Support

If Unicode symbols don't display correctly:

```yaml
# ASCII alternatives
# Instead of: ✓ ✗ ● ○ ▸ ⚠
# Use:       [✓] [X] [*] [ ] > [!]

Example:
"  {{ '[✓]' if enabled else '[X]' }} Feature Name"
```

## Terminal Compatibility Matrix

| Terminal | Colors | Unicode | Box Drawing |
|----------|--------|---------|-------------|
| macOS Terminal | Yes | Yes | Yes |
| iTerm2 | Yes | Yes | Yes |
| gnome-terminal | Yes | Yes | Yes |
| Windows Terminal | Yes | Yes | Yes |
| PowerShell | Yes | Partial | Partial |
| CMD | No | No | No |

## Troubleshooting

**Problem**: Colors show as `[1;32m` text
- **Solution**: Your terminal doesn't support ANSI codes. Use `--nocolor`

**Problem**: Unicode symbols show as `?` or boxes
- **Solution**: Terminal font doesn't support Unicode. Switch to UTF-8 font

**Problem**: Box lines are broken or misaligned
- **Solution**: Use monospace font or ASCII alternatives

**Problem**: Colors work locally but not over SSH
- **Solution**: Ensure `TERM` environment variable is set correctly on remote

## Additional Resources

- [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
- [Unicode Box Drawing](https://en.wikipedia.org/wiki/Box-drawing_character)
- [Ansible Color Output](https://docs.ansible.com/ansible/latest/reference_appendices/config.html#ansible-nocolor)
