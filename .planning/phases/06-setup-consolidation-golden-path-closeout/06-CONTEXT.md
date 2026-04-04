# Phase 6: Setup Consolidation & Golden Path Closeout - Context

**Gathered:** 2026-04-04
**Status:** Ready for execution
**Mode:** Auto-generated during autonomous milestone execution

<domain>
## Phase Boundary

Finish the golden-path rewrite by moving `sxmc setup` onto the same dedicated
service pattern as `add`, `status`, and `sync`, then close parity across the
entire maintained onboarding path.

</domain>

<decisions>
## Implementation Decisions

### locked decisions
- Keep the public `sxmc setup` CLI surface unchanged.
- Reuse the shared onboarding service introduced in Phase 5.
- Preserve stable `1.x` JSON contracts, generated files, install-scope
  behavior, stdout/stderr behavior, and release cadence.
- Keep the global rollback route until the explicit release-soak retirement
  criteria from Phase 2 are met.

### the agent's Discretion
- How much shared setup orchestration belongs in a dedicated service versus the
  generic app seam.
- How to record shim retirement now that the command-family migrations are
  complete but the top-level rollback seam is still intentionally retained.

</decisions>

<code_context>
## Existing Code Insights

- `status`, `sync`, and `add` already run through dedicated services.
- `setup` still lives in `src/app/golden_path.rs`, although it already consumes
  the shared onboarding service from Phase 5.
- The remaining architectural cleanup is to make `GoldenPathApp` a thin
  dispatcher across all four migrated commands.

</code_context>

<specifics>
## Specific Ideas

- add `src/app/setup.rs`
- move `SetupRequest` and setup execution into `SetupService`
- add explicit legacy-route and core-vs-legacy parity tests for `setup`
- use this phase to mark the command-specific migration shims retired while
  documenting why the top-level rollback route remains for one release cycle

</specifics>

<deferred>
## Deferred Ideas

- removing `SXMC_GOLDEN_PATH_ROUTE=legacy` before the required stable release
  soak
- extending the rewrite pattern to adjacent flows outside the maintained golden
  path

</deferred>
