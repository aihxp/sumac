---
phase: 07-canonical-asset-inventory-watch-parity
plan: 02
subsystem: watch
tags: [watch, polling, fingerprint, assets, parity]
requires: [07-01]
provides:
  - Polling watch fingerprinting anchored to the canonical managed asset inventory
  - Nested managed asset regression coverage for `scripts/**` and `references/**`
  - A shared correctness surface for native-event and polling watch modes
affects: [phase-08, phase-10, phase-11]
tech-stack:
  added: []
  patterns: [canonical asset fingerprinting, parser-driven watch invalidation]
key-files:
  created: []
  modified: [src/server/mod.rs]
key-decisions:
  - "Treat `parse_skill(...).assets` as the primary source of truth for polling fingerprints, with shallow directory hashing retained only as a fallback when parsing fails."
  - "Prove nested managed-asset parity with direct regression tests instead of relying on manual watch exercise."
patterns-established:
  - "Watch correctness should consume the canonical managed asset inventory rather than duplicate traversal rules."
requirements-completed: [WATCH-02]
duration: 10min
completed: 2026-04-04
---

# Phase 7: Canonical Asset Inventory & Watch Parity Summary

**Polling watch invalidation now fingerprints the canonical managed asset inventory, with regression coverage proving nested managed assets change the watch fingerprint**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-04T17:22:21Z
- **Completed:** 2026-04-04T17:27:05Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Refactored polling fingerprint generation to consume the canonical managed asset inventory from the skill parser
- Preserved the previous shallow hash path as a defensive fallback only when skill parsing fails
- Added regression tests proving nested script and nested reference changes alter the watch fingerprint

## Task Commits

Each task was committed atomically:

1. **Task 1: Fingerprint the canonical managed asset inventory** - `8214c6b` (fix)
2. **Task 2: Add nested-asset polling parity regression tests** - `6b7e07f` (test)

**Plan metadata:** `248ac6d` (docs: create execution plans)

## Files Created/Modified
- `src/server/mod.rs` - Fingerprints canonical managed assets and adds nested polling parity tests

## Decisions Made

- Routed polling fingerprint correctness through `parse_skill(...).assets` so native and polling watch logic can converge on the same managed surface
- Kept the shallow hashing branch only as parse-failure fallback to avoid turning parser failures into silent watch crashes

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- The draft verification command in the plan used multiple substring filters in one `cargo test` invocation, so verification used the valid single-filter form `cargo test --quiet compute_skill_fingerprint_changes_when`

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 8 can harden the runtime and notify path without inheriting a shallow top-level-only fingerprint model
- Later scan and serve policy work can rely on watch correctness already being tied to the managed asset inventory

---
*Phase: 07-canonical-asset-inventory-watch-parity*
*Completed: 2026-04-04*
