---
gsd_state_version: 1.0
milestone: v1.1
milestone_name: platform hardening and core expansion
status: Ready to plan
stopped_at: "Phase 10 complete; Phase 11 discuss and planning are next"
last_updated: "2026-04-04T23:21:25Z"
progress:
  total_phases: 6
  completed_phases: 4
  total_plans: 8
  completed_plans: 8
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-04-04)

**Core value:** Sumac must let AI systems understand and use real existing
tools and interfaces without bespoke glue, while staying fast, local-first,
and reliable.
**Current focus:** Phase 11 — Command-Family Extraction & Contract Gates

## Current Position

Phase: 11
Plan: Not started

## Performance Metrics

**Velocity:**

- Total plans completed: 10
- Total plans completed this milestone: 8
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

### Pending Todos

Remaining in Phase 11:

- Gather context, research, and execution plans for command-family extraction
  and contract gates.

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

- Phase 11 must preserve `skills` and `watch` command contracts while moving
  more orchestration behind clearer seams and parity gates.

## Session Continuity

Last session: 2026-04-04 13:19
Stopped at: Phase 10 complete; Phase 11 discuss and planning are next
Resume file: None
