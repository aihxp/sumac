---
phase: 12-soak-evidence-rollback-decision
plan: 02
subsystem: planning
tags: [rollback, decision, project-docs, governance, milestone]
requires: [12-01]
provides:
  - Explicit retained-or-retired rollback decision
  - Project-level documentation that `watch` and `skills` remain sole-route
  - Requirement/state tracking aligned to the rollback decision
affects: [milestone-closeout]
tech-stack:
  added: []
  patterns: [documented governance decision, explicit retention rationale]
key-files:
  created: []
  modified: [.planning/PROJECT.md, .planning/ROADMAP.md, .planning/REQUIREMENTS.md, .planning/STATE.md]
key-decisions:
  - "Keep `watch` / `skills` / CLI-facing `serve` as the sole route with no new rollback seam."
  - "Retain `SXMC_GOLDEN_PATH_ROUTE=legacy` intentionally for now because local validation is strong but true release-soak retirement evidence is still pending."
patterns-established:
  - "Legacy seams should survive only with explicit retained-or-retired language and a future re-evaluation trigger."
requirements-completed: [ROL-06]
duration: 2min
completed: 2026-04-04
---

# Phase 12: Soak Evidence & Rollback Decision Summary

**The rollback decision is now explicit: the migrated `watch` and `skills` route stays sole-route, while the older golden-path legacy seam is retained intentionally pending a later release-soak review**

## Performance

- **Duration:** 2 min
- **Started:** 2026-04-04T23:34:31Z
- **Completed:** 2026-04-04T23:35:34Z
- **Tasks:** 1
- **Files modified:** 4

## Accomplishments
- Updated project docs so maintainers can see that `watch`, `skills`, and CLI-facing `serve` stay on the migrated sole route
- Recorded that `SXMC_GOLDEN_PATH_ROUTE=legacy` remains intentionally available only for the older golden-path commands
- Set the future retirement trigger explicitly to a later release-soak review rather than same-session verification

## Task Commits

The rollback decision documentation landed in one focused documentation commit:

1. **Task 1: Record the retained-or-retired rollback decision** - `6229763` (docs)

**Plan metadata:** `7b9dc14` (docs: plan soak evidence and rollback decision)

## Files Created/Modified
- `.planning/PROJECT.md` - Records the retained legacy seam and sole-route status of the migrated families
- `.planning/ROADMAP.md` - Marks the phase complete
- `.planning/REQUIREMENTS.md` - Marks `ROL-06` complete
- `.planning/STATE.md` - Advances milestone state beyond the last phase

## Decisions Made

- Refused to retire the legacy seam based only on same-session verification because that would overstate the available evidence
- Refused to add any new rollback seam for `watch` or `skills` because the local contract evidence already supports them as the active sole route

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- The rollback decision had to separate the migrated-route status from the older golden-path seam, because they are related but not identical operational questions

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The milestone is ready for completion and archival work
- Future seam-retirement work now has an explicit evidence baseline and trigger instead of implicit carryover

---
*Phase: 12-soak-evidence-rollback-decision*
*Completed: 2026-04-04*
