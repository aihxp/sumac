# Summary 02-03: Add Cutover Routing and Shim Retirement Rules

## Outcome

Added explicit cutover routing for the golden path with a rollback-safe legacy
mode.

## Route Added

- env var: `SXMC_GOLDEN_PATH_ROUTE`
- supported values:
  - `core` (default)
  - `legacy`

The default shipped path now runs through the new app seam. The legacy path
remains available as a rollback switch during migration.

## Coverage Added

- rewrite contract tests still pass on the default routed path
- added integration coverage for the legacy route on:
  - `add`
  - `status`

## Shim Retirement Criteria

The legacy route and legacy execution branches can be removed only when all of
these are true:

1. Phases 3-6 are complete and the golden path is fully migrated behind shared
   services.
2. Phase 1 parity tests pass for the core route across local and global scope.
3. No maintained command in the golden path still depends on a legacy-only
   execution branch.
4. One stable release cycle has shipped without needing the legacy route as a
   rollback escape hatch.
