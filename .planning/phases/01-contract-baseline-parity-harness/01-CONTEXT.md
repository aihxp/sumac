# Phase 1: Contract Baseline & Parity Harness - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Auto-generated (infrastructure-first autonomous kickoff)

<domain>
## Phase Boundary

Lock down the golden-path contract baseline and parity validation for the
rewrite target before any migrated logic becomes the default path.

This phase covers the current shipped behavior for:
- `sxmc setup`
- `sxmc add`
- `sxmc status`
- `sxmc sync`

The output of this phase should give maintainers one stable interface inventory,
repeatable parity checks, and rewrite-focused validation reporting that can be
reused by the later migration phases.

</domain>

<decisions>
## Implementation Decisions

### locked decisions
- Preserve the existing `1.x` CLI behavior and JSON contracts while documenting
  the golden path.
- Treat `setup`, `add`, `status`, and `sync` as the canonical rewrite target
  for the first milestone.
- Capture parity at the command, artifact, and validation-report levels rather
  than relying on informal spot checks.
- Keep this phase focused on baseline publication and proof harnesses, not on
  moving runtime business logic yet.

### the agent's Discretion
- Exact file placement for the published contract inventory and rewrite-focused
  validation artifacts.
- Whether parity coverage lives primarily in Rust integration tests, shell
  harnesses, or a split model, as long as maintainers get deterministic,
  repeatable proof.
- How to summarize golden-path behavior most usefully for later migration
  phases, provided the inventory stays concise and versionable.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### project intent
- `.planning/PROJECT.md` — rewrite scope, non-negotiables, and compatibility
  constraints
- `.planning/REQUIREMENTS.md` — exact requirement IDs for this phase
- `.planning/ROADMAP.md` — phase goal, success criteria, and plan breakdown
- `.planning/STATE.md` — current milestone status and rewrite sequencing

### codebase guidance
- `.planning/codebase/CONVENTIONS.md` — code and output style expectations
- `.planning/codebase/TESTING.md` — current validation and regression patterns
- `docs/PRODUCT_CONTRACT.md` — current stable contract framing for public
  surfaces
- `docs/STABILITY.md` — `1.x` compatibility commitments
- `docs/VALIDATION.md` — current release validation story

### likely implementation touchpoints
- `src/main.rs` — current golden-path orchestration hotspot
- `tests/cli_integration.rs` — broad binary-level characterization suite
- `scripts/test-sxmc.sh` — expansive shell-based regression suite

</canonical_refs>

<code_context>
## Existing Code Insights

- The current codebase already enforces stable machine-readable stdout patterns
  and regression checks in `tests/cli_integration.rs`.
- Validation is layered across Rust tests, portable smoke scripts, and
  `scripts/test-sxmc.sh`; phase output should reuse that structure rather than
  inventing a separate verification stack.
- `src/main.rs` currently owns too much orchestration, but this phase should
  baseline behavior before Phase 2 starts carving out the new core/app seam.

</code_context>

<specifics>
## Specific Ideas

- Publish a single golden-path interface inventory that later phases can cite as
  the source of truth during migration.
- Add explicit rewrite parity coverage for command outputs, generated artifacts,
  and validation reporting so cutover can prove no regression.
- Keep cross-platform validation visible because this rewrite is meant to ship
  incrementally, not as a hidden branch.

</specifics>

<deferred>
## Deferred Ideas

- Any new public command design or broader product reshaping.
- Core/app seam implementation work (belongs to Phase 2).
- Migration of adjacent families like `doctor`, `watch`, `discover`, or
  `skills` (belongs to later milestones or phases).

</deferred>
