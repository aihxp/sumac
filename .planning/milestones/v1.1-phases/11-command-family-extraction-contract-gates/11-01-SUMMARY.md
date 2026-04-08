---
phase: 11-command-family-extraction-contract-gates
plan: 01
subsystem: app
tags: [app, skills, main, extraction, orchestration]
requires: []
provides:
  - Dedicated `app::skills` request/service seam
  - Thin `Commands::Skills` request assembly in `main.rs`
  - Preserved `skills` family behavior through existing handlers and install pipeline
affects: [phase-12]
tech-stack:
  added: []
  patterns: [typed app service requests, thin main dispatch]
key-files:
  created: [src/app/skills.rs]
  modified: [src/app/mod.rs, src/main.rs]
key-decisions:
  - "Let `app::skills` orchestrate existing command handlers, generator, and install/update helpers instead of reimplementing behavior."
  - "Keep install-scope resolution in request assembly where needed, but move the command-family branching and printing path behind the app service."
patterns-established:
  - "Remaining command families should follow the same request/service seam already used by `watch` and the golden path."
requirements-completed: [CORE-06, CORE-07]
duration: 5min
completed: 2026-04-04
---

# Phase 11: Command-Family Extraction & Contract Gates Summary

**The `skills` family now runs through a dedicated app service, and `main.rs` only assembles typed requests before delegating the command behavior**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-04T23:25:55Z
- **Completed:** 2026-04-04T23:31:18Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Added `src/app/skills.rs` with typed request structs and a `SkillsService` that owns list, info, run, create, install, and update orchestration
- Replaced the large inline `Commands::Skills` branch in `src/main.rs` with thin request construction plus service delegation
- Kept the current `skills` behavior by reusing `command_handlers`, generator, and install/update logic rather than rewriting them

## Task Commits

The `skills` extraction and its supporting service/module wiring landed together in one implementation commit:

1. **Task 1: Introduce a `skills` app service and typed requests** - `23eef4a` (refactor)
2. **Task 2: Thin `main.rs` skills dispatch down to request assembly** - `23eef4a` (refactor)

**Plan metadata:** `bc6a192` (docs: plan command-family extraction and contract gates)

## Files Created/Modified
- `src/app/skills.rs` - New `skills` app service and typed request seam
- `src/app/mod.rs` - Registers the new app service module
- `src/main.rs` - Delegates `Commands::Skills` through the new service

## Decisions Made

- Preserved the existing lower-level helpers as the source of truth for `skills` behavior and moved only orchestration ownership
- Kept the service API async so the extracted family can support both sync and async subcommands without splitting the outward seam

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- The extraction touched both sync and async `skills` actions, so the cleanest seam was one enum-based request service rather than separate per-subcommand entrypoints

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The serve wrapper can now follow the same app-service pattern for a consistent remaining command-family boundary
- Phase 12 soak review can rely on `skills` no longer being a large inline hotspot in `main.rs`

---
*Phase: 11-command-family-extraction-contract-gates*
*Completed: 2026-04-04*
