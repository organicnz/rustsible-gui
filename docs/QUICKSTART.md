# 🚀 Quick Start Guide - Tauri GUI

Get up and running in 5 minutes!

## Step 1: Install Prerequisites

### macOS
```bash
# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install node rust ansible
```

### Linux (Ubuntu/Debian)
```bash
# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install system dependencies
sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    ansible
```

## Step 2: Install Project Dependencies

```bash
cd gui-tauri
npm install
```

This will take a few minutes on first run.

## Step 3: Launch the GUI

```bash
./RUN_GUI.sh
```

Or manually:
```bash
npm run tauri dev
```

## Step 4: Configure Your Server

1. **Enter Connection Info**:
   - IP Address: Your server's IP (e.g., `192.168.1.100`)
   - SSH User: Username with sudo (default: `root`)
   - SSH Key Path: Path to your private key (e.g., `~/.ssh/id_rsa`)
   - Hostname: Optional server name

2. **Select Features**:
   - Check the boxes for features you want
   - Core features: Docker, LEMP, Fail2ban, etc.
   - Security options: System hardening, monitoring, etc.

3. **Click Launch**:
   - Click "🚀 Launch Provisioning"
   - Watch real-time output
   - Wait for completion

## Step 5: First Time Setup

### Generate SSH Key (if needed)

```bash
ssh-keygen -t ed25519 -C "your_email@example.com"
# Press Enter to accept default location (~/.ssh/id_ed25519)
```

### Copy Key to Server

```bash
ssh-copy-id -i ~/.ssh/id_ed25519.pub root@YOUR_SERVER_IP
```

### Test Connection

```bash
ssh -i ~/.ssh/id_ed25519 root@YOUR_SERVER_IP
```

## Common Issues

### "ansible-playbook not found"

Install Ansible:
```bash
# macOS
brew install ansible

# Linux
sudo apt install ansible

# Or via pip
pip3 install ansible
```

### "Permission denied (publickey)"

Your SSH key isn't set up correctly:
```bash
# Copy your key to the server
ssh-copy-id -i ~/.ssh/id_rsa.pub root@YOUR_SERVER_IP

# Or manually
cat ~/.ssh/id_rsa.pub | ssh root@YOUR_SERVER_IP "mkdir -p ~/.ssh && cat >> ~/.ssh/authorized_keys"
```

### "Failed to compile"

Update Rust:
```bash
rustup update
```

Clear cache and rebuild:
```bash
cd src-tauri
cargo clean
cd ..
npm run tauri dev
```

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Check out the [main project README](../readme.md) for Ansible details
- Explore security options for hardening your server
- Set up periodic reboots for maintenance

## Tips

- **Settings are auto-saved** - Your configuration persists between sessions
- **Real-time output** - Watch Ansible execution live in the GUI
- **Validation** - WordPress requires LEMP stack (GUI will warn you)
- **SSH keys only** - Password authentication is not supported (more secure!)

## Building for Production

When ready to create a distributable app:

```bash
./BUILD.sh
```

Find your app in:
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Linux**: `src-tauri/target/release/bundle/deb/` or `.AppImage`
- **Windows**: `src-tauri/target/release/bundle/msi/`

---

**Happy provisioning! 🎉**
