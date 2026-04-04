# Phase 3: Status Migration - Validation

## Required Proof

1. `sxmc status` is handled by a dedicated migrated app/service slice.
2. Default/core and explicit legacy routing produce parity on golden-path
   fixtures.
3. Human rendering, structured JSON output, and `--exit-code` behavior remain
   unchanged.

## Commands

- `cargo test --quiet test_rewrite_golden_path_status_`
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `bash scripts/test-sxmc.sh --json <tempfile>`

## Success Condition

If the default/core route and explicit legacy route agree on the golden-path
status contract while the full suite stays green, the phase is complete.
