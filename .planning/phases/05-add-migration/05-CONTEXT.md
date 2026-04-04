# Phase 5: Add Migration - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Auto-generated (infrastructure-first autonomous kickoff)

<domain>
## Phase Boundary

Migrate `sxmc add` into a dedicated onboarding slice while preserving profile
inspection, host detection, preview/apply behavior, structured JSON, and
generated file outputs.

</domain>

<decisions>
## Implementation Decisions

### locked decisions
- Keep the public `sxmc add` CLI surface unchanged.
- Preserve host auto-detection, preview fallback, and generated artifact
  behavior.
- Use this phase to create the reusable onboarding service that Phase 6 can
  reuse for `setup`.

### the agent's Discretion
- Exact split between `add` and shared onboarding helpers.
- How much of the current generic onboarding logic should move before Phase 6.

</decisions>

<code_context>
## Existing Code Insights

- `add` currently uses the shared onboarding path in `src/app/golden_path.rs`.
- the host selection and inspect/materialize logic is now ready to be promoted
  into a reusable service
- `setup` still depends on the same shared onboarding behavior

</code_context>

<specifics>
## Specific Ideas

- add `src/app/onboarding.rs`
- add `src/app/add.rs`
- make `add` the first onboarding-specific slice
- keep `setup` consuming the shared onboarding service until Phase 6

</specifics>

<deferred>
## Deferred Ideas

- final `setup` rebuild and legacy-route retirement

</deferred>
