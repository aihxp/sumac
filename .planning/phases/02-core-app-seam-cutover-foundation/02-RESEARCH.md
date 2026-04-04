# Phase 2: Core/App Seam & Cutover Foundation - Research

**Date:** 2026-04-04
**Scope:** Build the first greenfield internal seam for the golden path without
changing the public `sxmc` CLI, JSON contracts, generated files, or release
cadence.

## What the Codebase Looks Like Today

The golden path still lives directly inside `src/main.rs`.

- `Commands::Add` starts around line 10278
- `Commands::Setup` starts around line 10372
- `Commands::Doctor` starts around line 10898
- `Commands::Status` starts around line 10969
- `Commands::Sync` starts around line 10997

`src/main.rs` is currently over 11k lines and still mixes:

- clap command dispatch
- install-scope resolution
- AI host auto-detection
- profile inspection and readiness gating
- artifact preview/write orchestration
- status/sync recovery reporting
- rollback-sensitive state mutation

The good news is the codebase already has reusable building blocks:

- `src/paths.rs` already isolates install-scope and state-path behavior
- `src/command_handlers.rs` already shows a command-family extraction pattern
- `src/cli_surfaces/*` already holds most artifact generation/materialization
  behavior
- Phase 1 created a contract and parity harness for `setup`, `add`, `status`,
  and `sync`

## Architecture Judgment

For this phase, the safest greenfield move is not a big-bang library rewrite.
It is a binary-internal `app` seam that:

- introduces typed request/result contracts for the golden path
- reduces `src/main.rs` to stable CLI dispatch plus runtime composition
- routes commands through a new internal app layer
- keeps rollback explicit with a route switch
- allows later phases to migrate behavior slice by slice behind the same seam

This gives us a real new architecture now, without forcing unrelated public
surface churn.

## Proposed Shape

### New internal module family

Create a new binary-internal layer under `src/app/`:

- `src/app/mod.rs`
- `src/app/golden_path.rs`

This layer becomes the canonical orchestration boundary for:

- `setup`
- `add`
- `status`
- `sync`

### Typed contracts

Define request/result structs per command family instead of keeping the command
arms as ad hoc local orchestration.

Examples:

- `AddRequest` / `AddOutcome`
- `SetupRequest` / `SetupOutcome`
- `StatusRequest` / `StatusOutcome`
- `SyncRequest` / `SyncOutcome`

The result structs should preserve the data needed for current rendering, not
replace stable output contracts yet.

### Explicit adapters

The seam still needs access to legacy functions while we migrate behavior. The
best bridge for this phase is explicit adapters around current behavior, not a
full port/trait explosion.

Examples:

- install path adapter
- host detection adapter
- profile inspection/readiness adapter
- artifact resolution/materialization adapter
- status adapter
- sync adapter

That keeps infrastructure concerns visible and later phases can replace each
adapter with more native services incrementally.

### Rollback-safe routing

The migrated command family needs an explicit cutover mechanism.

Recommended approach:

- add a hidden env-driven route selector for the golden path
- default route uses the new core/app layer
- explicit legacy route remains available during the migration

This gives us:

- safe rollback if a stable release regresses
- mixed old/new internals during the rewrite
- clear shim-retirement criteria later

## Risks

### Risk: fake architecture

If the app layer only forwards one function call per command, the seam is not
real. We must move orchestration boundaries, not just rename helpers.

### Risk: permanent dual paths

If the rollback path has no retirement rule, the rewrite will calcify into two
implementations. The phase must define shim retirement criteria.

### Risk: contract drift

If the new seam changes stdout/stderr timing, JSON shape, or artifact writes,
later phases lose the baseline. Phase 1 parity tests must gate this phase.

## Recommendation

Implement a real `src/app/golden_path.rs` seam now, route the golden path
through it, keep rendering/output stable, and treat the current `main.rs`
helpers as legacy adapters until later phases migrate each command family in
depth.
