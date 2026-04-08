# Phase 8: Watch Runtime Hardening - Validation Strategy

**Created:** 2026-04-04
**Phase:** 08-watch-runtime-hardening

## Purpose

Define how Phase 8 proves that `sxmc watch` remains responsive while slow
notify commands and webhooks are isolated behind a dedicated runtime seam.

## Evidence Required

### 1. Watch Runtime Seam

The phase must move CLI watch orchestration behind a dedicated module or
service boundary in `src/app/`, with `main.rs` reduced to typed request
assembly and dispatch.

### 2. Bounded Side Effects

The phase must prove that:
- slow or hanging notify commands cannot stall the watch loop indefinitely
- webhook delivery has explicit timeout or failure handling
- one notification failure does not abort the main watch loop unexpectedly

### 3. No Contract Drift

Validation should confirm that existing watch CLI flags, JSON or human output,
exit-on-change behavior, exit-on-unhealthy behavior, and current notification
payload shapes remain intact.

## Required Checks

- focused watch integration tests for notify file, webhook, unhealthy exit, and
  first-frame output
- new regression tests covering slow notify commands and slow or failing
  webhooks
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Pass Conditions

- `src/app/watch.rs` owns the long-running watch runtime or equivalent seam
- blocking sleep and child-process handling are removed from the main watch
  loop
- slow notification side effects are bounded and explicitly reported
- existing watch CLI behavior remains stable

## Risks To Watch

- extraction lands but `main.rs` still owns most runtime logic
- timeout behavior exists but still serializes later watch frames behind one
  slow endpoint
- contract tests pass while slow side-effect handling remains under-tested
