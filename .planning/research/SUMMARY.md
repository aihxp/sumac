# Rewrite Research Summary

## Executive Summary

Sumac is already a broad, shipped `1.x` Rust CLI/MCP product. The rewrite should preserve that surface and rebuild the internals around a cleaner orchestration boundary, not narrow the product or restart it as a parallel codebase. The research consistently recommends a modular monolith inside the existing crate, with a thin stable CLI shell, a new app layer for command-family orchestration, a typed core layer for policies and reconciliation rules, and infrastructure adapters around the current transport, filesystem, process, HTTP, and MCP modules.

The strongest recommendation is to treat this as a parity-first migration program. Keep the existing Rust stack, add only the minimum tooling required to make cutover safe, and move one stable command family at a time behind an explicit seam. The golden onboarding and reconciliation path, `setup -> add -> status -> sync`, should drive the first milestone because it is the maintained user workflow and it exercises the exact boundary problems the rewrite is meant to fix.

The major risk is not framework choice. The risk is accidental contract drift while internal structure changes. Requirements and roadmap phases should therefore center on interface inventory, characterization tests, presenter-owned contracts, old/new compare capability, and rollback-safe routing. If the team preserves those controls, the rewrite can land incrementally without freezing releases or destabilizing the product.

## Recommended Stack Boundary

- Keep Rust as the only implementation language and pin the stable toolchain with `rust-toolchain.toml`.
- Keep the current transport and runtime stack: `clap`, `tokio`, `serde`, `reqwest`, `axum`, `tower`, `rmcp`, and `thiserror`.
- Add only migration safety tooling: `tracing`, `tracing-subscriber`, `insta`, and `trycmd`.
- Keep the first rewrite inside the existing crate. Do not start with a workspace split.
- Treat `clap` types, human output, JSON rendering, exit codes, and generated files as edge concerns owned by the CLI/presenter boundary.
- Move internal orchestration away from `serde_json::Value` and into typed request/result contracts.
- Keep adapter-heavy concerns out of the new core: filesystem traversal, subprocesses, HTTP, MCP transport, env lookup, and path mutation stay behind ports.

## Rewrite Table Stakes

- Publish and maintain an interface inventory covering commands, flags, aliases, env vars, exit codes, JSON surfaces, generated files, and MCP-facing behavior.
- Build characterization and golden-contract coverage before moving logic. Stable behavior must be proven, not inferred.
- Preserve command and artifact parity for the golden path, including stdout/stderr separation and file materialization.
- Support incremental cutover so old and new implementations can ship together behind the same stable command surface.
- Keep releases moving throughout the rewrite. Mixed old/new internals are expected.
- Define deprecation and shim retirement rules up front so temporary compatibility paths do not become permanent architecture.
- Track regression budgets for performance, reliability, and cross-platform behavior, not just correctness.

## Architectural Migration Pattern

The recommended seam is:

`stable CLI contract -> command adapter -> app use case -> core policy + ports -> infra adapter or legacy shim -> stable presenter`

This pattern matters more than any specific module naming. It gives Sumac a way to replace internals without changing the shipped surface.

Recommended structure inside the current crate:

- `cli`: command entrypoints, arg-to-request mapping, presenter selection, exit codes
- `app`: command-family orchestration for `setup`, `add`, `status`, `sync`, then adjacent flows
- `core`: typed models, diff/reconciliation logic, write plans, and narrow ports
- `infra`: adapters over current modules and external systems
- `legacy`: normalized wrappers over extracted old behavior for staged cutover
- `runtime`: dependency wiring and task ownership

Recommended migration order:

1. Establish the seam and reduce `main.rs` to composition and dispatch.
2. Move presenter ownership for stable outputs so contracts have one explicit owner.
3. Migrate `status` first as the read-only proof that the new boundary can preserve behavior.
4. Migrate `sync` next to prove safe write planning and state mutation through the same core.
5. Move `add` onto the new path after the shared reconciliation and artifact-planning services exist.
6. Rebuild `setup` as orchestration over the same `add` and reconciliation services rather than as a separate implementation.
7. Rehome `doctor` and `watch` onto the same family once the shared model is stable.

## Major Pitfalls

- Parallel-universe rewrite: if the new implementation becomes a second product tree, releases and bug fixes will drift away from it.
- No characterization baseline: preserving intent without snapshots for JSON, exit codes, stderr, and generated files will break scripts silently.
- Parser-only parity: keeping command names while changing defaults, aliases, file edits, or output semantics is still a breaking change.
- Cleanup disguised as migration: “fixing” legacy quirks during the rewrite will create avoidable trust breaks unless each change is explicitly classified.
- Orchestration plus state-format churn: changing control flow and persistent artifacts at the same time makes regressions ambiguous and hard to unwind.
- Happy-path-only validation: the rewrite must preserve degraded behavior, recovery guidance, cancellation, and partial-failure semantics, not just the demo path.
- Cutover without compare and rollback: every migrated family needs side-by-side diffing and a simple retreat path before legacy code is retired.

## Roadmap Implications

- Phase 1 should focus on contract capture and parity harnesses, not new architecture for its own sake.
- Phase 2 should establish the internal seam and presenter boundary while keeping behavior unchanged.
- Phase 3 should migrate `status` as the first production slice because it is the cheapest high-signal proof.
- Phase 4 should migrate `sync` immediately after `status` so the same core owns both read and write flows.
- Phase 5 should move `add` and then `setup`, using shared onboarding and reconciliation services instead of duplicating logic.
- Later phases should extend the same pattern to `doctor`, `watch`, skills flows, and transport-oriented surfaces.
- Any phase touching packaging, watch behavior, state files, or transport boundaries should carry extra research or validation gates because those are high-regression areas.

## Requirements Guidance

- Write requirements in terms of preserved contracts, cutover mechanics, and retirement criteria, not abstract architecture goals.
- Treat presenter-owned JSON and generated-file invariants as first-class acceptance criteria.
- Require old/new compare capability and rollback-safe routing before any command family is considered migrated.
- Keep scope discipline: no CLI redesign, no product narrowing, no release freeze, and no broad dependency churn inside the rewrite milestone.

## Sources

- `.planning/PROJECT.md`
- `.planning/research/STACK.md`
- `.planning/research/FEATURES.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/PITFALLS.md`
