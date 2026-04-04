# Summary 03-01: Migrate `status` onto the Shared Core/App Seam and Prove Parity

## Outcome

`status` is now a dedicated migrated read-only slice under `src/app/status.rs`
instead of being implemented as another generic method in
`src/app/golden_path.rs`.

## What Changed

- added `src/app/status.rs`
- moved `StatusRequest` and status execution into `StatusService`
- kept `GoldenPathApp` as the generic dispatch seam while delegating status to
  the dedicated service
- reduced `src/app/golden_path.rs` from 717 lines to 626 lines

## Parity Proof Added

- direct integration coverage for legacy-route `status`
- direct equality coverage comparing default/core and explicit legacy routing

This makes `status` the first command family to prove that the new seam can
host a focused migrated slice without changing the shipped surface.
