# Roadmap: Sumac

## Milestones

- ✅ **v1.0 Product-Preserving Greenfield Core Rewrite** - Phases 1-6 (shipped 2026-04-05)
- 🚧 **v1.1 Platform Hardening and Core Expansion** - Phases 7-12 (in progress)

Archive:
- [v1.0 roadmap](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.0-ROADMAP.md)
- [v1.0 requirements](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.0-REQUIREMENTS.md)

## Overview

This milestone extends the internal rewrite beyond the golden path by
hardening `watch`, tightening skill install and serve policy, and moving the
next orchestration-heavy command families behind clearer seams without changing
the shipped `1.x` surface. The delivery order follows the dependency chain
from one canonical managed asset model, through watch runtime hardening and
secure skill staging, into enforced scan and serve policy, then contract-gated
command extraction, and finally a soak-based rollback decision.

## Phases

- [x] **Phase 7: Canonical Asset Inventory & Watch Parity** - Define the (completed 2026-04-04)
  managed skill asset surface and make nested watch behavior consistent across
  native-event and polling modes.
- [x] **Phase 8: Watch Runtime Hardening** - Keep long-running watch sessions (completed 2026-04-04)
  responsive while notify side effects and routing move behind a dedicated
  runtime seam.
- [x] **Phase 9: Secure Skill Materialization & Atomic Activation** - Stage (completed 2026-04-04)
  local and git-backed skills ephemerally, enforce the allowlist before
  activation, and prevent partial installs.
- [ ] **Phase 10: Unified Scan & Serve Enforcement** - Align installed,
  scanned, and served skill inventories under one enforced managed asset set.
- [ ] **Phase 11: Command-Family Extraction & Contract Gates** - Move the
  remaining `watch` and `skills` orchestration off top-level hotspots while
  preserving `1.x` behavior under real-fixture parity checks.
- [ ] **Phase 12: Soak Evidence & Rollback Decision** - Keep or retire the
  rollback seam only after explicit release-soak evidence proves the new route
  is safe.

## Phase Details

### Phase 7: Canonical Asset Inventory & Watch Parity
**Goal**: Users can rely on one canonical managed skill asset inventory for
watch invalidation across native-event and polling modes.
**Depends on**: Phase 6
**Requirements**: WATCH-01, WATCH-02
**Success Criteria** (what must be TRUE):
  1. User can change a nested managed skill asset and see the watch flow
     invalidate or reload from the actual managed serve surface in native-event
     mode.
  2. User can make the same nested managed asset change in polling mode and get
     the same correctness outcome or an explicit degraded-state or rescan
     signal instead of a silent miss.
  3. User can rely on `sxmc watch` tracking the managed skill asset surface
     consistently rather than only top-level files.
**Plans**: 2 plans

### Phase 8: Watch Runtime Hardening
**Goal**: Long-running `sxmc watch` sessions stay responsive while notify side
effects and watch orchestration run through a dedicated runtime seam.
**Depends on**: Phase 7
**Requirements**: WATCH-03, CORE-05
**Success Criteria** (what must be TRUE):
  1. User can leave `sxmc watch` running and still receive reloads even when a
     notify command or webhook is slow or fails.
  2. User can see timeout or failure reporting for notify side effects without
     the main watch loop stalling or exiting unexpectedly.
  3. User can keep using the current `sxmc watch` CLI flags, JSON output, exit
     semantics, and wrapper entrypoints after the runtime seam is introduced.
**Plans**: 2 plans

### Phase 9: Secure Skill Materialization & Atomic Activation
**Goal**: Users can install or update local and git-backed skills through
ephemeral staging that activates only validated allowlisted contents.
**Depends on**: Phase 8
**Requirements**: SKILL-01, SKILL-02, SKILL-03
**Success Criteria** (what must be TRUE):
  1. User can install or update a skill from a local path, git repo root, or
     git repo subpath and get the intended managed skill contents activated.
  2. User sees symlinks plus hidden, VCS, and build-artifact content rejected
     before the skill becomes active.
  3. User never ends up with a partially activated skill after a failed install
     or update attempt.
**Plans**: 2 plans

### Phase 10: Unified Scan & Serve Enforcement
**Goal**: Installed skill contents are scanned and served under one enforced
managed asset policy.
**Depends on**: Phase 9
**Requirements**: SEC-01, SEC-02
**Success Criteria** (what must be TRUE):
  1. User can verify that Sumac serves only the approved managed skill asset
     set after install or update.
  2. User gets an explicit finding when a served or executable skill file is
     unreadable or non-UTF-8 instead of it being silently skipped during scan
     coverage.
  3. User can rely on the installed, scanned, and served skill inventories
     matching for an active managed skill.
**Plans**: 2 plans

### Phase 11: Command-Family Extraction & Contract Gates
**Goal**: `watch` and `skills` run through clearer service boundaries without
breaking existing `1.x` command contracts.
**Depends on**: Phase 10
**Requirements**: CORE-06, CORE-07, ROL-05
**Success Criteria** (what must be TRUE):
  1. User can run `sxmc skills install`, `update`, `list`, `info`, and `serve`
     with the same observable CLI behavior, JSON output, exit codes, and file
     side effects as before the extraction.
  2. User can run migrated `watch` and `skills` flows against real fixtures and
     see parity checks pass before the new paths become the default route.
  3. User can keep existing wrappers and automation pointed at `watch` and
     `skills` without contract regressions after shared orchestration moves out
     of top-level dispatch hotspots.
**Plans**: TBD

### Phase 12: Soak Evidence & Rollback Decision
**Goal**: The migrated watch and skills route is kept or rolled back based on
explicit release-soak evidence rather than assumption.
**Depends on**: Phase 11
**Requirements**: ROL-06
**Success Criteria** (what must be TRUE):
  1. Maintainers can point to explicit release-soak evidence showing whether
     the migrated watch and skills path is safe as the sole route.
  2. Users either see the rollback seam retired after safe soak or retained
     intentionally with documented justification, not by default inertia.
**Plans**: TBD

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 7. Canonical Asset Inventory & Watch Parity | 2/2 | Complete   | 2026-04-04 |
| 8. Watch Runtime Hardening | 2/2 | Complete   | 2026-04-04 |
| 9. Secure Skill Materialization & Atomic Activation | 2/2 | Complete   | 2026-04-04 |
| 10. Unified Scan & Serve Enforcement | 0/2 | Not started | - |
| 11. Command-Family Extraction & Contract Gates | 0/TBD | Not started | - |
| 12. Soak Evidence & Rollback Decision | 0/TBD | Not started | - |
