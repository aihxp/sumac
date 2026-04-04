---
gsd_state_version: 1.0
milestone: v1.1
milestone_name: platform hardening and core expansion
status: defining-requirements
stopped_at: Milestone v1.1 opened; defining requirements and roadmap
last_updated: "2026-04-04T14:48:12Z"
progress:
  total_phases: 0
  completed_phases: 0
  total_plans: 0
  completed_plans: 0
---

# Project State

## Project Reference

See: `.planning/PROJECT.md` (updated 2026-04-04)

**Core value:** Sumac must let AI systems understand and use real existing
tools and interfaces without bespoke glue, while staying fast, local-first,
and reliable.
**Current focus:** Milestone v1.1 - Platform Hardening and Core Expansion

## Current Position

Phase: Not started (defining requirements)
Plan: -
Status: Defining requirements
Last activity: 2026-04-04 - Milestone v1.1 started

Progress: [░░░░░░░░░░] 0%

## Performance Metrics

**Velocity:**

- Total plans completed: 10
- Total plans completed this milestone: 0
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

- Phase 1: Start the rewrite by locking down the golden-path contract baseline
  and parity harness.
- Phase 1 complete: Use `docs/GOLDEN_PATH_CONTRACT.md` plus the rewrite parity
  checks as the migration source of truth for the golden path.

- Phase 2: Introduce the core/app seam inside the existing repo rather than
  creating a parallel product tree.

- Phase 2 complete: Use `src/app/golden_path.rs` as the canonical orchestration
  entrypoint for the golden path while `SXMC_GOLDEN_PATH_ROUTE=legacy` remains
  as a rollback seam until later phases retire it.

- Phase 3 complete: Treat `src/app/status.rs` as the first dedicated migrated
  command slice and use direct core-vs-legacy parity as the proof pattern for
  later migrations.

- Phase 4 complete: Treat `src/app/sync.rs` as the write-side companion to the
  status slice and keep core-vs-legacy parity as the migration proof pattern.

- Phase 5 complete: Treat `src/app/add.rs` plus `src/app/onboarding.rs` as the
  onboarding migration pattern and reuse that shared service when rebuilding
  `setup`.

- Phase 6 complete: Treat `src/app/setup.rs` as the final golden-path slice and
  `src/app/golden_path.rs` as dispatch-only across the maintained workflow.

- Phase 3-6: Migrate `status`, then `sync`, then `add`, then `setup` in that
  order to preserve the maintained onboarding path.

### Pending Todos

No scoped milestone todos yet. Requirements and roadmap are being defined.

### Blockers/Concerns

- Golden-path parity must stay stable across CLI output, JSON, artifacts, and
  release behavior during migration.

- The command-family migration is complete, but the top-level rollback seam
  remains intentionally available until the documented release-soak criterion
  is met.

- `watch` and `skills` still contain the most visible reliability and security
  risks outside the completed golden path migration.

## Session Continuity

Last session: 2026-04-04 10:48
Stopped at: Milestone v1.1 kickoff and research handoff
Resume file: None
