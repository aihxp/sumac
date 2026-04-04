# Phase 2: Core/App Seam & Cutover Foundation - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Auto-generated (infrastructure-first autonomous kickoff)

<domain>
## Phase Boundary

Introduce a new internal core/app seam for the golden path so `sxmc` can route
future migrated slices through cleaner orchestration boundaries without changing
the public CLI.

This phase is about architecture scaffolding and cutover safety, not about
changing the user-facing behavior of `setup`, `add`, `status`, or `sync`.

</domain>

<decisions>
## Implementation Decisions

### locked decisions
- Keep the public CLI surface, stable JSON outputs, generated files, and release
  cadence unchanged while building the new seam.
- Reduce `src/main.rs` to CLI dispatch and runtime composition for migrated
  slices.
- Build the seam inside the existing repo and crate structure rather than as a
  separate parallel product tree.
- Make cutover and rollback explicit so mixed old/new internals can ship safely
  during the rewrite.

### the agent's Discretion
- Exact module layout for the new core/app/runtime layers.
- Whether the first seam route uses thin adapters, shims, or composition
  wrappers, as long as later phases can migrate command families slice by slice.
- Naming of internal request/result types and adapter traits.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### rewrite baseline
- `.planning/PROJECT.md` — rewrite intent and non-negotiables
- `.planning/REQUIREMENTS.md` — CORE-* and ROL-* requirements
- `.planning/ROADMAP.md` — Phase 2 goal and plan split
- `.planning/STATE.md` — current project state
- `.planning/phases/01-contract-baseline-parity-harness/01-VERIFICATION.md` —
  proven baseline from Phase 1

### contract and validation
- `docs/GOLDEN_PATH_CONTRACT.md` — stable golden-path contract baseline
- `docs/STABILITY.md` — `1.x` compatibility rules
- `docs/VALIDATION.md` — rewrite parity proof path

### likely implementation touchpoints
- `src/main.rs` — current orchestration hotspot
- `src/cli_args.rs` — stable CLI parsing surface
- `src/command_handlers.rs` — existing command-family helpers
- `src/paths.rs` — install scope and state-path logic already extracted

</canonical_refs>

<code_context>
## Existing Code Insights

- `src/main.rs` still contains the golden-path command routing and a large
  amount of orchestration logic.
- `src/command_handlers.rs`, `src/paths.rs`, and `src/cli_surfaces/*` already
  hold reusable pieces that can become adapters or ports instead of being
  reimplemented.
- Phase 1 now provides the contract and parity harness needed to refactor
  internally without guessing about behavior.

</code_context>

<specifics>
## Specific Ideas

- Introduce a small internal app layer for the golden path first instead of a
  repo-wide architectural reset.
- Prefer typed requests/results and explicit dependencies passed through runtime
  composition over more helper sprawl inside `main.rs`.
- Make the cutover path visible, ideally with routing hooks or migration shims
  that can be retired later.

</specifics>

<deferred>
## Deferred Ideas

- Public CLI redesign
- Migration of `status`, `sync`, `add`, and `setup` behavior themselves
  (belongs to later phases after the seam exists)
- Broader migration of discovery, serve, wrap, or skills families

</deferred>
