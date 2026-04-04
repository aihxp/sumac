# Summary 04-01: Migrate `sync` onto the Shared Core/App Seam and Prove Parity

## Outcome

`sync` is now a dedicated migrated write-side slice under `src/app/sync.rs`
instead of generic logic inside `src/app/golden_path.rs`.

## What Changed

- added `src/app/sync.rs`
- moved `SyncRequest`, sync execution, rendering, and `--check` exit behavior
  into `SyncService`
- kept `GoldenPathApp` as the stable dispatch seam while delegating sync to the
  dedicated service
- reduced `src/app/golden_path.rs` again as more command-specific logic moved
  out of the shared file

## Parity Proof Added

- direct integration coverage for legacy-route `sync`
- direct equality coverage for default/core versus legacy routing, with only
  the dynamic `last_synced_at` timestamp normalized

This makes `sync` the first write-oriented command family proven on the new
architecture.
