# Phase 2: Core/App Seam & Cutover Foundation - Validation

**Date:** 2026-04-04

## What Must Be Proven

This phase is successful only if the new app seam exists and the shipped golden
path still behaves the same from the outside.

## Validation Gates

### Architecture gates

- `src/main.rs` no longer owns the golden-path orchestration directly
- `setup`, `add`, `status`, and `sync` route through a new internal app layer
- typed request/result contracts exist for the migrated command family
- the route/cutover mechanism is explicit and rollback-safe

### Parity gates

- Phase 1 rewrite parity tests still pass unchanged
- generated artifacts and state paths remain unchanged for local/global scope
- structured JSON output contracts remain unchanged
- exit-code behavior remains unchanged for `status --exit-code` and
  `sync --check`

### Safety gates

- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `bash scripts/test-sxmc.sh --json <tempfile>`
- `git diff --check`

## Verification Focus

Review should confirm all of these:

1. `src/app/` exists and is the canonical entry point for golden-path
   orchestration.
2. `src/main.rs` is reduced to CLI dispatch/runtime composition for the
   migrated family.
3. Explicit routing or rollback control exists for the new seam.
4. The new seam has defined shim-retirement criteria recorded in the phase
   summary/verification.
