# Process Cleanup Implementation

## Overview

The `rustsible-gui` application now includes **automatic process cleanup** functionality that runs on startup. This eliminates the need for a separate `launch-gui` bash script by integrating process management directly into the Rust application.

## What It Does

When you run `rustsible-gui`, it automatically:

1. **Detects the OS** (macOS/Linux/Windows) using Rust's cross-platform capabilities
2. **Finds all existing rustsible-gui processes** (excluding the current process)
3. **Attempts graceful termination** using SIGTERM (or OS equivalent)
4. **Force kills stubborn processes** using SIGKILL after a brief wait
5. **Verifies cleanup** and reports any zombie processes
6. **Launches the GUI** only after cleanup is complete

## Implementation Details

### Dependencies Added

The implementation uses the `sysinfo` crate for cross-platform process management:

```toml
# Cargo.toml
sysinfo = "0.30"
```

**Why `sysinfo`?**
- Cross-platform (works on macOS, Linux, Windows)
- Safe, idiomatic Rust API
- No unsafe code required
- Well-maintained and widely used
- Type-safe process handling

### Code Architecture

The cleanup logic is implemented in the `cleanup_previous_instances()` function, which:

1. **Retrieves current PID** to avoid self-termination
2. **Scans system processes** using `sysinfo::System`
3. **Filters processes** by name ("rustsible-gui")
4. **Executes two-phase termination:**
   - Phase 1: Graceful shutdown (SIGTERM) with 300ms wait
   - Phase 2: Force kill (SIGKILL) for remaining processes with 500ms wait
5. **Reports results** with detailed console output

### Platform-Specific Behavior

#### Unix/Linux/macOS
- Uses `Signal::Kill` (SIGKILL) for forced termination
- Uses `process.kill()` for graceful termination (SIGTERM)
- Provides detailed logging of each signal sent

#### Windows
- Uses standard `process.kill()` which is inherently forceful on Windows
- Provides consistent behavior across platforms

### Error Handling

The function uses `Result<(), String>` for error handling:

- **Success**: All processes cleaned up successfully
- **Partial Success**: Some processes terminated, warnings logged
- **Graceful Degradation**: If cleanup fails, the application continues anyway with a warning

### Code Location

**File:** `/Users/organic/dev/work/ansible/ansible-ubuntu/gui-egui/src/main.rs`

**Key Functions:**
- `cleanup_previous_instances()` - Main cleanup logic (lines 789-896)
- `main()` - Calls cleanup before GUI initialization (lines 898-919)

## Usage

### Building

```bash
cd gui-egui
cargo build --release
```

### Running

Simply execute the binary directly - cleanup happens automatically:

```bash
./rustsible-gui
```

**Console Output Example:**
```
Cleaning up previous instances...
Found 5 process(es) to terminate
Phase 1: Attempting graceful termination...
  Sending TERM signal to PID 12345...
  Sending TERM signal to PID 12346...
  Sending TERM signal to PID 12347...
  Sending TERM signal to PID 12348...
  Sending TERM signal to PID 12349...
Phase 2: Force killing remaining processes...
  Force quitting PID 12345...
  Force quitting PID 12346...
Cleanup completed successfully.
Launching Rustsible GUI...
```

## Advantages Over Bash Script

### Type Safety
- Rust's type system prevents common bugs
- No string parsing or brittle grep patterns
- Compile-time guarantees about process handling

### Cross-Platform
- Single codebase works on macOS, Linux, and Windows
- No need for platform-specific bash scripts
- Consistent behavior across operating systems

### Error Handling
- Explicit Result types
- Graceful degradation if cleanup fails
- No silent failures

### Maintainability
- All code in one language (Rust)
- Better IDE support (autocomplete, refactoring, etc.)
- Unit testable with Rust's testing framework

### Performance
- Compiled binary runs faster than shell script
- Efficient process scanning with sysinfo crate
- Minimal overhead on startup

## Testing

### Manual Testing

1. **Start multiple instances** (they should fail to cleanup themselves):
   ```bash
   ./rustsible-gui &
   ./rustsible-gui &
   ./rustsible-gui &
   ```

2. **Check running processes:**
   ```bash
   ps aux | grep rustsible-gui
   ```

3. **Run new instance** (should clean up old ones):
   ```bash
   ./rustsible-gui
   ```

4. **Verify cleanup:**
   ```bash
   ps aux | grep rustsible-gui  # Should show only one instance
   ```

### Automated Testing

The cleanup function can be unit tested:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_no_processes() {
        // Should succeed gracefully when no processes exist
        assert!(cleanup_previous_instances().is_ok());
    }
}
```

## Migration from Bash Script

### Old Approach (launch-gui script)
```bash
#!/bin/bash
# Detect OS, kill processes manually
if [ "$OS" = "Darwin" ]; then
    PIDS=$(ps aux | grep "[r]ustsible-gui" | awk '{print $2}')
    for pid in $PIDS; do
        kill -15 "$pid"
    done
    # ... more bash scripting
fi
exec ./rustsible-gui
```

### New Approach (Rust integration)
```rust
fn main() -> Result<(), eframe::Error> {
    // Automatic cleanup - no external script needed!
    cleanup_previous_instances()?;

    // Launch GUI
    eframe::run_native(/* ... */)
}
```

## Known Limitations

1. **Zombie Processes**: On some systems, zombie processes (state UE) cannot be killed and must be reaped by the parent process or system. The cleanup function detects and reports these.

2. **Permissions**: If old processes are owned by a different user, cleanup may fail. This is expected and handled gracefully.

3. **Race Conditions**: If multiple instances start simultaneously, they may not kill each other before initializing. This is an edge case and generally not an issue in practice.

## Future Enhancements

Possible improvements:

1. **Single Instance Guard**: Use a PID file or named mutex to prevent multiple instances entirely
2. **IPC Communication**: Allow new instance to communicate with old instance to transfer state
3. **Graceful Handoff**: Transfer any pending operations from old to new instance
4. **Configuration Option**: Allow users to disable auto-cleanup via config file

## Troubleshooting

### Issue: "Failed to clean up previous instances"
**Cause**: Permission error or system limitation
**Solution**: Check process ownership with `ps aux | grep rustsible-gui`. If processes are owned by root or another user, you may need elevated privileges.

### Issue: "Warning: X zombie process(es) detected"
**Cause**: Zombie processes in uninterruptable state
**Solution**: These will be automatically cleaned up by the OS. You can ignore this warning.

### Issue: Cleanup takes too long
**Cause**: Many old processes or slow system
**Solution**: The timeouts (300ms + 500ms) are conservative. You can reduce them in the code if needed.

## References

- **sysinfo crate**: https://docs.rs/sysinfo/
- **Rust std::process**: https://doc.rust-lang.org/std/process/
- **Signal handling**: https://en.wikipedia.org/wiki/Signal_(IPC)
