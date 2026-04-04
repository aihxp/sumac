# Phase 8: Watch Runtime Hardening - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Autonomous discuss (`--auto`)

<domain>
## Phase Boundary

Keep long-running `sxmc watch` sessions responsive even when notification side
effects are slow or failing, and move the command’s orchestration behind a
dedicated runtime seam. This phase covers the CLI `watch` loop, its notify
command and webhook side effects, and its module boundary in `src/app/`. It
does not yet expand into skill install staging, serve-policy enforcement, or
the broader `skills` command-family extraction.

</domain>

<decisions>
## Implementation Decisions

### Watch runtime seam
- **D-01:** `sxmc watch` should move out of the top-level `Commands::Watch`
  branch in `src/main.rs` and into a dedicated app/runtime module that matches
  the established `src/app/status.rs` and `src/app/setup.rs` pattern.
- **D-02:** CLI flags, structured output, human output, exit semantics, and
  wrapper entrypoints must remain unchanged while the seam is introduced.
- **D-03:** Request assembly may stay in `main.rs`, but the long-running watch
  loop, frame diffing, notification fanout, and exit policy should live behind
  the new module boundary.

### Bounded side effects
- **D-04:** Slow or failed notify commands and webhooks must not stall the main
  watch loop or prevent later frames from rendering.
- **D-05:** Notification failures should surface as explicit stderr messages or
  structured side-effect results, not as silent drops.
- **D-06:** Timeouts should be enforced with the existing Tokio stack instead
  of introducing a new runtime or worker framework.

### Phase boundaries
- **D-07:** Keep this phase focused on CLI watch runtime behavior and the app
  seam. Server-side `serve --watch` reload internals can benefit indirectly
  from the seam patterns, but they are not the primary migration target here.
- **D-08:** Do not absorb secure skill staging, scanner fail-closed behavior,
  or broad `skills` extraction work; those belong to Phases 9-11.

### the agent's Discretion
- Exact module shape inside `src/app/watch.rs`, including helper structs and
  adapter layering.
- Concrete timeout defaults and concurrency limits for notify commands and
  webhook fanout, as long as failures are isolated and reported.
- Whether to model notification outcomes explicitly in code or rely on focused
  logging plus tests, provided the watch loop remains responsive.

</decisions>

<specifics>
## Specific Ideas

- Use a typed `WatchRequest` and `WatchService` so the `watch` family follows
  the same migration pattern as the shipped golden-path commands.
- Replace blocking `std::thread::sleep` and `child.status()` usage with Tokio
  async primitives.
- Add tests that simulate a slow webhook and a hanging notify command so the
  phase proves responsiveness rather than assuming it.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone and phase contract
- `.planning/ROADMAP.md` — Phase 8 goal, dependency order, and success
  criteria.
- `.planning/REQUIREMENTS.md` — `WATCH-03` and `CORE-05` define the required
  user-visible outcomes for this phase.
- `.planning/PROJECT.md` — milestone constraints, compatibility rules, and the
  product-preserving rewrite strategy.
- `.planning/STATE.md` — current milestone position and decisions inherited
  from Phase 7.

### Milestone research
- `.planning/research/ARCHITECTURE.md` — recommended command-family service
  pattern for `watch` and `skills`.
- `.planning/research/STACK.md` — existing Tokio/Reqwest/Notify stack choices
  for bounded watch runtime work.
- `.planning/research/FEATURES.md` — expected maturity bar for watch
  side-effect isolation and responsiveness.
- `.planning/codebase/CONCERNS.md` — repo-specific watch loop, blocking sleep,
  webhook timeout, and `main.rs` hotspot concerns.

### Current implementation touchpoints
- `src/main.rs` — current `Commands::Watch` loop plus notify command/webhook
  helpers.
- `src/app/mod.rs` and `src/app/status.rs` — established app-service pattern
  to mirror.
- `tests/cli_integration.rs` — current watch behavior coverage for output,
  notify file, webhook, and exit behavior.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/app/status.rs` already demonstrates the preferred request/service/outcome
  pattern for stable CLI commands.
- `status_value_with_health()` and `render_status_output()` in `src/main.rs`
  can stay as shared helpers initially while the watch loop moves behind a new
  app seam.
- Existing CLI integration tests already prove `watch` output flushing,
  unhealthy exit behavior, and webhook payload shape, which gives Phase 8 a
  good contract harness.

### Established Patterns
- Command-family extractions in this repo keep `main.rs` responsible for flag
  parsing and request assembly while pushing orchestration into `src/app/`.
- Async external effects already use Tokio and Reqwest elsewhere in the repo,
  so watch hardening should reuse those primitives instead of adding new
  runtime layers.

### Integration Points
- The `Commands::Watch` branch in `src/main.rs` currently mixes status polling,
  render diffing, notification fanout, and exit policy inside one loop.
- `run_watch_notify_command()` currently blocks on `child.status()`, and
  `send_watch_webhook()` currently awaits sequential requests without explicit
  timeout handling.
- The watch integration tests in `tests/cli_integration.rs` are the best place
  to add slow-notification regression coverage without changing the public CLI
  surface.

</code_context>

<deferred>
## Deferred Ideas

- Watch backend changes inside `src/server/mod.rs` beyond what the CLI runtime
  seam indirectly benefits from.
- Ephemeral skill staging, git materialization fixes, and atomic activation —
  Phase 9.
- Unified managed asset enforcement for install, scan, and serve — Phase 10.
- Broader `skills` extraction and parity gates — Phase 11.

</deferred>

---

*Phase: 08-watch-runtime-hardening*
*Context gathered: 2026-04-04*
