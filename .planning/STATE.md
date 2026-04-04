---
gsd_state_version: 1.0
milestone: v1.1
milestone_name: platform hardening and core expansion
status: Ready to plan
stopped_at: "Phase 08 complete; Phase 09 discuss and planning are next"
last_updated: "2026-04-04T17:44:34Z"
progress:
  total_phases: 6
  completed_phases: 2
  total_plans: 4
  completed_plans: 4
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-04-04)

**Core value:** Sumac must let AI systems understand and use real existing
tools and interfaces without bespoke glue, while staying fast, local-first,
and reliable.
**Current focus:** Phase 09 — Secure Skill Materialization & Atomic Activation

## Current Position

Phase: 9
Plan: Not started

## Performance Metrics

**Velocity:**

- Total plans completed: 10
- Total plans completed this milestone: 4
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

### Pending Todos

Remaining in Phase 09:

- Gather context, research, and execution plans for secure skill staging and atomic activation.

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

- Phase 09 must narrow the install payload to managed assets without leaving
  partially activated skills behind after failures.

## Session Continuity

Last session: 2026-04-04 13:19
Stopped at: Phase 08 complete; Phase 09 discuss and planning are next
Resume file: None
