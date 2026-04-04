---
phase: 09-secure-skill-materialization-atomic-activation
plan: 02
subsystem: skills
tags: [skills, allowlist, policy, symlink, trust-boundary]
requires: [09-01]
provides:
  - Managed-asset-only staged skill payloads
  - Explicit rejection of symlinked, hidden, VCS, and build-artifact managed assets
  - Regression coverage for unsafe payload rejection
affects: [phase-10]
tech-stack:
  added: []
  patterns: [canonical-asset-driven install policy, pre-activation payload validation]
key-files:
  created: []
  modified: [src/skills/install.rs]
key-decisions:
  - "Drive install payload selection from the canonical managed asset inventory instead of recursive whole-tree copy."
  - "Reject unsafe content only when it appears in the managed payload, allowing git repos to contain sibling `.git` metadata outside the installed skill payload."
patterns-established:
  - "Install trust boundaries should be defined by Sumac-managed assets, not by the source tree layout."
requirements-completed: [SKILL-02]
duration: 10min
completed: 2026-04-04
---

# Phase 9: Secure Skill Materialization & Atomic Activation Summary

**Installed skill payloads now come only from the canonical managed asset inventory, and unsafe managed files are rejected before a skill can become active**

## Performance

- **Duration:** 10 min
- **Started:** 2026-04-04T23:09:08Z
- **Completed:** 2026-04-04T23:12:44Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments
- Replaced recursive whole-tree copy with staged payload construction from `Skill.assets`
- Added managed-asset validation that rejects symlinks plus hidden, VCS, and build-artifact path components before activation
- Added regression tests proving unsafe managed payloads fail and do not reach the active skill root

## Task Commits

The allowlist enforcement and its tests landed together inside the install pipeline refactor:

1. **Task 1: Stage only managed assets and managed metadata** - `c8d5fbf` (feat)
2. **Task 2: Reject unsafe payload contents before activation** - `c8d5fbf` (feat)

**Plan metadata:** `df3f0b5` (docs: plan secure skill materialization)

## Files Created/Modified
- `src/skills/install.rs` - Selects staged payloads from canonical managed assets and validates unsafe managed content before activation

## Decisions Made

- Treated extra non-managed files in the source tree as out of install scope, while failing only when unsafe content appears inside the managed payload itself
- Preserved `.sxmc-source.json` as Sumac-managed provenance written during staging rather than copied from the source tree

## Deviations from Plan

None - plan executed exactly as written

## Issues Encountered

- Git clones always contain a root `.git` directory, so the install policy had to reject unsafe managed payload entries rather than failing any source tree that merely contains VCS metadata outside the allowlist

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 10 can reuse the managed payload boundary for scan and serve enforcement instead of inventing a separate file policy
- Future `skills` extraction work now has a clearer policy seam inside the install pipeline

---
*Phase: 09-secure-skill-materialization-atomic-activation*
*Completed: 2026-04-04*
