# Project Retrospective

*A living document updated after each milestone. Lessons feed forward into
future planning.*

## Milestone: v1.1 — Platform Hardening and Core Expansion

**Shipped:** 2026-04-08
**Phases:** 6 | **Plans:** 12 | **Sessions:** Multiple autonomous batches

### What Was Built
- canonical managed skill asset policy shared by watch, install, scan, and
  serve flows
- a dedicated runtime seam for `sxmc watch` with bounded notification side
  effects
- staged, atomic skill activation for local and git-backed installs
- dedicated app services and contract tests for `skills` and CLI-facing
  `serve`

### What Worked
- mapping each requirement to exactly one phase kept scope and verification
  crisp
- real-fixture contract tests gave the migration a practical cutover gate
- keeping the milestone audit separate from the phase work made the final
  recommendation easy to justify

### What Was Inefficient
- live requirement checkboxes drifted from the traceability table and had to be
  repaired during archive
- the helper-produced archive files still needed manual cleanup for summaries,
  links, and next-state wording

### Patterns Established
- define one canonical policy surface before hardening its downstream consumers
- move CLI families behind typed request assembly and scoped app services
- require contract or parity evidence before a migrated path becomes sole-route

### Key Lessons
1. Close requirement state as part of phase completion, not just at milestone
   archive time.
2. If archival moves evidence files, patch their links in the same closeout
   commit.

### Cost Observations
- Model mix: balanced-profile autonomous execution plus manual archive curation
- Sessions: multiple autonomous batches
- Notable: separating code delivery from milestone closeout kept implementation
  fast, but made the archive pass more detail-sensitive

---

## Cross-Milestone Trends

### Process Evolution

| Milestone | Sessions | Phases | Key Change |
|-----------|----------|--------|------------|
| v1.0 | Multiple | 6 | Established the internal core/app seam and golden-path parity model |
| v1.1 | Multiple | 6 | Reused that seam pattern for operational hardening and contract-gated cutover |

### Cumulative Quality

| Milestone | Validation Style | Coverage Focus | Operational Outcome |
|-----------|------------------|----------------|---------------------|
| v1.0 | Parity-first | Golden path contracts and generated artifacts | Stable core/app rewrite foundation |
| v1.1 | Contract-plus-soak evidence | Watch, skills, serve, and rollback governance | Stable operational seam expansion |

### Top Lessons (Verified Across Milestones)

1. Stable external contracts make aggressive internal rewrites shippable.
2. Thin dispatch plus typed service seams scales better than growing `main.rs`
   orchestration.
