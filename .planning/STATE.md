---
gsd_state_version: 1.0
milestone: v1.1
milestone_name: platform hardening and core expansion
status: Milestone complete
stopped_at: "Phase 12 complete; milestone audit and archive are next"
last_updated: "2026-04-04T23:35:34Z"
progress:
  total_phases: 6
  completed_phases: 6
  total_plans: 12
  completed_plans: 12
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-04-04)

**Core value:** Sumac must let AI systems understand and use real existing
tools and interfaces without bespoke glue, while staying fast, local-first,
and reliable.
**Current focus:** Milestone completion — audit and archive v1.1

## Current Position

Phase: Complete
Plan: Milestone closeout

## Performance Metrics

**Velocity:**

- Total plans completed: 10
- Total plans completed this milestone: 10
- Average duration: - min
- Total execution time: 0.0 hours

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| - | - | - | - |

**Recent Trend:**

- Last 5 plans: None yet
- Trend: Not enough data

## Accumulated Context

### Decisions

Decisions are logged in `.planning/PROJECT.md`.
Recent decisions affecting current work:

- v1.0 start: Lock down the golden-path contract baseline and parity harness
  before changing internals.

- v1.0 architecture: Introduce the core/app seam inside the existing repo and
  keep `SXMC_GOLDEN_PATH_ROUTE=legacy` as a rollback seam until soak evidence
  justifies removal.

- v1.0 migration: Move `status`, `sync`, `add`, and `setup` in that order and
  use direct core-vs-legacy parity as the proof pattern.

- v1.1 roadmap: Sequence work as canonical asset parity, watch runtime
  hardening, secure skill staging, unified scan/serve enforcement,
  command-family extraction, then soak-based rollback review.

- v1.1 coverage: Map every v1.1 requirement exactly once across Phases 7-12 and
  keep parity coverage plus soak evidence as mandatory cutover gates.

- Phase 07: Keep `scripts` and `references` compatibility views stable while
  making the canonical managed asset inventory recursive for later watch and
  policy work.

- Phase 10: Align scan and serve policy to canonical `Skill.assets` and treat
  managed file read or UTF-8 decode failures as explicit findings.

- Phase 11: Move `skills` and CLI-facing `serve` orchestration behind app
  services and require real-fixture contract coverage for the migrated paths.

- Phase 12: Keep migrated `watch` and `skills` as the sole route, but retain
  `SXMC_GOLDEN_PATH_ROUTE=legacy` intentionally until a later release-soak
  review justifies retirement.

### Pending Todos

Remaining after Phase 12:

- Audit milestone completion, archive v1.1 artifacts, and prepare the planning
  space for the next milestone.

### Blockers/Concerns

- Golden-path parity must stay stable across CLI output, JSON, artifacts, and
  release behavior during migration.

- The command-family migration is complete, but the top-level rollback seam
  remains intentionally available until the documented release-soak criterion
  is met.

- `watch` and `skills` still contain the most visible reliability and security
  risks outside the completed golden path migration.

- v1.1 phase plans are still TBD; parity coverage and soak evidence must be
  preserved as hard gates during implementation.

- The golden-path legacy seam is retained intentionally for now, so a later
  release-soak review must still revisit retirement explicitly.

## Session Continuity

Last session: 2026-04-04 13:19
Stopped at: Phase 12 complete; milestone audit and archive are next
Resume file: None
