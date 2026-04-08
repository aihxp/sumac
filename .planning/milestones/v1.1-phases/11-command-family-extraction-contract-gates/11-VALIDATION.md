# Phase 11: Command-Family Extraction & Contract Gates - Validation Strategy

**Created:** 2026-04-04
**Phase:** 11-command-family-extraction-contract-gates

## Purpose

Define how Phase 11 proves that `skills` and CLI-facing `serve` orchestration
move behind clearer service boundaries without breaking the shipped command
contracts.

## Evidence Required

### 1. Service Boundary Extraction

The phase must prove that:
- `src/main.rs` delegates `skills` orchestration to `app::skills`
- `src/main.rs` delegates CLI-facing `serve` orchestration to `app::serve`
- extracted services reuse existing lower-level behavior instead of changing it

### 2. Real-Fixture Contract Gates

The phase must prove that migrated flows still behave the same on real fixtures:
- `skills list` and `skills info` still render expected output
- `skills install` and `skills update` still produce the same install side
  effects and metadata
- stdio hybrid `serve` still exposes skills through the expected MCP surface
- migrated serve/watch behavior still passes its existing real-fixture reload
  regression

## Required Checks

- focused CLI integration tests for `skills` list/info/install/update
- focused CLI integration tests for stdio hybrid serve and serve-watch reload
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

## Pass Conditions

- `skills` and `serve` command families no longer rely on large inline
  orchestration branches in `main.rs`
- current user-visible CLI behavior stays stable across migrated flows
- the extraction is backed by explicit contract-gate coverage on real fixtures

## Risks To Watch

- moving the branches but subtly changing output or exit behavior
- duplicating lower-level logic inside the app layer instead of orchestrating
  existing helpers
- relying on old tests without pinning the newly extracted contract surfaces
