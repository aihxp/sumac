# Phase 5: Add Migration - Validation

## Required Proof

1. `sxmc add` is handled by a dedicated migrated app/service slice.
2. Shared onboarding logic exists outside `src/app/golden_path.rs`.
3. Default/core and explicit legacy routing preserve add contract parity.
4. Full regression validation remains green.

## Commands

- `cargo test --quiet test_rewrite_golden_path_add_`
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `bash scripts/test-sxmc.sh --json <tempfile>`
