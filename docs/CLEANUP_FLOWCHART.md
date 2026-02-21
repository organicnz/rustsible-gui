# Aggressive Process Cleanup Flowchart

```
┌─────────────────────────────────────────────────────────────────┐
│                  RUSTSIBLE-GUI STARTUP                          │
│                                                                 │
│              cleanup_previous_instances()                       │
└────────────────────────┬────────────────────────────────────────┘
                         │
                         ▼
              ┌──────────────────────┐
              │  Scan for existing   │
              │  rustsible-gui       │
              │  processes           │
              └──────────┬───────────┘
                         │
                    ┌────┴────┐
                    │ Found?  │
                    └────┬────┘
                         │
              ┌──────────┴──────────┐
              │ NO                  │ YES
              ▼                     ▼
    ┌─────────────────┐   ┌─────────────────────┐
    │  No cleanup     │   │  Start 6-Phase      │
    │  needed         │   │  Cleanup            │
    └─────────────────┘   └──────────┬──────────┘
              │                      │
              │                      │
              └──────────┬───────────┘
                         │
                         ▼
    ┌───────────────────────────────────────────────────────────┐
    │                     PHASE 0                               │
    │           Kill Parent Processes (Unix)                    │
    │                                                           │
    │  1. Find parent PIDs of all target processes             │
    │  2. Exclude system processes (launchd, init, systemd)    │
    │  3. Execute: kill -9 <parent_pid>                        │
    │  4. Wait 200ms                                           │
    │                                                           │
    │  WHY: Releases zombie children held by parent            │
    └───────────────────────┬───────────────────────────────────┘
                            │
                            ▼
    ┌───────────────────────────────────────────────────────────┐
    │                     PHASE 1                               │
    │            Graceful Termination (SIGTERM)                 │
    │                                                           │
    │  1. Send SIGTERM to each target process                  │
    │  2. Wait 300ms for graceful shutdown                     │
    │                                                           │
    │  WHY: Allows cleanup handlers to run                     │
    └───────────────────────┬───────────────────────────────────┘
                            │
                            ▼
    ┌───────────────────────────────────────────────────────────┐
    │                     PHASE 2                               │
    │              Force Kill (SIGKILL)                         │
    │                                                           │
    │  1. Send SIGKILL to remaining processes                  │
    │  2. Wait 500ms for termination                           │
    │                                                           │
    │  WHY: Force immediate exit for stuck processes           │
    └───────────────────────┬───────────────────────────────────┘
                            │
                            ▼
    ┌───────────────────────────────────────────────────────────┐
    │                     PHASE 3                               │
    │           System-wide Cleanup (Unix)                      │
    │                                                           │
    │  1. Execute: killall -9 rustsible-gui                    │
    │  2. Execute: pkill -9 -f rustsible-gui                   │
    │  3. Wait 300ms                                           │
    │                                                           │
    │  WHY: Catch processes missed by direct PID targeting     │
    └───────────────────────┬───────────────────────────────────┘
                            │
                            ▼
    ┌───────────────────────────────────────────────────────────┐
    │                     PHASE 4                               │
    │         macOS launchctl Force Quit (macOS only)           │
    │                                                           │
    │  1. launchctl kill SIGKILL system/<pid>                  │
    │  2. launchctl kill SIGKILL gui/<pid>                     │
    │  3. launchctl remove system/<pid>                        │
    │  4. Wait 300ms                                           │
    │                                                           │
    │  WHY: Force launchd to release stuck processes           │
    └───────────────────────┬───────────────────────────────────┘
                            │
                            ▼
    ┌───────────────────────────────────────────────────────────┐
    │                     PHASE 5                               │
    │        Process Group Termination (Unix) - NUCLEAR         │
    │                                                           │
    │  1. Get process group ID: ps -o pgid= -p <pid>           │
    │  2. Execute: kill -9 -<pgid>  (negative PID!)            │
    │  3. Wait 300ms                                           │
    │                                                           │
    │  WHY: Kill entire process tree if individual kills fail  │
    └───────────────────────┬───────────────────────────────────┘
                            │
                            ▼
              ┌─────────────────────────┐
              │  Final Verification     │
              │  Scan for remaining     │
              │  processes              │
              └──────────┬──────────────┘
                         │
                    ┌────┴────┐
                    │ Found?  │
                    └────┬────┘
                         │
              ┌──────────┴──────────┐
              │ NO                  │ YES
              ▼                     ▼
    ┌─────────────────┐   ┌─────────────────────┐
    │  SUCCESS        │   │  LOG WARNING        │
    │  All processes  │   │  List zombie PIDs   │
    │  terminated     │   │  with status        │
    └────────┬────────┘   └──────────┬──────────┘
             │                       │
             └───────────┬───────────┘
                         │
                         ▼
              ┌──────────────────────┐
              │  Continue GUI        │
              │  Startup             │
              └──────────────────────┘


═══════════════════════════════════════════════════════════════════

PLATFORM MATRIX:

┌──────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│ Platform │ Phase 0 │ Phase 1 │ Phase 2 │ Phase 3 │ Phase 4 │ Phase 5 │
├──────────┼─────────┼─────────┼─────────┼─────────┼─────────┼─────────┤
│ macOS    │    ✓    │    ✓    │    ✓    │    ✓    │    ✓    │    ✓    │
│ Linux    │    ✓    │    ✓    │    ✓    │    ✓    │    ✗    │    ✓    │
│ Windows  │    ✗    │    ✓    │    ✓    │    ✗    │    ✗    │    ✗    │
└──────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘

═══════════════════════════════════════════════════════════════════

SAFETY CHECKS (Applied to all phases):

┌────────────────────────────────────────────────────────────┐
│  ✓ Never kill current process (pid != current_pid)        │
│  ✓ Never kill PID 1 (init/launchd)                        │
│  ✓ Never kill system daemons (launchd, systemd, init)     │
│  ✓ Never kill current process group                       │
│  ✓ All commands use .ok() - ignore failures gracefully    │
│  ✓ All output redirected to /dev/null                     │
└────────────────────────────────────────────────────────────┘

═══════════════════════════════════════════════════════════════════

TIMING BREAKDOWN (Worst Case):

Phase 0: Parent kill          200ms
Phase 1: SIGTERM wait         300ms
Phase 2: SIGKILL wait         500ms
Phase 3: System cleanup       300ms
Phase 4: launchctl (macOS)    300ms
Phase 5: Process group        300ms
                        ─────────────
Total (macOS):               1900ms
Total (Linux):               1600ms
Total (Windows):              800ms

Best case: 0ms (no processes found)

═══════════════════════════════════════════════════════════════════

ZOMBIE PROCESS STATES HANDLED:

┌──────────┬────────────────────────────────────────────────────┐
│  State   │  Description                                       │
├──────────┼────────────────────────────────────────────────────┤
│  R       │  Running (Phase 1-2 sufficient)                   │
│  S       │  Sleeping (Phase 1-2 sufficient)                  │
│  T       │  Stopped (Phase 2-3 required)                     │
│  Z       │  Zombie (Phase 0 required - kill parent)          │
│  D       │  Uninterruptible (Phase 5 nuclear option)         │
│  UE      │  Uninterruptible Exiting (macOS - Phase 4+5)      │
│  UE+     │  Uninterruptible Exiting + (macOS - Phase 4+5)    │
└──────────┴────────────────────────────────────────────────────┘

═══════════════════════════════════════════════════════════════════
```

## Key Innovations for Zombie Elimination

### 1. Parent Process Killing (Phase 0)
**Problem:** Zombie processes persist because parent hasn't called `wait()`.
**Solution:** Kill parent first, forcing kernel to reparent to init, which automatically reaps zombies.

### 2. launchctl Integration (Phase 4 - macOS)
**Problem:** macOS processes can be held by launchd even after SIGKILL.
**Solution:** Use `launchctl kill/remove` to force launchd to release the process.

### 3. Process Group Termination (Phase 5)
**Problem:** Process spawned children or is part of a session that holds it.
**Solution:** Kill entire process group with `kill -9 -<pgid>`.

## Success Metrics

After all 6 phases complete:
- `ps aux | grep rustsible-gui | grep -v grep` returns **0 results**
- Final verification shows **0 remaining processes**
- GUI starts with clean process environment
