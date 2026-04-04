---
phase: 08-watch-runtime-hardening
plan: 01
subsystem: watch
tags: [watch, runtime, app-service, cli, extraction]
requires: []
provides:
  - Dedicated `src/app/watch.rs` runtime seam for `sxmc watch`
  - Thin `Commands::Watch` dispatch in `src/main.rs`
  - Stable watch loop ownership outside the top-level CLI hotspot
affects: [phase-08, phase-11]
tech-stack:
  added: []
  patterns: [command-family app service, watch runtime seam]
key-files:
  created: [src/app/watch.rs]
  modified: [src/app/mod.rs, src/main.rs]
key-decisions:
  - "Move the watch loop and notification orchestration behind `WatchService`, while keeping request assembly in `main.rs`."
  - "Return exit codes from the watch service instead of calling `std::process::exit` from deep inside the loop."
patterns-established:
  - "`watch` now follows the same request/service pattern as the migrated golden-path commands in `src/app/`."
requirements-completed: [CORE-05]
duration: 6min
completed: 2026-04-04
---

# Phase 8: Watch Runtime Hardening Summary

**`sxmc watch` now runs through a dedicated app/runtime seam, with top-level CLI dispatch reduced to typed request assembly and service invocation**

## Performance

- **Duration:** 6 min
- **Started:** 2026-04-04T17:30:57Z
- **Completed:** 2026-04-04T17:36:06Z
- **Tasks:** 1
- **Files modified:** 3

## Accomplishments
- Introduced `src/app/watch.rs` with a typed `WatchRequest`, notification options, and `WatchService`
- Moved the long-running watch loop plus notification payload helpers out of `src/main.rs`
- Kept `main.rs` responsible only for CLI path resolution, header parsing, request assembly, and exit handling

## Task Commits

Each task was committed atomically:

1. **Task 1: Extract the watch request and service seam** - `5552d0f` (refactor)

**Plan metadata:** `0f8f0d4` (docs: plan watch runtime hardening)

## Files Created/Modified
- `src/app/watch.rs` - Defines `WatchRequest`, `WatchNotificationOptions`, `WatchService`, and the extracted watch helpers
- `src/app/mod.rs` - Exposes the new watch app module
- `src/main.rs` - Replaces the inline watch loop with typed request assembly and service invocation

## Decisions Made

- Preserved the existing watch notification payload and exit behavior while changing only the ownership boundary
- Let the watch service return `CommandOutcome` so the top-level CLI remains the only place that performs process exit

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- A broad `cargo fmt --all` run reformatted unrelated files, so those style-only diffs were trimmed back before the plan commit to keep the change focused

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan `08-02` can now harden notification timeouts and isolation inside one dedicated watch module instead of editing a monolithic `main.rs` branch
- Phase 11 already has the intended seam for future watch contract gating work

---
*Phase: 08-watch-runtime-hardening*
*Completed: 2026-04-04*
