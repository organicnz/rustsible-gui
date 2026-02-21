# Rustsible GUI - Ansible Provisioning Dashboard

A pure Rust desktop GUI for managing Ansible server provisioning with automatic process cleanup.

## Quick Start

Simply run the binary:

```bash
./rustsible-gui
```

The GUI will:
1. Automatically clean up any previous instances
2. Launch the Ansible provisioning dashboard
3. Provide an interactive interface for server configuration

## Features

✅ **Automatic Cleanup** - Kills old zombie processes on startup  
✅ **Interactive Dashboard** - Point-and-click server provisioning  
✅ **Pure Rust** - Fast, safe, and efficient  
✅ **Cross-platform** - Works on macOS, Linux, and Windows  
✅ **Real-time Output** - See Ansible playbook execution live  

## Usage

### Basic Launch

```bash
./rustsible-gui
```

### From GUI-EGUI Directory

```bash
cd gui-egui
cargo run
```

### Build Release Binary

```bash
cd gui-egui
cargo build --release
cp target/release/rustsible-gui ../
```

## Configuration

The GUI provides an interactive form for:

- **Target Server**: IP address and hostname
- **SSH Credentials**: User and key path
- **Services**: Docker, LEMP, WordPress, Certbot
- **Security**: Fail2ban, system hardening, AppArmor
- **Monitoring**: Rootkit detection, file integrity, log monitoring
- **Automation**: Cron jobs, periodic reboot scheduling

## Technical Details

### Built With

- **eframe/egui** - Immediate mode GUI framework
- **tokio** - Async runtime for Ansible execution
- **sysinfo** - Cross-platform process management
- **signal-hook** - Unix signal handling

### Process Management

The application includes sophisticated process cleanup:

1. **Startup Cleanup** (6 phases):
   - Phase 0: Kill parent processes
   - Phase 1: Graceful SIGTERM
   - Phase 2: Force SIGKILL
   - Phase 3: System-wide killall/pkill
   - Phase 4: macOS launchctl cleanup
   - Phase 5: Process group termination

2. **Graceful Shutdown**:
   - Joins background threads
   - Kills ansible-playbook processes
   - Shuts down tokio runtime
   - Handles SIGTERM/SIGINT signals

### Known Limitations

**macOS Zombie Processes**: Due to an egui/eframe + macOS WindowServer interaction, forcefully terminated processes (kill -9) may leave unkillable zombie processes in "UE" (Uninterruptible, Exiting) state. These:

- Consume **zero CPU and memory**
- Are **automatically cleaned on next launch**
- Are **reaped by macOS eventually**
- **Only occur on abnormal termination** (testing, force quit)
- **Do not occur on normal window close**

**Workarounds**:
- Close the window normally (no zombies created)
- Launch via `./rustsible-gui` (automatic cleanup)
- Reboot system to clear all zombies
- Ignore them (harmless, just take up PID numbers)

## File Structure

```
ansible-ubuntu/
├── rustsible-gui          # Main executable
├── gui-egui/              # Source code
│   ├── src/
│   │   └── main.rs        # Application code
│   ├── Cargo.toml         # Dependencies
│   └── target/            # Build artifacts
├── playbook.yml           # Ansible playbook
├── inventory.ini          # Ansible inventory
└── roles/                 # Ansible roles
```

## Development

### Prerequisites

- Rust 1.70+ (2021 edition)
- Ansible installed and in PATH
- SSH key configured

### Build & Run

```bash
cd gui-egui
cargo run              # Development build
cargo run --release    # Optimized build
cargo build --release  # Build only
```

### Testing

```bash
# Launch and test normal shutdown (clean exit)
./rustsible-gui
# Close window normally - no zombie

# Test aggressive cleanup
./rustsible-gui        # Launches with cleanup output
```

## Documentation

- `gui-egui/ZOMBIE_PROCESS_STATUS.md` - Complete technical analysis
- `docs/AGGRESSIVE_CLEANUP.md` - Cleanup implementation details
- `docs/PROCESS_CLEANUP.md` - Process management architecture
- `CLAUDE.md` - Development guidelines

## Deployment

1. **Build release binary**:
   ```bash
   cd gui-egui
   cargo build --release
   cp target/release/rustsible-gui ../
   ```

2. **Deploy to server/workstation**:
   ```bash
   scp rustsible-gui user@host:/usr/local/bin/
   # or
   mv rustsible-gui /usr/local/bin/
   ```

3. **Run**:
   ```bash
   rustsible-gui
   ```

## Troubleshooting

### "No zombies found" but ps shows processes

The zombies are in UE (Uninterruptible, Exiting) state and cannot be killed by any user-space method. Just reboot or ignore them.

### GUI doesn't launch

Ensure you have:
- X11/XQuartz installed (macOS)
- Wayland or X11 running (Linux)
- Display environment set

### Ansible playbook fails

Check:
- SSH key path is correct
- Target server is accessible
- Ansible is installed (`ansible --version`)
- Playbook syntax is valid

## License

See main project LICENSE file.

## Contributing

This is part of the ansible-ubuntu project. See main README for contribution guidelines.
