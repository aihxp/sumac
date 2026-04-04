# Roadmap: Sumac

## Overview

This milestone preserves the shipped `sxmc` product while rebuilding its
internals around a cleaner core/app seam. The roadmap starts by locking down
golden-path contracts and parity checks, then introduces the new seam, and
migrates `status`, `sync`, `add`, and `setup` in dependency order so releases
can continue without breaking CLI, JSON, artifact, or release behavior.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Contract Baseline & Parity Harness** - Lock down the golden
  path interface inventory and parity checks before migration.
- [x] **Phase 2: Core/App Seam & Cutover Foundation** - Introduce the new
  orchestration boundary and rollback-safe routing inside the existing binary.
- [x] **Phase 3: Status Migration** - Move `status` onto the new core as the
  first read-only proof slice.
- [x] **Phase 4: Sync Migration** - Move `sync` onto the new core to prove
  write planning and state mutation through the same seam.
- [ ] **Phase 5: Add Migration** - Move `add` onto the new core while
  preserving onboarding behavior and generated outputs.
- [ ] **Phase 6: Setup Consolidation & Golden Path Closeout** - Rebuild
  `setup` on shared services and close parity across the full golden path.

## Phase Details

### Phase 1: Contract Baseline & Parity Harness
**Goal**: The rewrite has a published golden-path contract baseline and parity
checks that prove behavior before migrated logic becomes the default path.
**Depends on**: Nothing (first phase)
**Requirements**: PAR-01, PAR-02, ROL-04
**Success Criteria** (what must be TRUE):
  1. Maintainers can consult one published interface inventory for `setup`,
     `add`, `status`, and `sync` covering commands, flags, aliases, env vars,
     exit codes, JSON fields, generated files, and MCP-facing behavior.
  2. Real CLI and artifact-level characterization coverage exists for the
     golden path and must pass before migrated logic becomes the default path.
  3. Rewrite validation reports track performance, reliability, and
     cross-platform parity for migrated golden-path slices instead of relying
     on spot checks.
**Plans**: 2 plans

Plans:
- [x] 01-01: Capture and publish the golden-path interface inventory
- [x] 01-02: Build characterization, artifact, and regression tracking for
  parity validation

### Phase 2: Core/App Seam & Cutover Foundation
**Goal**: Golden-path commands route through a new typed core/app seam with
explicit adapters, incremental cutover, and rollback-safe routing.
**Depends on**: Phase 1
**Requirements**: CORE-01, CORE-02, CORE-03, CORE-04, ROL-01, ROL-02, ROL-03
**Success Criteria** (what must be TRUE):
  1. The stable `sxmc` CLI can route migrated slices through a new internal
     core/app seam without changing command entrypoints or freezing releases.
  2. `src/main.rs` is reduced to CLI dispatch and runtime composition for the
     migrated golden path, with business orchestration moved behind the seam.
  3. Migrated slices exchange typed internal request/result contracts and reach
     filesystem, subprocess, transport, and env/path concerns through explicit
     adapters or ports.
  4. Each migrated slice has an explicit cutover path, rollback-safe routing,
     and defined shim retirement criteria before legacy logic is removed.
**Plans**: 3 plans

Plans:
- [x] 02-01: Create the core/app/runtime seam and trim golden-path logic out of
  `src/main.rs`
- [x] 02-02: Define typed requests, results, and adapter interfaces for the
  migrated command family
- [x] 02-03: Add cutover routing, rollback controls, and shim retirement rules

### Phase 3: Status Migration
**Goal**: `status` proves the new seam on a read-only production slice while
preserving its current behavior and output contracts.
**Depends on**: Phase 2
**Requirements**: PATH-01
**Success Criteria** (what must be TRUE):
  1. `sxmc status` runs through the new core/app path for golden-path
     scenarios.
  2. `sxmc status` preserves its current behavior and output contracts on parity
     fixtures.
**Plans**: 1 plan

Plans:
- [x] 03-01: Migrate `status` onto the shared core/app seam and prove parity

### Phase 4: Sync Migration
**Goal**: `sync` proves write planning, state mutation, and artifact
materialization through the same core/app seam.
**Depends on**: Phase 3
**Requirements**: PATH-02
**Success Criteria** (what must be TRUE):
  1. `sxmc sync` runs through the new core/app path for the maintained golden
     path.
  2. `sxmc sync` preserves current behavior, state mutation, and generated
     artifact behavior.
**Plans**: 1 plan

Plans:
- [x] 04-01: Migrate `sync` onto the shared core/app seam and prove parity

### Phase 5: Add Migration
**Goal**: `add` moves onto the new core/app path while keeping onboarding,
profile generation, host detection, and file outputs stable.
**Depends on**: Phase 4
**Requirements**: PATH-03
**Success Criteria** (what must be TRUE):
  1. `sxmc add` runs through the new core/app path for supported golden-path
     onboarding scenarios.
  2. `sxmc add` preserves onboarding behavior, profile generation, host
     detection, and generated file outputs relied on today.
**Plans**: 1 plan

Plans:
- [x] 05-01: Migrate `add` onto the shared core/app seam and prove parity

### Phase 6: Setup Consolidation & Golden Path Closeout
**Goal**: `setup` is rebuilt as orchestration over the shared migrated
services, completing parity for `setup`, `add`, `status`, and `sync`.
**Depends on**: Phase 5
**Requirements**: PATH-04, PAR-03, PAR-04
**Success Criteria** (what must be TRUE):
  1. `sxmc setup` runs through the new core/app path and reuses the same
     migrated onboarding and reconciliation services as the rest of the golden
     path.
  2. Across `setup`, `add`, `status`, and `sync`, stdout/stderr separation,
     exit behavior, and stable `1.x` JSON output contracts remain unchanged.
  3. Across local and global install scopes, generated host artifacts and state
     file behavior remain unchanged after the full golden path is migrated.
**Plans**: 2 plans

Plans:
- [ ] 06-01: Rebuild `setup` over shared onboarding and reconciliation services
- [ ] 06-02: Close full golden-path parity and retire temporary migration shims

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4 → 5 → 6

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Contract Baseline & Parity Harness | 2/2 | Complete   | 2026-04-04 |
| 2. Core/App Seam & Cutover Foundation | 3/3 | Complete   | 2026-04-04 |
| 3. Status Migration | 1/1 | Complete   | 2026-04-04 |
| 4. Sync Migration | 1/1 | Complete   | 2026-04-04 |
| 5. Add Migration | 1/1 | Complete   | 2026-04-04 |
| 6. Setup Consolidation & Golden Path Closeout | 0/2 | Not started | - |
