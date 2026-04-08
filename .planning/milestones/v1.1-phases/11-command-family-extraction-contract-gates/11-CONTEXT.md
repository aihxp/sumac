# Phase 11: Command-Family Extraction & Contract Gates - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Autonomous discuss (`--auto`)

<domain>
## Phase Boundary

Move the remaining `skills` and CLI-facing `serve` orchestration out of the
large `src/main.rs` dispatch block and behind focused app-service seams without
changing the shipped `1.x` command contracts. This phase is about command
boundary cleanup and contract proof, not new product behavior: the extracted
services should reuse the already-hardened install, scan, serve, and watch
internals from earlier phases while preserving flags, output, exit semantics,
and file side effects.

</domain>

<decisions>
## Implementation Decisions

### Service boundaries
- **D-01:** `src/main.rs` should become a thin request-assembly layer for the
  `skills` family, mirroring the existing `app::watch` and golden-path
  services.
- **D-02:** The top-level `serve` command should get a focused CLI wrapper seam
  in `src/app/serve.rs` so registration prep and transport dispatch are no
  longer inline in `main.rs`.
- **D-03:** Existing lower-level helpers in `src/command_handlers.rs`,
  `src/skills/install.rs`, `src/server/*`, and `generator` should be reused
  rather than rewritten.

### Contract gates
- **D-04:** Real-fixture CLI integration tests are the proof mechanism for this
  phase; they should cover the migrated `skills` and `serve` flows that users
  actually depend on.
- **D-05:** The extraction should preserve current stdout/stderr, exit codes,
  JSON output shapes, install scope behavior, and side effects on disk.
- **D-06:** Watch contract coverage from Phase 8 remains part of the safety
  story, but Phase 11 should add or tighten tests around the newly extracted
  `skills` and `serve` command families.

### Phase boundaries
- **D-07:** Do not add a new rollback route or framework. This is seam cleanup
  inside the existing repo structure.
- **D-08:** Soak evidence and rollback-retirement decisions stay deferred to
  Phase 12.

### the agent's Discretion
- Exact request/result types for the `skills` and `serve` app services.
- Whether to move small utility helpers into `src/app/skills.rs` / `serve.rs`
  or keep them in `main.rs` if they remain shared and stable.
- Which integration tests are sufficient as the explicit contract-gate set, as
  long as they exercise real fixture flows for `skills` and `serve`.

</decisions>

<specifics>
## Specific Ideas

- Introduce `app::skills::SkillsService` with typed request enums for list,
  info, run, create, install, and update.
- Introduce `app::serve::ServeService` for registration setup, auth parsing,
  and transport dispatch while keeping `server::serve_stdio/http` as the
  library boundary.
- Promote the existing `skills install/update` and stdio/http serve tests into
  an explicit contract-gate suite and add missing real-fixture checks where the
  current test set is still thin.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone and phase contract
- `.planning/ROADMAP.md` — Phase 11 goal, dependency order, and success
  criteria.
- `.planning/REQUIREMENTS.md` — `CORE-06`, `CORE-07`, and `ROL-05` define the
  required outcomes for this phase.
- `.planning/PROJECT.md` — milestone constraints and the product-preserving
  rewrite strategy.
- `.planning/STATE.md` — current milestone position after Phase 10.

### Milestone research
- `.planning/research/ARCHITECTURE.md` — recommended `app::skills` and optional
  `app::serve` seams.
- `.planning/research/STACK.md` — reinforces that no new architecture layer is
  needed for this extraction.
- `.planning/codebase/CONCERNS.md` — documents `src/main.rs` as the remaining
  orchestration hotspot and calls for command-family splitting.

### Current implementation touchpoints
- `src/main.rs` — current inline `serve` and `skills` dispatch branches plus
  shared parsing helpers.
- `src/app/mod.rs` and `src/app/watch.rs` — existing app-service pattern to
  mirror.
- `src/command_handlers.rs` — current `skills` list/info/run helper functions.
- `src/skills/install.rs` — hardened install/update lifecycle that the app
  service should call, not reimplement.
- `tests/cli_integration.rs` — existing contract coverage for `skills`,
  `serve`, stdio hybrid behavior, and watch reload flows.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `app::watch::WatchService` already demonstrates the milestone's preferred
  request/service seam inside `src/app/`.
- `command_handlers.rs` already contains stable `skills` list/info/run logic,
  so extraction can primarily reorganize ownership rather than rebuild
  behavior.
- Earlier phases already hardened install/update (`src/skills/install.rs`) and
  serve policy (`src/server/handler.rs`), which means Phase 11 can focus on
  orchestration seams and tests.

### Established Patterns
- The v1.0 golden-path services in `src/app/*` use typed request structs and
  return `CommandOutcome`, which Phase 11 should mirror for the remaining
  command families.
- Existing CLI integration tests already assert real outputs for `skills
  install`, `skills update`, stdio hybrid MCP operations, and `serve --watch`
  reload behavior; these are the right contract anchors.

### Integration Points
- `Commands::Skills` still occupies a large inline match arm in `src/main.rs`
  that owns path resolution, install scope resolution, printing, and async
  dispatch.
- `Commands::Serve` still mixes registration setup, auth parsing, and transport
  dispatch directly in `main.rs`.
- `tests/cli_integration.rs` already has end-to-end coverage for `skills`
  install/update and several `serve` flows, but that coverage is not yet
  clearly organized as the extraction gate for this phase.

</code_context>

<deferred>
## Deferred Ideas

- Soak-based cutover review and rollback decisions — Phase 12.
- Larger `main.rs` decomposition for command families beyond `skills` and
  `serve`.
- Any public CLI redesign or contract changes.

</deferred>

---

*Phase: 11-command-family-extraction-contract-gates*
*Context gathered: 2026-04-04*
