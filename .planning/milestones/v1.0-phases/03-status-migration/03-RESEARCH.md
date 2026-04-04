# Phase 3: Status Migration - Research

## What Phase 2 Already Delivered

Phase 2 created the shared seam:

- `GoldenPathApp`
- typed golden-path requests/results
- `SXMC_GOLDEN_PATH_ROUTE=core|legacy`

`status` already routes through that seam, but it is still not a distinct
migrated service. It shares generic routing code in `src/app/golden_path.rs`
and still depends on status-specific logic living outside a dedicated slice.

## Best Next Step

The most valuable Phase 3 move is to give `status` its own app/service module
and make the generic golden-path seam delegate to that module.

That proves:

- the seam can host a real per-command service
- a read-only slice can migrate first with low operational risk
- parity can be tested directly between core and legacy routing

## Migration Shape

Recommended implementation:

- add `src/app/status.rs`
- move `StatusRequest`, `StatusService`, and status rendering there
- keep legacy value-building helpers as adapters for now
- add direct parity tests comparing:
  - core/default route
  - explicit legacy route

This keeps the slice meaningful without prematurely migrating write behavior.
