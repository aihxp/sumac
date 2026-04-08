# Phase 12 Soak Evidence Report

**Created:** 2026-04-04T23:34:31Z
**Milestone:** v1.1 Platform Hardening and Core Expansion
**Decision Scope:** migrated `watch` / `skills` route safety and status of the
existing `SXMC_GOLDEN_PATH_ROUTE=legacy` rollback seam

## Executive Summary

The repo contains strong local evidence that the migrated `watch`, `skills`,
and CLI-facing `serve` route is safe to operate as the sole route now. That
evidence comes from phase-level verification, real-fixture contract tests, and
full-suite validation across the completed v1.1 phases.

The repo does **not** contain true post-release soak evidence showing the older
golden-path legacy seam is no longer needed across shipped versions. Because of
that distinction, the correct Phase 12 decision is:

1. keep `watch` and `skills` on the migrated sole route with no new rollback
   seam introduced;
2. retain `SXMC_GOLDEN_PATH_ROUTE=legacy` intentionally for now, with explicit
   documentation that retirement requires a later release-soak review rather
   than same-session test success alone.

## Evidence Matrix

| Area | Evidence | What It Proves |
|------|----------|----------------|
| Watch runtime seam | [08-VERIFICATION.md](/Users/hprincivil/Projects/sxmc/.planning/phases/08-watch-runtime-hardening/08-VERIFICATION.md) | `sxmc watch` moved behind `src/app/watch.rs`, slow notify commands and slow webhooks do not stall unhealthy exits, and full validation remained green |
| Skill install lifecycle | [09-VERIFICATION.md](/Users/hprincivil/Projects/sxmc/.planning/phases/09-secure-skill-materialization-atomic-activation/09-VERIFICATION.md) | install/update materialization is correct, activation is atomic, and unsafe payloads are rejected before activation |
| Scan and serve policy | [10-VERIFICATION.md](/Users/hprincivil/Projects/sxmc/.planning/phases/10-unified-scan-serve-enforcement/10-VERIFICATION.md) | managed assets are scanned from the canonical inventory, unreadable/non-UTF-8 files become explicit findings, and unmanaged in-tree files are not served |
| Command-family extraction | [11-VERIFICATION.md](/Users/hprincivil/Projects/sxmc/.planning/phases/11-command-family-extraction-contract-gates/11-VERIFICATION.md) | `skills` and CLI-facing `serve` moved behind app services, and real-fixture contract tests stayed green |
| Golden-path rollback seam parity | `tests/cli_integration.rs` legacy/core parity tests for setup/add/status/sync | the older rollback seam still has parity coverage and remains isolated to the onboarding/golden-path commands |

## Key Passed Checks

### Migrated-route checks

- `cargo test --quiet watch_`
- `cargo test --quiet materialize_source_dir`
- `cargo test --quiet install_skill_`
- `cargo test --quiet skill_scanner`
- `cargo test --quiet handler`
- `cargo test --quiet skills_`
- `cargo test --quiet test_skills_install_and_serve_round_trip_via_stdio`
- `cargo test --quiet test_stdio_hybrid_`
- `cargo test --quiet test_serve_watch_reloads_skill_prompt_over_http`

### Broad validation checks

- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

### Older rollback-seam checks

- `test_rewrite_golden_path_setup_core_and_legacy_match`
- `test_rewrite_golden_path_add_core_and_legacy_match`
- `test_rewrite_golden_path_status_core_and_legacy_match`
- `test_rewrite_golden_path_sync_core_and_legacy_match`

## What This Evidence Proves

- The migrated `watch` route has explicit seam, timeout, and regression
  coverage.
- The migrated `skills` lifecycle and serve policy have explicit correctness
  and security coverage.
- The migrated `skills` plus `serve` command families have real-fixture
  contract coverage after extraction.
- No separate rollback seam is needed for `watch` or `skills` based on the
  current local evidence.

## What This Evidence Does Not Prove

- It does not prove multi-release soak under real downstream automation after
  shipping the Phase 11 extraction work.
- It does not prove the older `SXMC_GOLDEN_PATH_ROUTE=legacy` seam is now safe
  to retire solely on the basis of same-session verification.
- It does not replace a future post-release review of support issues, upgrade
  experience, and real usage telemetry or maintainer observation.

## Decision

### Migrated `watch` and `skills` route

Keep the migrated `watch` / `skills` / CLI-facing `serve` route as the sole
route. The repo now has explicit local evidence for reliability, security, and
contract stability, and no additional rollback seam should be introduced for
those families.

### Existing golden-path rollback seam

Retain `SXMC_GOLDEN_PATH_ROUTE=legacy` intentionally for now.

**Why retained:** the repo has strong local parity and contract evidence, but it
does not yet have a distinct post-release soak window after the v1.1 migration
work to justify final retirement.

**Retirement trigger:** revisit retirement after a later stable release soak in
which maintainers can point to successful real-world operation of the migrated
route without needing the legacy fallback.

## Maintainer Guidance

- Do not expand the legacy seam to `watch`, `skills`, or `serve`.
- Treat the current retained seam as a bounded compatibility valve for the
  older golden-path commands only.
- Use this report plus the phase verification artifacts as the reference when
  the next release-soak review happens.
