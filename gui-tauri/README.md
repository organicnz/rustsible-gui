# Ansible Provisioning GUI (Rust + Tauri)

Modern, cross-platform desktop application for Ubuntu server provisioning built with **Rust** (backend) and **Svelte** (frontend) using the Tauri framework.

## Features

✨ **Native Performance** - Rust backend for blazing-fast execution
🎨 **Modern Dark UI** - Beautiful, responsive interface with dark theme
💾 **Auto-save** - Configuration automatically saved and restored
🖥️ **Cross-platform** - Works on macOS, Windows, and Linux
📱 **Native Scrolling** - Proper trackpad/mousewheel support on all platforms
🔒 **Type-safe** - TypeScript frontend with Rust backend
⚡ **Small Binary** - ~3MB app size (vs 100MB+ for Electron)

## Architecture

```
gui-tauri/
├── src/                    # Svelte frontend
│   └── routes/
│       └── +page.svelte   # Main UI component
├── src-tauri/             # Rust backend
│   └── src/
│       ├── main.rs        # Entry point
│       └── lib.rs         # Tauri commands & logic
└── package.json           # Node dependencies
```

### Tech Stack

- **Frontend**: Svelte 5 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.0
- **State Management**: Svelte runes ($state, $effect)
- **Styling**: CSS-in-Svelte (no external CSS framework)

## Prerequisites

- **Rust** 1.83+ ([Install Rust](https://rustup.rs/))
- **Node.js** 20+ ([Install Node](https://nodejs.org/))
- **npm** 10+

## Installation

1. Navigate to the gui-tauri directory:
```bash
cd gui-tauri
```

2. Install dependencies:
```bash
npm install
```

3. Run in development mode:
```bash
npm run tauri dev
```

## Building

### Development Build
```bash
npm run tauri dev
```

### Production Build
```bash
npm run tauri build
```

The compiled application will be in `src-tauri/target/release/`.

### Platform-Specific Builds

**macOS:**
- Creates `.app` bundle in `src-tauri/target/release/bundle/macos/`
- Creates `.dmg` installer

**Windows:**
- Creates `.exe` in `src-tauri/target/release/`
- Creates `.msi` installer

**Linux:**
- Creates AppImage, deb, and rpm packages

## Configuration

### Rust Backend (src-tauri/src/lib.rs)

The backend implements these Tauri commands:

- `load_cache()` - Loads saved configuration from `~/.ansible_provisioning_cache.json`
- `save_cache(config)` - Saves configuration to disk
- `launch_provisioning(config)` - Executes ansible-playbook with provided config
- `validate_ip(ip)` - Validates IP address format

### Frontend (src/routes/+page.svelte)

The UI automatically:
- Loads cached configuration on mount
- Saves configuration on every change (using `$effect`)
- Provides real-time validation
- Shows success/error results

## Cache Location

Configuration is saved to:
- **macOS/Linux**: `~/.ansible_provisioning_cache.json`
- **Windows**: `%USERPROFILE%\.ansible_provisioning_cache.json`

## Ansible Integration

The app executes:
```bash
ansible-playbook ../playbook.yml \
  -i ../inventory.ini \
  -u <ssh_user> \
  -k \
  -e "ip_address=<ip> install_docker=<bool> ..."
```

## Development

### Hot Reload

The development server supports hot reload:
- Frontend changes: Instant hot reload (Vite HMR)
- Backend changes: Requires restart (Rust recompilation)

### Debugging

**Frontend:**
```bash
# Open DevTools in the app
npm run tauri dev
# Then press Cmd+Option+I (macOS) or Ctrl+Shift+I (Windows/Linux)
```

**Backend:**
```bash
# View Rust logs
RUST_LOG=debug npm run tauri dev
```

### Adding Dependencies

**Frontend:**
```bash
npm install <package>
```

**Backend:**
```toml
# Edit src-tauri/Cargo.toml
[dependencies]
your-crate = "1.0"
```

## Comparison: Python vs Rust GUI

| Feature | Python (Tkinter) | Rust (Tauri) |
|---------|------------------|--------------|
| **Startup Time** | ~500ms | ~50ms |
| **Memory Usage** | ~80MB | ~30MB |
| **Bundle Size** | ~50MB | ~3MB |
| **macOS Scrolling** | ❌ Broken | ✅ Native |
| **Hot Reload** | ❌ No | ✅ Yes |
| **Type Safety** | ⚠️ Partial | ✅ Full |
| **Performance** | 🐌 Slow | 🚀 Fast |
| **Development** | Easy | Moderate |
| **Cross-platform** | ⚠️ OK | ✅ Excellent |

## Troubleshooting

### "command not found: cargo"
Install Rust: https://rustup.rs/

### "npm ERR! peer dependency warnings"
Ignore or run `npm install --legacy-peer-deps`

### macOS: "App is damaged"
```bash
xattr -cr gui-tauri.app
```

### Linux: Missing dependencies
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

## Contributing

1. Frontend changes: Edit `src/routes/+page.svelte`
2. Backend changes: Edit `src-tauri/src/lib.rs`
3. Test: `npm run tauri dev`
4. Build: `npm run tauri build`

## License

Same as parent project (ansible-ubuntu).

## Resources

- [Tauri Documentation](https://tauri.app/)
- [Svelte Documentation](https://svelte.dev/)
- [Rust Documentation](https://www.rust-lang.org/)

---

**Built with ❤️ using Tauri + Svelte + Rust**
