# Phase 4: Sync Migration - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Auto-generated (infrastructure-first autonomous kickoff)

<domain>
## Phase Boundary

Migrate `sxmc sync` into its own dedicated app/service slice so write planning,
profile refresh, artifact materialization, and `--check` behavior are no longer
just generic golden-path plumbing.

</domain>

<decisions>
## Implementation Decisions

### locked decisions
- Keep the public `sxmc sync` CLI surface unchanged.
- Preserve structured JSON, text rendering, state-file behavior, and
  `--check` exit semantics.
- Reuse the Phase 3 proof pattern: dedicated service plus direct parity between
  the default/core route and the legacy route.

### the agent's Discretion
- Exact `sync` module layout
- Whether parity proof is full-value comparison or focused contract comparison

</decisions>

<code_context>
## Existing Code Insights

- `sync` already routes through `GoldenPathApp`.
- `sync_saved_profiles_value` remains the key adapter-backed value builder.
- `sync` is the first write-oriented command slice after the read-only status
  migration proved the seam.

</code_context>

<specifics>
## Specific Ideas

- add `src/app/sync.rs`
- move `SyncRequest`, execution, and rendering there
- add explicit default/core versus legacy parity checks for `sync`

</specifics>

<deferred>
## Deferred Ideas

- onboarding-specific migration work for `add` and `setup`

</deferred>
