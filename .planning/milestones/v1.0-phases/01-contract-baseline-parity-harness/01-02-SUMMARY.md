---
phase: 01-contract-baseline-parity-harness
plan: 02
subsystem: testing
tags: [rewrite, parity, validation, shell, integration]
requires:
  - phase: 01-01
    provides: golden-path contract inventory for the maintained lifecycle
provides:
  - Explicit Rust rewrite parity checks for `setup`, `add`, `status`, and `sync`
  - Shell-level rewrite parity checks in the broad regression suite
  - Validation and compatibility docs that point maintainers to the rewrite proof path
affects: [phase-02, phase-03, phase-04, phase-05, phase-06]
tech-stack:
  added: []
  patterns: [rewrite parity naming, reuse of existing validation layers]
key-files:
  created: []
  modified: [tests/cli_integration.rs, scripts/test-sxmc.sh, docs/VALIDATION.md, docs/COMPATIBILITY_MATRIX.md]
key-decisions:
  - "Keep rewrite parity inside the existing Rust and shell validation stack instead of creating a separate harness."
  - "Name the new Rust checks `test_rewrite_golden_path_*` so later migration phases can target them directly."
patterns-established:
  - "Rewrite phases should extend existing validation layers, not fork them."
requirements-completed: [PAR-02, ROL-04]
duration: 40min
completed: 2026-04-04
---

# Phase 1: Contract Baseline & Parity Harness Summary

**Explicit rewrite parity proof for `setup`, `add`, `status`, and `sync` added to Rust tests, the shell suite, and maintained validation docs**

## Performance

- **Duration:** 40 min
- **Started:** 2026-04-04T05:26:00-04:00
- **Completed:** 2026-04-04T06:06:00-04:00
- **Tasks:** 2
- **Files modified:** 4

## Accomplishments
- Added named Rust rewrite parity checks for the maintained golden path
- Added a shell-level rewrite parity block to `scripts/test-sxmc.sh`
- Updated validation and compatibility docs so later migration phases know exactly where parity is enforced
- Verified the phase with `cargo test`, Clippy, and a clean `332/0/0` shell suite run

## Task Commits

Each task was committed atomically:

1. **Task 1: Add explicit rewrite parity checks to Rust integration coverage** - `d640fdd` (test)
2. **Task 2: Add shell-level rewrite regression tracking and document it** - `a973e2e` (test)

**Plan metadata:** `a08e166` (docs: create phase plans)

## Files Created/Modified
- `tests/cli_integration.rs` - Rewrite-focused golden-path parity tests
- `scripts/test-sxmc.sh` - Shell parity checks for the maintained lifecycle
- `docs/VALIDATION.md` - Rewrite baseline and proof-path documentation
- `docs/COMPATIBILITY_MATRIX.md` - Rewrite parity coverage note in the compatibility story

## Decisions Made

- Reused the existing validation stack instead of building a rewrite-only harness
- Made the rewrite proof path obvious through test naming and maintained doc links

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- The first `setup` parity assertion guessed the wrong nested field and was corrected to the actual stable `tool` field after checking real output

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 1 now has both a contract baseline and executable parity proof
- Phase 2 can start carving out the new core/app seam with a stable migration reference and proof harness already in place

---
*Phase: 01-contract-baseline-parity-harness*
*Completed: 2026-04-04*
