# Phase 3: Status Migration - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Auto-generated (infrastructure-first autonomous kickoff)

<domain>
## Phase Boundary

Take `sxmc status` from “routed through the new seam” to a real migrated
read-only slice with its own dedicated app/service boundary while preserving
all current behavior and output contracts.

</domain>

<decisions>
## Implementation Decisions

### locked decisions
- Keep the public `sxmc status` surface unchanged.
- Preserve structured JSON, human rendering, and `--exit-code` behavior.
- Treat `status` as the first proof slice for the new internal architecture.

### the agent's Discretion
- Exact module layout for the status service.
- Whether parity proof is expressed through snapshot comparison, targeted
  assertions, or both.

</decisions>

<code_context>
## Existing Code Insights

- Phase 2 already routes `status` through `GoldenPathApp`.
- `status` still shares generic golden-path plumbing with other commands.
- `status_value_with_health` and status rendering are strong candidates for a
  dedicated read-only service boundary.

</code_context>

<specifics>
## Specific Ideas

- Extract a dedicated `status` service/module under `src/app/`.
- Add direct parity proof between the default core route and the legacy route.

</specifics>

<deferred>
## Deferred Ideas

- Broader migration of `sync`, `add`, and `setup` (belongs to later phases)

</deferred>
