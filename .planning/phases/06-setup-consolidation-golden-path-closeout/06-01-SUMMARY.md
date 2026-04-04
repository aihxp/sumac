# Summary 06-01: Rebuild `setup` over Shared Onboarding and Reconciliation Services

## Outcome

`setup` is now a dedicated migrated onboarding slice under `src/app/setup.rs`,
which means all four maintained golden-path commands have their own service
modules and `src/app/golden_path.rs` is reduced to thin dispatch only.

## What Changed

- added `src/app/setup.rs`
- moved `SetupRequest` and setup execution into `SetupService`
- rewired `main.rs` and `GoldenPathApp` to use the dedicated setup service
- added explicit legacy-route coverage for `setup`
- added direct core-vs-legacy parity proof for the structured `setup` contract
- reduced `src/app/golden_path.rs` to a 35-line dispatch seam across
  `setup`, `add`, `status`, and `sync`

## Parity Proof Added

- direct integration coverage for legacy-route `setup`
- direct core-vs-legacy parity coverage comparing the stable structured setup
  contract
- full validation proof that setup still preserves multi-tool onboarding,
  preview fallback, host auto-detection, and global install-scope behavior

This completes the command-family migration pattern for the maintained golden
path and turns the new app layer into the canonical orchestration boundary for
that workflow.
