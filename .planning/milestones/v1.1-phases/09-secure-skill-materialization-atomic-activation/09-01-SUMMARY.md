---
phase: 09-secure-skill-materialization-atomic-activation
plan: 01
subsystem: skills
tags: [skills, install, git, staging, atomic]
requires: []
provides:
  - Correct git repo root and repo-subpath materialization in scoped temp storage
  - Atomic staged activation that preserves the previous install on failure
  - Install lifecycle regressions for git materialization and failure preservation
affects: [phase-10, phase-11]
tech-stack:
  added: []
  patterns: [scoped source materialization, stage-then-activate lifecycle]
key-files:
  created: []
  modified: [src/skills/install.rs]
key-decisions:
  - "Keep transient clone directories scoped inside `TempDir` ownership instead of persisting them with `keep()`."
  - "Stage the next install fully before touching the active target, then restore the previous target if the final swap fails."
patterns-established:
  - "Install/update lifecycle helpers should treat source materialization and activation as distinct steps."
requirements-completed: [SKILL-01, SKILL-03]
duration: 10min
completed: 2026-04-04
---

# Phase 9: Secure Skill Materialization & Atomic Activation Summary

**Git and local skill sources now materialize through scoped staging, and install/update activation preserves the previous live skill if the replacement cannot be activated cleanly**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-04T23:09:08Z
- **Completed:** 2026-04-04T23:12:44Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Replaced leaked `temp.keep()` clone handling with scoped materialization that resolves repo roots and repo subpaths from the actual clone directory
- Refactored staged activation so replacement content is prepared before the active target is touched
- Added lifecycle tests proving git root/subpath materialization works and failed updates keep the previous installed skill intact

## Task Commits

The implementation landed in one focused commit because the lifecycle, staging, and verification changes all live in the same install pipeline file:

1. **Task 1: Correct source materialization for local and git-backed installs** - `c8d5fbf` (feat)
2. **Task 2: Make activation preserve the previous install on failure** - `c8d5fbf` (feat)

**Plan metadata:** `df3f0b5` (docs: plan secure skill materialization)

## Files Created/Modified
- `src/skills/install.rs` - Introduces scoped materialization ownership, staged payload construction, and failure-safe activation tests

## Decisions Made

- Held git clone tempdirs alive through a `MaterializedSkillSource` wrapper so staged copies can safely read clone contents without leaking them
- Used a staging-local backup rename for updates so the previous target can be restored if the final activation rename fails

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- The install pipeline tasks overlapped heavily inside one file, so the safest atomic change was one cohesive implementation commit rather than multiple partial file commits

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Plan `09-02` can now enforce the managed allowlist on top of a correct source and activation lifecycle
- Later serve/scan policy work can rely on installs already being staged and failure-safe

---
*Phase: 09-secure-skill-materialization-atomic-activation*
*Completed: 2026-04-04*
