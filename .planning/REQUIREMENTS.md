# Requirements: Sumac

**Defined:** 2026-04-04
**Core Value:** Sumac must let AI systems understand and use real existing
tools and interfaces without bespoke glue, while staying fast, local-first,
and reliable.

## v1 Requirements

Requirements for the initial rewrite milestone. Each maps to exactly one phase
in the roadmap.

### Contracts & Parity

- [ ] **PAR-01**: The project maintains a published interface inventory for the
  rewrite target covering commands, flags, aliases, env vars, exit codes, JSON
  fields, generated files, and MCP-facing behavior for `setup`, `add`,
  `status`, and `sync`.
- [ ] **PAR-02**: The rewrite includes characterization coverage for the golden
  path using real CLI and artifact-level tests before migrated logic becomes the
  default path.
- [ ] **PAR-03**: Migrated command families preserve existing stdout/stderr
  separation, exit behavior, and stable `1.x` JSON output contracts.
- [ ] **PAR-04**: Migrated command families preserve generated host artifacts
  and state file behavior for both local and global install scopes.

### Core Architecture

- [ ] **CORE-01**: Sumac introduces a new internal core/app layer inside the
  existing repo that becomes the canonical orchestration boundary for migrated
  command families.
- [ ] **CORE-02**: `src/main.rs` is reduced to stable CLI dispatch and runtime
  composition responsibilities rather than containing golden-path business logic
  directly.
- [ ] **CORE-03**: The new core uses typed internal request/result contracts
  for migrated flows instead of loosely shaped orchestration through
  ad-hoc values.
- [ ] **CORE-04**: Infrastructure-heavy concerns such as filesystem mutation,
  subprocess execution, HTTP/MCP transport, and environment/path resolution are
  accessed through explicit adapters or ports from the migrated core.

### Golden Path Migration

- [ ] **PATH-01**: `status` runs through the new core/app path while preserving
  current behavior and output contracts.
- [ ] **PATH-02**: `sync` runs through the new core/app path while preserving
  current behavior, state mutation, and generated artifact behavior.
- [ ] **PATH-03**: `add` runs through the new core/app path while preserving
  current onboarding behavior, profile generation, host detection, and file
  outputs.
- [ ] **PATH-04**: `setup` runs through the new core/app path and reuses the
  same migrated onboarding/reconciliation services rather than duplicating its
  own orchestration logic.

### Rollout & Operations

- [ ] **ROL-01**: The rewrite ships incrementally without freezing releases,
  allowing mixed old/new internals in stable releases during migration.
- [ ] **ROL-02**: Each migrated slice has an explicit cutover mechanism and a
  rollback-safe routing strategy before legacy logic is retired.
- [ ] **ROL-03**: Temporary compatibility shims introduced during migration
  have explicit retirement criteria so the rewrite does not end with permanent
  dual implementations.
- [ ] **ROL-04**: Rewrite validation tracks performance, reliability, and
  cross-platform parity for the migrated golden path so architectural cleanup
  does not silently degrade the shipped product.

## v2 Requirements

Deferred to later rewrite waves after the first golden-path milestone lands.

### Broader Migration

- **NEXT-01**: Move adjacent maintenance flows such as `doctor` and `watch`
  onto the same core/app model once the golden path is stable.
- **NEXT-02**: Extend the same migration pattern to discovery, wrapping,
  serving, and skills lifecycle families after the first architecture seam is
  proven.
- **NEXT-03**: Introduce a repeatable migration scorecard and deeper
  side-by-side diffing across additional command families.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Public CLI redesign during the rewrite | The rewrite must preserve the shipped `1.x` surface first; CLI/product changes are separate decisions |
| Product narrowing or category reset | This project is a product-preserving rewrite, not a new-product search |
| Big-bang replacement of all subsystems | The migration must be slice-by-slice so releases and rollback remain possible |
| Broad dependency churn unrelated to migration goals | Simultaneous stack and architecture churn would make regressions hard to classify |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| PAR-01 | — | Pending |
| PAR-02 | — | Pending |
| PAR-03 | — | Pending |
| PAR-04 | — | Pending |
| CORE-01 | — | Pending |
| CORE-02 | — | Pending |
| CORE-03 | — | Pending |
| CORE-04 | — | Pending |
| PATH-01 | — | Pending |
| PATH-02 | — | Pending |
| PATH-03 | — | Pending |
| PATH-04 | — | Pending |
| ROL-01 | — | Pending |
| ROL-02 | — | Pending |
| ROL-03 | — | Pending |
| ROL-04 | — | Pending |

**Coverage:**
- v1 requirements: 16 total
- Mapped to phases: 0
- Unmapped: 16 ⚠️

---
*Requirements defined: 2026-04-04*
*Last updated: 2026-04-04 after initial definition*
