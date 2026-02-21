# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a cross-platform desktop application built with **Tauri**, **Svelte**, and **Rust**, providing a modern, secure, and performant native desktop experience. The project leverages Rust's safety and performance for the backend, Svelte's reactive UI framework for the frontend, and Tauri's lightweight webview for cross-platform deployment.

**Key Capabilities:**
- Cross-platform desktop application (Windows, macOS, Linux)
- Rust backend with type safety and memory safety guarantees
- Svelte frontend with reactive, component-based UI
- Tauri's lightweight webview (no Electron overhead)
- Native system integration and file access
- Small binary size with optimized builds (typically 3-10 MB)
- Secure inter-process communication between frontend and backend
- Auto-update capabilities for seamless deployment

**Technology Stack:**
- **Backend**: Rust with Tauri
- **Frontend**: Svelte 4/5 with TypeScript
- **Build Tool**: Vite
- **Styling**: CSS/SCSS or Tailwind CSS
- **State Management**: Svelte stores
- **Testing**: Vitest + Svelte Testing Library (frontend), cargo test (backend)

## Critical Configuration Files

### Tauri Configuration
- `src-tauri/tauri.conf.json`: Main Tauri configuration (app metadata, build settings, security policies)
- `src-tauri/Cargo.toml`: Rust dependencies and project metadata
- `src-tauri/src/main.rs`: Rust backend entry point with Tauri commands
- `src-tauri/build.rs`: Build script for compile-time tasks

### Frontend Configuration (Svelte)
- `package.json`: Frontend dependencies and build scripts
- `vite.config.ts`: Vite configuration with Svelte plugin
- `svelte.config.js`: Svelte compiler options
- `tsconfig.json`: TypeScript compiler configuration
- `index.html`: Application entry point
- `src/App.svelte`: Root Svelte component

### Project Structure
```
.
├── src/                          # Svelte frontend source code
│   ├── lib/                      # Reusable components and utilities
│   │   ├── components/           # Svelte components
│   │   ├── stores/              # Svelte stores for state management
│   │   ├── utils/               # Utility functions
│   │   └── types/               # TypeScript type definitions
│   ├── routes/                   # SvelteKit routes (if using SvelteKit)
│   ├── App.svelte               # Root component
│   ├── main.ts                  # Frontend entry point
│   └── app.css                  # Global styles
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Tauri app initialization
│   │   ├── commands.rs          # Backend command handlers
│   │   ├── state.rs             # Application state management
│   │   ├── events.rs            # Event emitters
│   │   └── lib.rs               # Shared library code
│   ├── Cargo.toml               # Rust dependencies
│   ├── tauri.conf.json          # Tauri configuration
│   └── icons/                   # Application icons
├── public/                       # Static assets
├── dist/                         # Build output (generated)
└── static/                       # Static files served by Vite
```

## Tauri Configuration Reference

### tauri.conf.json Key Sections

**Build Configuration:**
```json
{
  "build": {
    "distDir": "../dist",
    "devPath": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  }
}
```

**Application Metadata:**
- `package.productName`: Application display name
- `package.version`: Semantic version (synced with Cargo.toml)
- `tauri.bundle.identifier`: Unique app identifier (e.g., `com.yourcompany.appname`)
- `tauri.bundle.icon`: Icon paths for different platforms

**Security Configuration:**
- `tauri.allowlist`: Fine-grained permissions for Tauri APIs
- `tauri.security.csp`: Content Security Policy for webview
- `tauri.windows.*.fileDropEnabled`: Drag-and-drop file handling

**Window Configuration:**
```json
{
  "tauri": {
    "windows": [{
      "title": "App Name",
      "width": 1200,
      "height": 800,
      "resizable": true,
      "fullscreen": false,
      "decorations": true,
      "transparent": false
    }]
  }
}
```

## Common Commands

### Development Workflow

**Start Development Server:**
```bash
# Runs Vite dev server + Rust backend with hot reload
npm run tauri dev

# Or using cargo directly
cd src-tauri && cargo tauri dev
```

**Build for Production:**
```bash
# Creates optimized production build for current platform
npm run tauri build

# Build with specific features
cargo tauri build --features custom-feature

# Build for release with debugging symbols stripped
cargo tauri build --release
```

**Frontend Only Development:**
```bash
# Start Vite dev server without Tauri (useful for UI work)
npm run dev

# Build frontend only
npm run build
```

### Testing

**Run Rust Tests:**
```bash
cd src-tauri
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with code coverage
cargo tarpaulin --out Html
```

**Run Svelte Frontend Tests:**
```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run in watch mode
npm run test:watch

# Run specific test file
npm test -- ComponentName
```

**Lint and Format:**
```bash
# Rust formatting and linting
cd src-tauri
cargo fmt --check
cargo clippy -- -D warnings

# Svelte/TypeScript linting
npm run lint
npm run format

# Check Svelte component syntax
npm run check
```

### Platform-Specific Builds

**macOS:**
```bash
# Universal binary (Apple Silicon + Intel)
npm run tauri build -- --target universal-apple-darwin

# Code signing (requires Apple Developer account)
npm run tauri build -- --bundles app,dmg

# Notarize for macOS distribution
npm run tauri build -- --sign
```

**Windows:**
```bash
# Build MSI installer
npm run tauri build -- --bundles msi

# Build NSIS installer
npm run tauri build -- --bundles nsis

# Build both
npm run tauri build -- --bundles msi,nsis
```

**Linux:**
```bash
# Build AppImage
npm run tauri build -- --bundles appimage

# Build .deb package (Debian/Ubuntu)
npm run tauri build -- --bundles deb

# Build .rpm package (Fedora/RHEL)
npm run tauri build -- --bundles rpm
```

## Architecture Patterns

### Tauri Commands (IPC)

Commands are the primary way for Svelte frontend to communicate with the Rust backend:

**Rust Backend (src-tauri/src/commands.rs):**
```rust
#[tauri::command]
async fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// Register in main.rs
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Svelte Frontend Invocation:**
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  let greeting = '';
  let fileContent = '';

  async function loadGreeting() {
    greeting = await invoke<string>('greet', { name: 'World' });
  }

  async function loadFile() {
    try {
      fileContent = await invoke<string>('read_file', {
        path: '/path/to/file.txt'
      });
    } catch (error) {
      console.error('Failed to read file:', error);
    }
  }

  onMount(() => {
    loadGreeting();
  });
</script>

<main>
  <h1>{greeting}</h1>
  <button on:click={loadFile}>Load File</button>
  <pre>{fileContent}</pre>
</main>
```

### State Management with Svelte Stores

**Rust State Management:**
```rust
// src-tauri/src/state.rs
use tauri::State;
use std::sync::Mutex;

pub struct AppState {
    pub counter: Mutex<i32>,
    pub user_data: Mutex<Option<String>>,
}

// src-tauri/src/commands.rs
#[tauri::command]
fn increment(state: State<AppState>) -> i32 {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    *counter
}

#[tauri::command]
fn get_counter(state: State<AppState>) -> i32 {
    *state.counter.lock().unwrap()
}

// src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .manage(AppState {
            counter: Mutex::new(0),
            user_data: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![increment, get_counter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Svelte Store Pattern (src/lib/stores/counter.ts):**
```typescript
import { writable, derived } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';

function createCounter() {
  const { subscribe, set, update } = writable(0);

  return {
    subscribe,
    increment: async () => {
      const newValue = await invoke<number>('increment');
      set(newValue);
    },
    reset: () => set(0),
    load: async () => {
      const value = await invoke<number>('get_counter');
      set(value);
    }
  };
}

export const counter = createCounter();

// Derived store example
export const doubleCounter = derived(
  counter,
  $counter => $counter * 2
);
```

**Using Stores in Svelte Components:**
```svelte
<script lang="ts">
  import { counter, doubleCounter } from '$lib/stores/counter';
  import { onMount } from 'svelte';

  onMount(() => {
    counter.load();
  });
</script>

<div>
  <p>Counter: {$counter}</p>
  <p>Double: {$doubleCounter}</p>
  <button on:click={() => counter.increment()}>Increment</button>
  <button on:click={() => counter.reset()}>Reset</button>
</div>
```

### Error Handling

**Rust Error Handling Pattern:**
```rust
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AppError {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    Io(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err.to_string())
    }
}

#[tauri::command]
async fn safe_operation(path: String) -> Result<String, AppError> {
    if path.is_empty() {
        return Err(AppError::InvalidInput("Path cannot be empty".to_string()));
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|_| AppError::FileNotFound(path))?;

    Ok(content)
}
```

**Svelte Error Handling:**
```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let error: string | null = null;
  let content = '';

  async function loadFile(path: string) {
    error = null;
    try {
      content = await invoke<string>('safe_operation', { path });
    } catch (err) {
      error = err as string;
      console.error('Operation failed:', err);
    }
  }
</script>

{#if error}
  <div class="error">
    Error: {error}
  </div>
{/if}

{#if content}
  <pre>{content}</pre>
{/if}
```

### Event System

**Emit Events from Rust:**
```rust
use tauri::Manager;
use serde::Serialize;

#[derive(Clone, Serialize)]
struct ProgressPayload {
    percentage: u32,
    message: String,
}

#[tauri::command]
async fn long_running_task(app: tauri::AppHandle) -> Result<(), String> {
    // Emit progress updates
    app.emit_all("task_progress", ProgressPayload {
        percentage: 25,
        message: "Starting...".to_string(),
    }).unwrap();

    // Simulate work
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    app.emit_all("task_progress", ProgressPayload {
        percentage: 100,
        message: "Complete!".to_string(),
    }).unwrap();

    Ok(())
}
```

**Listen to Events in Svelte:**
```svelte
<script lang="ts">
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  interface ProgressPayload {
    percentage: number;
    message: string;
  }

  let progress = 0;
  let message = '';
  let unlisten: UnlistenFn | null = null;

  onMount(async () => {
    unlisten = await listen<ProgressPayload>('task_progress', (event) => {
      progress = event.payload.percentage;
      message = event.payload.message;
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<div>
  <div class="progress-bar" style="width: {progress}%"></div>
  <p>{message} - {progress}%</p>
</div>
```

### File System Access

**Secure File Operations (Rust):**
```rust
use tauri::api::path::{download_dir, app_data_dir};

#[tauri::command]
async fn save_to_downloads(
    content: String,
    filename: String,
) -> Result<String, String> {
    let downloads = download_dir()
        .ok_or("Could not find downloads directory")?;

    let path = downloads.join(filename);
    std::fs::write(&path, content)
        .map_err(|e| e.to_string())?;

    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
async fn read_app_data(
    filename: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let app_data = app_data_dir(&app.config())
        .ok_or("Could not find app data directory")?;

    let path = app_data.join(filename);
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
}
```

**File Dialogs in Svelte:**
```svelte
<script lang="ts">
  import { open, save } from '@tauri-apps/api/dialog';
  import { readTextFile, writeTextFile } from '@tauri-apps/api/fs';

  let content = '';

  async function openFile() {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Text Files',
        extensions: ['txt', 'md', 'json']
      }]
    });

    if (selected && typeof selected === 'string') {
      content = await readTextFile(selected);
    }
  }

  async function saveFile() {
    const filePath = await save({
      filters: [{
        name: 'Text File',
        extensions: ['txt']
      }]
    });

    if (filePath) {
      await writeTextFile(filePath, content);
    }
  }
</script>

<button on:click={openFile}>Open File</button>
<button on:click={saveFile}>Save File</button>
<textarea bind:value={content}></textarea>
```

## Development Guidelines

### Rust Best Practices

**Code Organization:**
- Separate concerns into modules (`commands.rs`, `state.rs`, `events.rs`, `utils.rs`)
- Use `lib.rs` for shared code between main app and tests
- Keep `main.rs` minimal - mainly for app initialization
- Use feature flags for conditional compilation

**Error Handling:**
- Always use `Result<T, E>` for fallible operations
- Use `thiserror` crate for custom error types implementing `Serialize`
- Use `?` operator for error propagation
- Avoid `unwrap()` in production code - use proper error handling
- Return descriptive error messages to the frontend

**Performance:**
- Use `async` for I/O-bound operations (file operations, network requests)
- Prefer `tokio` runtime for async operations
- Use `Mutex` or `RwLock` for shared state (prefer `RwLock` for read-heavy workloads)
- Profile with `cargo flamegraph` for CPU-bound bottlenecks
- Avoid blocking the main thread

**Security:**
- Validate all inputs from frontend
- Use Tauri's allowlist to restrict API access
- Implement proper CSP in `tauri.conf.json`
- Sanitize file paths to prevent directory traversal
- Use `tauri::api::path` helpers for safe path operations

### Svelte Best Practices

**Component Organization:**
- Keep components small and focused (< 200 lines)
- Use `$lib` alias for imports (`$lib/components/Button.svelte`)
- Separate UI components from business logic
- Use TypeScript for better type safety

**Reactivity:**
- Use reactive declarations (`$:`) for derived state
- Avoid unnecessary reactive statements
- Use stores for shared state across components
- Leverage Svelte's automatic subscriptions with `$` prefix

**Performance:**
- Lazy load components with dynamic imports
```svelte
<script lang="ts">
  import { onMount } from 'svelte';

  let HeavyComponent;

  onMount(async () => {
    const module = await import('$lib/components/HeavyComponent.svelte');
    HeavyComponent = module.default;
  });
</script>

{#if HeavyComponent}
  <svelte:component this={HeavyComponent} />
{/if}
```

- Use `{#key}` blocks to force re-renders only when needed
- Debounce expensive Tauri command calls
```svelte
<script lang="ts">
  import { debounce } from '$lib/utils';

  const debouncedSearch = debounce(async (query: string) => {
    const results = await invoke('search', { query });
    // Handle results
  }, 300);
</script>

<input on:input={(e) => debouncedSearch(e.target.value)} />
```

**TypeScript Usage:**
- Define types for all Tauri command responses
```typescript
// src/lib/types/api.ts
export interface FileInfo {
  name: string;
  size: number;
  modified: number;
}

export interface AppConfig {
  theme: 'light' | 'dark';
  language: string;
}
```

- Use strict mode in `tsconfig.json`
- Avoid `any` types - use `unknown` when type is truly unknown
- Use generic types for reusable components
```svelte
<script lang="ts" generics="T">
  export let items: T[];
  export let renderItem: (item: T) => string;
</script>

<ul>
  {#each items as item}
    <li>{renderItem(item)}</li>
  {/each}
</ul>
```

### Naming Conventions

**Rust:**
- `snake_case` for: functions, variables, modules, files
- `PascalCase` for: types, structs, enums, traits
- `SCREAMING_SNAKE_CASE` for: constants, statics
- Prefix private items with `_` if unused

**Svelte/TypeScript:**
- `PascalCase` for: component files (`Button.svelte`, `UserProfile.svelte`)
- `camelCase` for: variables, functions, props
- `kebab-case` for: CSS classes, event names
- `SCREAMING_SNAKE_CASE` for: constants
- Prefix stores with `$` when using auto-subscription

### Testing Strategy

**Rust Unit Tests:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_logic() {
        let result = process_data("input");
        assert_eq!(result, "expected");
    }

    #[tokio::test]
    async fn test_async_command() {
        let result = async_operation().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_handling() {
        let result = validate_input("");
        assert!(result.is_err());
    }
}
```

**Integration Tests:**
```rust
// tests/integration_test.rs
use my_app::commands;

#[tokio::test]
async fn test_full_workflow() {
    // Test complete user workflows
    let result = commands::read_file("test.txt".to_string()).await;
    assert!(result.is_ok());
}
```

**Svelte Component Tests:**
```typescript
// src/lib/components/Button.test.ts
import { render, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi } from 'vitest';
import Button from './Button.svelte';

describe('Button', () => {
  it('renders with text', () => {
    const { getByText } = render(Button, { props: { text: 'Click me' } });
    expect(getByText('Click me')).toBeTruthy();
  });

  it('calls onClick handler', async () => {
    const onClick = vi.fn();
    const { getByRole } = render(Button, { props: { onClick } });

    await fireEvent.click(getByRole('button'));
    expect(onClick).toHaveBeenCalledOnce();
  });
});
```

**Testing Tauri Commands from Svelte:**
```typescript
// src/lib/stores/counter.test.ts
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import { counter } from './counter';

// Mock Tauri API
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn()
}));

import { invoke } from '@tauri-apps/api/tauri';

describe('Counter Store', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('increments counter', async () => {
    vi.mocked(invoke).mockResolvedValue(1);

    await counter.increment();

    expect(invoke).toHaveBeenCalledWith('increment');
    expect(get(counter)).toBe(1);
  });
});
```

## Dependency Management

### Rust Dependencies (Cargo.toml)

**Core Dependencies:**
```toml
[dependencies]
tauri = { version = "1.5", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
anyhow = "1.0"        # Flexible error handling
thiserror = "1.0"     # Custom error types
```

**Common Tauri Features:**
- `shell-open`: Open URLs in default browser
- `fs-all`: Full file system access
- `dialog-all`: File/message dialogs
- `notification-all`: System notifications
- `global-shortcut-all`: Global keyboard shortcuts
- `clipboard-all`: Clipboard access
- `http-all`: HTTP client

**Example with multiple features:**
```toml
[dependencies]
tauri = { version = "1.5", features = [
  "shell-open",
  "dialog-all",
  "fs-all",
  "notification-all"
] }
```

**Development Dependencies:**
```toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"      # Mocking for tests
serial_test = "3.0"   # Serial test execution
```

### Frontend Dependencies (package.json)

**Core Dependencies:**
```json
{
  "dependencies": {
    "@tauri-apps/api": "^1.5.0",
    "svelte": "^4.2.0"
  },
  "devDependencies": {
    "@sveltejs/vite-plugin-svelte": "^3.0.0",
    "@tauri-apps/cli": "^1.5.0",
    "@tsconfig/svelte": "^5.0.0",
    "typescript": "^5.0.0",
    "vite": "^5.0.0",
    "svelte-check": "^3.6.0",
    "@testing-library/svelte": "^4.0.0",
    "vitest": "^1.0.0"
  }
}
```

**Optional UI Libraries:**
- **Tailwind CSS**: `tailwindcss`, `autoprefixer`, `postcss`
- **Svelte Motion**: `svelte-motion` (animations)
- **Svelte Icons**: `svelte-icons` or `lucide-svelte`
- **Form Validation**: `felte`, `svelte-forms-lib`

## Vite Configuration for Svelte + Tauri

**vite.config.ts:**
```typescript
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],

  // Prevent vite from obscuring rust errors
  clearScreen: false,

  // Tauri expects a fixed port, fail if that port is not available
  server: {
    port: 5173,
    strictPort: true,
  },

  // To access the Tauri environment variables set by the CLI with information about the current target
  envPrefix: ['VITE_', 'TAURI_'],

  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // Don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // Produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },

  // Resolve aliases
  resolve: {
    alias: {
      '$lib': '/src/lib',
      '$components': '/src/lib/components',
      '$stores': '/src/lib/stores',
    }
  }
});
```

## Security Considerations

### Tauri Allowlist Configuration

Restrict API access to only what's needed:
```json
{
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "scope": ["$APPDATA/*", "$DOWNLOAD/*", "$RESOURCE/*"],
        "readFile": true,
        "writeFile": true,
        "readDir": true,
        "createDir": true,
        "removeFile": true
      },
      "dialog": {
        "open": true,
        "save": true,
        "message": true,
        "ask": true
      },
      "shell": {
        "open": true
      },
      "notification": {
        "all": true
      }
    }
  }
}
```

### Content Security Policy

```json
{
  "tauri": {
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:"
    }
  }
}
```

### Input Validation

Always validate frontend inputs in Rust:
```rust
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct UserInput {
    #[validate(length(min = 1, max = 100))]
    name: String,

    #[validate(email)]
    email: String,

    #[validate(range(min = 0, max = 120))]
    age: u8,
}

#[tauri::command]
fn validate_input(data: UserInput) -> Result<String, String> {
    data.validate()
        .map_err(|e| format!("Validation error: {}", e))?;

    Ok("Valid input".to_string())
}
```

## Build Optimization

### Release Build Configuration

**Cargo.toml:**
```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization, slower compile
panic = "abort"       # Smaller binaries
strip = true          # Strip symbols
```

### Frontend Build Optimization

**vite.config.ts:**
```typescript
export default defineConfig({
  build: {
    minify: 'esbuild',
    target: 'esnext',
    rollupOptions: {
      output: {
        manualChunks: {
          'tauri': ['@tauri-apps/api'],
        },
      },
    },
  },
});
```

## Common Workflow Examples

### Desktop App with File Operations
```bash
# 1. Initialize project
npm create tauri-app
# Select: Svelte, TypeScript, Vite

# 2. Add file system commands in src-tauri/src/commands.rs
# 3. Configure allowlist in tauri.conf.json
# 4. Create Svelte components for file management
# 5. Test in development
npm run tauri dev

# 6. Build for production
npm run tauri build
```

### Create a Settings Panel
```svelte
<!-- src/lib/components/Settings.svelte -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { settings } from '$lib/stores/settings';

  async function saveSettings() {
    try {
      await invoke('save_settings', {
        settings: $settings
      });
      // Show success message
    } catch (error) {
      console.error('Failed to save settings:', error);
    }
  }
</script>

<div class="settings">
  <h2>Settings</h2>

  <label>
    Theme
    <select bind:value={$settings.theme}>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
    </select>
  </label>

  <button on:click={saveSettings}>Save</button>
</div>
```

### System Tray Integration
```rust
// src-tauri/src/main.rs
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::Manager;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        let window = app.get_window("main").unwrap();
                        window.show().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Documentation Resources

This repository should include:

- **CLAUDE.md** (this file): Technical reference for Claude Code AI assistant
- **README.md**: User-facing documentation with quick start guide
- **ARCHITECTURE.md**: System architecture and design decisions
- **CONTRIBUTING.md**: Contribution guidelines and development setup
- **CHANGELOG.md**: Version history and release notes

## Debugging and Troubleshooting

### Rust Debugging

**Enable verbose logging:**
```bash
RUST_LOG=debug npm run tauri dev
```

**Attach debugger (VS Code):**
```json
// .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Tauri Development Debug",
      "cargo": {
        "args": [
          "build",
          "--manifest-path=./src-tauri/Cargo.toml",
          "--no-default-features"
        ]
      },
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

### Svelte Debugging

**Open DevTools in dev mode:**
```rust
// src-tauri/src/main.rs
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Svelte DevTools:**
- Install Svelte DevTools browser extension
- Use `console.log` for reactive statements:
```svelte
<script lang="ts">
  $: console.log('Value changed:', value);
</script>
```

### Common Issues

**Build Failures:**
```bash
# Clear all caches and rebuild
cargo clean
rm -rf node_modules dist .svelte-kit
npm install
npm run tauri build
```

**Hot Reload Not Working:**
- Check that Vite dev server is running on port 5173
- Verify `devPath` in `tauri.conf.json` matches Vite port
- Ensure `beforeDevCommand` is correct

**Svelte Component Not Updating:**
- Check reactive statements use `$:` prefix
- Ensure stores are subscribed with `$` prefix
- Verify component key blocks if forcing re-render

**Runtime Errors:**
- Check browser console for frontend errors
- Review Rust logs with `RUST_LOG=debug`
- Verify allowlist permissions in `tauri.conf.json`
- Check that Tauri commands are registered in `main.rs`

## Version History

**v1.0.0** (2025-01-13):
- Initial Tauri + Svelte + Rust project setup
- Cross-platform desktop application framework
- Rust backend with Tauri commands
- Svelte frontend with TypeScript
- Vite build pipeline with hot reload
- Comprehensive testing setup (Vitest + Svelte Testing Library)
- Development and production build configurations
