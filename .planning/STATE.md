---
gsd_state_version: 1.0
milestone: v1.1
milestone_name: Platform Hardening and Core Expansion
status: Archived
stopped_at: "v1.1 archived; next step is /gsd-new-milestone"
last_updated: "2026-04-08T06:17:48Z"
progress:
  total_phases: 6
  completed_phases: 6
  total_plans: 12
  completed_plans: 12
  percent: 100
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-04-08)

**Core value:** Sumac must let AI systems understand and use real existing
tools and interfaces without bespoke glue, while staying fast, local-first,
and reliable.
**Current focus:** Planning the next milestone

## Current Position

Phase: Milestone archived
Plan: Next milestone definition

## Performance Metrics

**Velocity:**

- Total plans completed: 12
- Total plans completed this milestone: 12
- Average duration: Same-day phase execution within the milestone window
- Total execution time: Multi-session milestone

**By Phase:**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 7 | 2 | Complete | 1.0 phase slice |
| 8 | 2 | Complete | 1.0 phase slice |
| 9 | 2 | Complete | 1.0 phase slice |
| 10 | 2 | Complete | 1.0 phase slice |
| 11 | 2 | Complete | 1.0 phase slice |
| 12 | 2 | Complete | 1.0 phase slice |

**Recent Trend:**

- Last 5 plans: All completed
- Trend: Milestone closed cleanly with no open phase plans

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

Remaining after the archive:

- Define the next milestone with `$gsd-new-milestone`.
- Revisit retirement of `SXMC_GOLDEN_PATH_ROUTE=legacy` only after a later
  release-soak review.

### Blockers/Concerns

- The live planning tree has no active requirements file until the next
  milestone is started.
- The older golden-path legacy seam is retained intentionally for now, so a
  later release-soak review must still revisit retirement explicitly.
- Future migration slices should keep using real-fixture contract coverage
  before sole-route cutovers.

## Session Continuity

Last session: 2026-04-08 02:12
Stopped at: v1.1 archived and planning reset
Resume file: None
