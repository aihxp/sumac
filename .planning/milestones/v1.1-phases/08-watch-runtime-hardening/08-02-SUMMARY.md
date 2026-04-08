---
phase: 08-watch-runtime-hardening
plan: 02
subsystem: watch
tags: [watch, notifications, timeout, webhook, reliability]
requires: [08-01]
provides:
  - Timeout-bounded async notify command execution
  - Timeout-bounded webhook delivery with explicit failure reporting
  - Regression coverage for slow notify commands and slow webhooks
affects: [phase-11]
tech-stack:
  added: []
  patterns: [bounded async side effects, exit-time notification flush]
key-files:
  created: []
  modified: [src/app/watch.rs, tests/cli_integration.rs]
key-decisions:
  - "Spawn notify commands and webhook sends as async tasks so normal watch frame progression is not serialized behind external side effects."
  - "Await only the current event's notification tasks on exit so fast notifications still land before process termination while slow ones remain bounded."
patterns-established:
  - "Watch side effects should carry their own timeout and failure reporting instead of blocking the main loop."
requirements-completed: [WATCH-03]
duration: 8min
completed: 2026-04-04
---

# Phase 8: Watch Runtime Hardening Summary

**Watch notification side effects now run with explicit timeouts and isolated async handling, with regressions proving slow commands and slow webhooks no longer pin the `sxmc watch` process indefinitely**

## Performance

- **Duration:** 8 min
- **Started:** 2026-04-04T17:36:34Z
- **Completed:** 2026-04-04T17:43:51Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments
- Refactored notify commands to use Tokio subprocess execution with an explicit timeout and non-inherited stdio
- Added timeout-bounded Reqwest webhook delivery with explicit timeout messages
- Added slow-side-effect regressions for notify commands and webhooks while keeping the existing watch contract tests green

## Task Commits

Each task was committed atomically:

1. **Task 1: Add timeout-bounded async notification helpers** - `b730001` (fix)
2. **Task 2: Add slow-side-effect regression tests** - `1ef250f` (test)

**Plan metadata:** `0f8f0d4` (docs: plan watch runtime hardening)

## Files Created/Modified
- `src/app/watch.rs` - Spawns timeout-bounded notify command and webhook tasks, waits only on the current event when exiting, and surfaces explicit timeout errors
- `tests/cli_integration.rs` - Adds slow notify-command and slow webhook regressions for unhealthy watch exits

## Decisions Made

- Closed notify child stdio explicitly so slow children cannot keep the parent process's captured pipes open after timeout
- Kept notify-file writes synchronous and cheap while moving only external process and network effects behind async boundaries

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- The first timeout implementation still let slow notify commands keep the parent test harness open through inherited pipes, so the final fix closed child stdio and used explicit child process spawning before timeout

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 8 now leaves `watch` with both a dedicated runtime seam and bounded side effects, which is the contract Phase 11 needs before broader command-family extraction
- The remaining watch work can focus on parity and contract gates rather than raw responsiveness bugs

---
*Phase: 08-watch-runtime-hardening*
*Completed: 2026-04-04*
