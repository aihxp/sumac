# Requirements: Sumac

**Defined:** 2026-04-04
**Core Value:** Sumac must let AI systems understand and use real existing
tools and interfaces without bespoke glue, while staying fast, local-first,
and reliable.

## v1 Requirements

Requirements for milestone `v1.1 Platform Hardening and Core Expansion`.
Each will map to exactly one roadmap phase.

### Watch Reliability

- [ ] **WATCH-01**: Sumac detects nested skill-asset changes across the actual
  managed serve surface in both native-event and fallback watch modes.
- [ ] **WATCH-02**: Sumac treats polling as a supported degraded mode with the
  same correctness guarantees as native events, including explicit rescan or
  degraded-state behavior instead of silent misses.
- [ ] **WATCH-03**: Sumac executes watch notify commands and webhooks with
  bounded async behavior, explicit timeouts, and isolated failures so one slow
  side effect does not stall the main watch loop.

### Skill Lifecycle Hardening

- [ ] **SKILL-01**: Sumac materializes local and git-backed skill sources into
  ephemeral staging, resolves repo-root and subpath inputs correctly, and
  cleans up transient staging data after install or update.
- [ ] **SKILL-02**: Sumac installs and updates only an explicit allowlisted
  skill payload, rejecting symlinks plus hidden, VCS, and build-artifact
  content before activation.
- [ ] **SKILL-03**: Sumac applies skill installs and updates atomically so a
  failed validation never leaves a partially activated skill in place.

### Security & Serve Policy

- [ ] **SEC-01**: Sumac scans every install-surface file that may later be
  served or executed, and unreadable or non-UTF-8 content becomes an explicit
  finding instead of a silent skip.
- [ ] **SEC-02**: Sumac serves only the approved managed skill asset set, and
  the installed, scanned, and served inventories stay aligned.

### Command Architecture

- [ ] **CORE-05**: `sxmc watch` runs through a dedicated app or runtime seam
  that preserves current CLI behavior, JSON output, exit semantics, and wrapper
  compatibility while removing watch orchestration from top-level dispatch
  hotspots.
- [ ] **CORE-06**: The `skills` command family runs through clearer service
  boundaries that preserve current install, update, list, info, and serve
  behavior while reducing `src/main.rs` and adjacent orchestration coupling.
- [ ] **CORE-07**: Shared watch or skills orchestration helpers move into
  scoped modules so `src/main.rs` remains primarily responsible for CLI parsing,
  composition, and dispatch rather than subsystem business logic.

### Rollout & Evidence

- [ ] **ROL-05**: Migrated watch and skills flows have contract or parity
  coverage on real fixtures before full cutover, including output, exit-code,
  and file-side-effect checks where applicable.
- [ ] **ROL-06**: The rollback seam is retained or retired only after explicit
  release-soak evidence shows the new path is safe to keep as the sole route.

## v2 Requirements

Deferred follow-on work once the v1.1 safety baseline is in place.

### Diagnostics & Provenance

- **DIAG-01**: Operators can inspect watch backend health, last successful
  reload, and degraded-mode reasons without digging through ad hoc logs.
- **SKILL-04**: Sumac can preview install provenance and the resulting serve
  surface before activating a remote skill.

### Broader Policy & Diff Tooling

- **TRUST-01**: Teams can enforce origin and pinning policy for remote skill
  installs.
- **NEXT-04**: Maintainers can run generalized parity-diff tooling for command
  families beyond `watch` and `skills`.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Public `1.x` CLI redesign or JSON contract rewrite | v1.1 is internal hardening and seam work, not a product reset |
| Big-bang migration of every remaining command family | The rewrite must keep landing in stable slices |
| Stack or framework replacement (`git2`, new runtime, DI framework, etc.) | Research showed the current stack is sufficient; disciplined usage matters more than churn |
| Recursive copy of full remote skill trees | v1.1 must narrow the trust boundary, not preserve unsafe extras for convenience |
| Auto-serving newly installed remote skills by default | Acquisition, validation, and exposure should stay explicit and reviewable |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| WATCH-01 | Phase 7 | Complete |
| WATCH-02 | Phase 7 | Complete |
| WATCH-03 | Phase 8 | Complete |
| CORE-05 | Phase 8 | Complete |
| SKILL-01 | Phase 9 | Pending |
| SKILL-02 | Phase 9 | Pending |
| SKILL-03 | Phase 9 | Pending |
| SEC-01 | Phase 10 | Pending |
| SEC-02 | Phase 10 | Pending |
| CORE-06 | Phase 11 | Pending |
| CORE-07 | Phase 11 | Pending |
| ROL-05 | Phase 11 | Pending |
| ROL-06 | Phase 12 | Pending |

**Coverage:**
- v1 requirements: 13 total
- Mapped to phases: 13
- Unmapped: 0

---
*Requirements defined: 2026-04-04*
*Last updated: 2026-04-04 after v1.1 roadmap creation*
