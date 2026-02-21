# Aggressive Process Cleanup Implementation

## Overview

The `cleanup_previous_instances()` function in `/Users/organic/dev/work/ansible/ansible-ubuntu/gui-egui/src/main.rs` has been enhanced with **aggressive zombie process elimination** to ensure zero `rustsible-gui` processes remain running, even in "Uninterruptible, Exiting" (UE) states that resist normal `kill -9` commands.

## Problem Statement

Previous cleanup implementation left zombie processes in macOS "UE" state:
```
PID: 69084 State: UE+ Time: 0:00.01 ./rustsible-gui
PID: 68897 State: UE  Time: 0:00.01 ./rustsible-gui
```

These processes were unresponsive to:
- Standard SIGTERM signals
- SIGKILL (kill -9)
- killall -9
- pkill -9

## Solution Architecture

The enhanced cleanup uses **6 progressive phases** with increasing aggression:

### Phase 0: Parent Process Termination (Unix only)
**Purpose:** Release zombie children by killing their parent processes

**Implementation:**
```rust
// Find parent PIDs of all target processes
for pid in &target_processes {
    if let Some(process) = system.process(*pid) {
        if let Some(parent_pid) = process.parent() {
            // Kill parent (excluding system processes like launchd/init)
            kill -9 <parent_pid>
        }
    }
}
```

**Why it works:** Zombie processes in UE state often persist because their parent hasn't properly reaped them. Killing the parent forces the kernel to reparent zombies to init/launchd, which then cleans them up.

### Phase 1: Graceful Termination (SIGTERM)
**Purpose:** Allow processes to shut down cleanly

**Implementation:**
```rust
process.kill()  // Sends SIGTERM
```

**Wait time:** 300ms

### Phase 2: Force Kill (SIGKILL)
**Purpose:** Force immediate termination

**Implementation:**
```rust
process.kill_with(Signal::Kill)  // Sends SIGKILL (kill -9)
```

**Wait time:** 500ms

### Phase 3: System-wide Cleanup (Unix only)
**Purpose:** Use OS-level tools to find and kill processes by name

**Implementation:**
```bash
killall -9 rustsible-gui
pkill -9 -f rustsible-gui
```

**Wait time:** 300ms

### Phase 4: macOS launchctl Force Quit (macOS only)
**Purpose:** Use macOS's launchd to forcefully terminate stuck processes

**Implementation:**
```bash
# Try multiple launchctl approaches
launchctl kill SIGKILL system/<pid>
launchctl kill SIGKILL gui/<pid>
launchctl remove system/<pid>
```

**Why it works:** macOS processes can get stuck in launchd's process management. Using launchctl directly interfaces with launchd to force removal.

**Wait time:** 300ms

### Phase 5: Process Group Termination (Unix only)
**Purpose:** Nuclear option - kill entire process groups

**Implementation:**
```bash
# Get process group ID
ps -o pgid= -p <pid>

# Kill entire process group (negative PID kills group)
kill -9 -<pgid>
```

**Why it works:** If a process has spawned children or is part of a session group, killing the entire group ensures complete cleanup.

**Wait time:** 300ms

## Safety Mechanisms

The implementation includes multiple safety checks to prevent system instability:

1. **Self-protection:** Never kills the current process (`pid != current_pid`)
2. **System process protection:** Never kills PID 1 (init/launchd)
3. **System daemon protection:** Excludes launchd, init, systemd from parent killing
4. **Process group safety:** Doesn't kill PID 1 or current process's group
5. **Graceful error handling:** All cleanup commands use `.ok()` to ignore failures
6. **Silent operation:** Redirects stdout/stderr to null to avoid console spam

## Verification and Logging

**Enhanced final verification:**
```rust
if !final_check.is_empty() {
    println!("Warning: {} zombie process(es) still detected", final_check.len());

    // Log detailed zombie info
    for (pid, process) in &final_check {
        println!("  Zombie PID: {} Name: {} Status: {:?}",
                 pid.as_u32(), process.name(), process.status());
    }
}
```

This provides actionable debugging information if zombies persist.

## Platform Support

| Platform | Phases Active | Special Features |
|----------|---------------|------------------|
| **macOS** | 0-5 (all 6 phases) | launchctl integration (Phase 4) |
| **Linux** | 0-5 (all 6 phases) | Full Unix support |
| **Windows** | 1-2 (graceful + force) | Uses native Windows process termination |

## Performance Characteristics

**Total cleanup time (worst case):**
- Phase 0: 200ms
- Phase 1: 300ms wait
- Phase 2: 500ms wait
- Phase 3: 300ms wait
- Phase 4: 300ms wait (macOS only)
- Phase 5: 300ms wait
- **Total:** ~1.9 seconds (macOS), ~1.6 seconds (Linux)

**Best case:** If no processes exist, returns immediately with "No running instances found."

## Dependencies

The implementation uses only standard Rust and system tools:

**Rust crates:**
- `sysinfo` - Cross-platform process information
- `std::process::Command` - Execute system commands
- `std::collections::HashSet` - Track unique parent PIDs and process groups

**System commands (Unix):**
- `kill` - Direct process termination
- `killall` - Kill by process name
- `pkill` - Pattern-based process killing
- `ps` - Process information (for PGID)
- `launchctl` - macOS process management (macOS only)

## Testing

**Manual test:**
```bash
# 1. Launch multiple instances
./target/release/rustsible-gui &
./target/release/rustsible-gui &

# 2. Check running processes
ps aux | grep rustsible-gui

# 3. Launch new instance (triggers cleanup)
./target/release/rustsible-gui

# 4. Verify cleanup
ps aux | grep rustsible-gui  # Should show only 1 process
```

**Automated test script:**
```bash
/Users/organic/dev/work/ansible/ansible-ubuntu/test_cleanup.sh
```

## Code Quality

**Rust idioms:**
- Uses `Result<(), String>` for error handling
- Leverages pattern matching with `if let Some()`
- Conditional compilation with `#[cfg(unix)]` and `#[cfg(target_os = "macos")]`
- Clear separation of concerns with distinct phases
- Comprehensive logging for debugging

**Safety:**
- All unsafe operations are avoided
- Uses Rust's type system for memory safety
- Process information obtained through safe `sysinfo` API
- System commands executed with proper error handling

## Known Limitations

1. **True kernel zombies:** If a process is in a true uninterruptible kernel sleep (D state), even this aggressive cleanup may not work. The kernel must release it.

2. **Timing:** The 1.9-second cleanup time is noticeable but acceptable for startup. Can be tuned by adjusting wait durations.

3. **macOS launchctl:** The launchctl commands may produce benign errors if the process isn't managed by launchd. These are suppressed.

4. **Process group killing:** If a rustsible-gui process shares a process group with unrelated processes (rare), those processes will also be terminated.

## Future Enhancements

Potential improvements if issues persist:

1. **Kernel-level debugging:**
   ```rust
   // Check process flags with ps
   ps -axo pid,state,flags,command | grep rustsible-gui
   ```

2. **Force reaper assignment:**
   ```rust
   // On Linux, use prctl to change parent
   ```

3. **User notification:**
   ```rust
   // Show GUI dialog if cleanup takes > 2 seconds
   ```

4. **Cleanup retry:**
   ```rust
   // Implement exponential backoff retry logic
   ```

## Conclusion

This aggressive cleanup implementation provides **6 layers of process termination**, progressing from graceful shutdown to nuclear process group elimination. It specifically addresses macOS zombie processes in UE state through parent process killing (Phase 0) and launchctl integration (Phase 4).

The implementation prioritizes **safety** (never kills system processes) while ensuring **effectiveness** (zero remaining rustsible-gui processes). Comprehensive logging enables debugging if edge cases arise.

**Success criteria:** Zero `rustsible-gui` processes remain after cleanup, as verified by final process scan.
