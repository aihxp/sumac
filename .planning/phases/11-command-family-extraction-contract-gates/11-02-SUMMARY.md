---
phase: 11-command-family-extraction-contract-gates
plan: 02
subsystem: app
tags: [app, serve, contract-tests, stdio, watch]
requires: [11-01]
provides:
  - Dedicated `app::serve` request/service seam
  - Real-fixture contract coverage for extracted `skills` and `serve` flows
  - End-to-end install-to-serve regression coverage
affects: [phase-12]
tech-stack:
  added: []
  patterns: [cli wrapper service seam, real-fixture contract gates]
key-files:
  created: [src/app/serve.rs]
  modified: [src/app/mod.rs, src/main.rs, tests/cli_integration.rs]
key-decisions:
  - "Keep `server::serve_stdio/http` as the real server boundary and make `app::serve` only own CLI wrapper orchestration."
  - "Use real temp installs and real fixture serve flows as the contract gate instead of synthetic-only tests."
patterns-established:
  - "Migration safety for extracted command families should be proven with fixture-driven CLI integration tests, not assumed from compilation alone."
requirements-completed: [ROL-05]
duration: 5min
completed: 2026-04-04
---

# Phase 11: Command-Family Extraction & Contract Gates Summary

**The CLI-facing `serve` wrapper now has its own app seam, and the migrated `skills` plus `serve` paths are pinned by real-fixture contract tests**

## Performance

- **Duration:** 5 min
- **Started:** 2026-04-04T23:25:55Z
- **Completed:** 2026-04-04T23:31:18Z
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Added `src/app/serve.rs` so registration setup, auth parsing, and transport dispatch no longer live inline in `main.rs`
- Converted `Commands::Serve` into thin request assembly and service delegation
- Added an end-to-end install-to-serve roundtrip regression and verified the existing stdio hybrid and HTTP watch reload flows still pass after extraction

## Task Commits

The `serve` seam and contract-gate coverage landed together in one implementation commit:

1. **Task 1: Move top-level `serve` orchestration behind `app::serve`** - `23eef4a` (refactor)
2. **Task 2: Add real-fixture contract gates for migrated `skills` and `serve` flows** - `23eef4a` (refactor)

**Plan metadata:** `bc6a192` (docs: plan command-family extraction and contract gates)

## Files Created/Modified
- `src/app/serve.rs` - New CLI-facing serve service
- `src/app/mod.rs` - Registers the new serve module
- `src/main.rs` - Delegates `Commands::Serve` through the new service
- `tests/cli_integration.rs` - Adds end-to-end install-to-serve contract coverage

## Decisions Made

- Kept the extracted serve seam focused on CLI orchestration so earlier phases’ hardened server internals stayed untouched
- Treated the existing hybrid MCP and watch reload integration tests as first-class contract gates and supplemented them with one install-to-serve roundtrip

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- Pulling `HttpServeLimits` out of the main serve branch initially exposed a broader shared import dependency in `main.rs`, so the fix was to restore the shared import rather than duplicate the type in app code

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 12 soak review can evaluate the migrated route with explicit contract evidence instead of only architectural intent
- The remaining milestone work can focus on soak evidence and rollback decision-making rather than additional boundary cleanup

---
*Phase: 11-command-family-extraction-contract-gates*
*Completed: 2026-04-04*
