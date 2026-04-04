# Phase 6: Setup Consolidation & Golden Path Closeout - Validation

## Required Proof

1. `sxmc setup` is handled by a dedicated migrated app/service slice.
2. `GoldenPathApp` is reduced to thin dispatch across the full golden path.
3. Default/core and explicit legacy routing preserve setup contract parity.
4. Full validation stays green after the final golden-path extraction.
5. The milestone records command-shim retirement while keeping the top-level
   rollback seam under the existing release-soak rule.

## Commands

- `cargo test rewrite_golden_path_setup -- --nocapture`
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `bash scripts/test-sxmc.sh --json <tempfile>`
- `git diff --check`
