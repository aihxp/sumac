# Phase 4: Sync Migration - Validation

## Required Proof

1. `sxmc sync` is handled by a dedicated migrated app/service slice.
2. Default/core and explicit legacy routing preserve sync contract parity.
3. `--check` exit behavior remains unchanged.
4. Full regression validation remains green.

## Commands

- `cargo test --quiet test_rewrite_golden_path_sync_`
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `bash scripts/test-sxmc.sh --json <tempfile>`
