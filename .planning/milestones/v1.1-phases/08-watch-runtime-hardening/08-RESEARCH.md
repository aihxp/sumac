# Phase 8: Watch Runtime Hardening - Research

**Researched:** 2026-04-04
**Domain:** Long-running watch runtime responsiveness and CLI seam extraction
for a shipped Rust CLI
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Move `sxmc watch` orchestration behind a dedicated app/runtime seam while
  keeping current CLI flags, output, exit semantics, and wrapper entrypoints.
- Make slow or failing notify commands and webhooks isolated, bounded side
  effects instead of loop-blocking work.
- Reuse the existing Tokio/Reqwest stack; do not introduce a new runtime or
  event framework for this phase.
- Keep the phase focused on `watch`, not broader `skills` extraction or
  install/serve policy work.

### the agent's Discretion
- Exact module structure for `src/app/watch.rs`.
- Concrete timeout defaults and concurrency bounds.
- Whether notification outcomes become explicit types or stay as internal
  logging plus regression coverage.

### Deferred Ideas (OUT OF SCOPE)
- Secure skill staging and atomic activation.
- Unified scan/serve enforcement over the managed asset inventory.
- Broader command-family extraction for `skills`.
</user_constraints>

<research_summary>
## Summary

Phase 8 should extract the CLI `watch` loop into `src/app/watch.rs` and use
Tokio-native side-effect handling so notification commands and webhooks cannot
stall the long-running status loop. The repo already has the right stack for
this: Tokio for async sleep, subprocesses, and timeouts; Reqwest for HTTP with
builder-configured deadlines; and an established app-service pattern in
`src/app/status.rs`. The main gap is not capability but structure: the current
`Commands::Watch` branch in `src/main.rs` still owns too much orchestration and
runs blocking work directly in the async command path.

The strongest implementation direction is a two-step migration. First, create a
typed `WatchRequest` and `WatchService` that owns frame collection, render
diffing, notification decisions, and exit policy. Second, harden notification
fanout with Tokio subprocess execution, timeout-wrapped webhook requests, and
concurrent or otherwise isolated delivery so one slow endpoint or child process
cannot serialize the loop.

**Primary recommendation:** introduce `src/app/watch.rs` as the canonical CLI
watch seam, then make notify command/webhook delivery bounded and isolated
inside that service with regression coverage for slow side effects.
</research_summary>

<standard_stack>
## Standard Stack

The established tools already present in this repo for this domain:

### Core
| Tool | Purpose | Why Standard Here |
|------|---------|-------------------|
| `src/app/status.rs` pattern | Request/service/outcome seam | Proven migration shape for stable CLI behavior |
| `tokio::time::sleep` and `tokio::time::timeout` | Non-blocking intervals and deadlines | Already shipped in the repo and directly solves watch stalls |
| `tokio::process::Command` | Async notify command execution | Avoids blocking the runtime on child status collection |
| `reqwest::ClientBuilder` | Webhook timeout policy | Existing stack for outbound HTTP with deadline support |
| `tests/cli_integration.rs` | Contract-level watch proof | Already covers watch output and notification payload behavior |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `.planning/research/ARCHITECTURE.md` | App-service boundary guidance | To keep the `watch` seam aligned with the rest of the rewrite |
| `.planning/research/STACK.md` | Concrete runtime recommendations | To anchor timeout/process choices in milestone research |
| `.planning/codebase/CONCERNS.md` | Repo-local watch failure map | To prioritize real blocking and timeout gaps |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| App-service extraction in `src/app/watch.rs` | Leave orchestration in `src/main.rs` and only patch timeouts | Would fix one symptom but keep the main CLI hotspot intact |
| Tokio-native notify/webhook handling | Background threads with blocking IO | Harder to cancel and less aligned with the repo’s async command path |
| Bounded concurrent delivery | Sequential awaited requests | Keeps the current stall behavior when one endpoint is slow |
</standard_stack>

<architecture_patterns>
## Architectural Patterns

### Pattern 1: Command-Family App Service
**What:** Mirror `src/app/status.rs` by moving the typed watch request and
runtime orchestration into `src/app/watch.rs`.
**Why here:** Phase 8 explicitly requires a dedicated runtime seam (`CORE-05`)
and the repo already uses this shape successfully.
**Recommended here:** yes, as the primary structural change.

### Pattern 2: Async Side-Effect Isolation
**What:** Run notify commands and webhook sends behind timeout-bounded async
helpers so notification work cannot freeze the main loop.
**Why here:** `WATCH-03` is about bounded, isolated failures more than about
transport changes.
**Recommended here:** yes, using Tokio and Reqwest deadlines already in-tree.

### Pattern 3: Contract-First Refactoring
**What:** Preserve public flags/output/exit behavior while changing internal
ownership and adding slow-side-effect tests.
**Why here:** `watch` already ships and has wrapper consumers, so internal seam
work must be proved by behavior tests rather than structural arguments alone.
**Recommended here:** use the existing CLI integration tests plus new
responsiveness regressions.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Long-running watch intervals | Manual thread sleeps inside async code | `tokio::time::sleep` | Keeps the runtime responsive and cancellation-aware |
| Notify command execution | Blocking `std::process::Command::status()` | `tokio::process::Command` with timeout | Prevents the watch loop from waiting indefinitely on children |
| Webhook reliability | Per-request ad hoc clients with no deadline | Shared/client-built Reqwest requests with timeout | Gives explicit failure bounds and consistent behavior |
| Structural extraction | A second ad hoc watch helper pile in `main.rs` | `src/app/watch.rs` request/service seam | Matches the repo’s migration pattern and reduces hotspot coupling |
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Moving code without fixing blocking behavior
**What goes wrong:** the seam exists on paper, but `watch` still blocks on
sleep, child processes, or webhook latency.
**How to avoid:** treat extraction and bounded side effects as separate,
explicit deliverables.

### Pitfall 2: Fixing timeouts while breaking the CLI contract
**What goes wrong:** output cadence, exit behavior, or notification payloads
drift while the runtime is refactored.
**How to avoid:** keep the existing integration tests green and add focused
slow-side-effect tests instead of reworking the user-facing contract.

### Pitfall 3: Serializing webhook fanout behind one bad endpoint
**What goes wrong:** a timeout helps eventually, but one slow endpoint still
delays every other notification and next-frame processing.
**How to avoid:** isolate or bound fanout so the loop can move on after
launching or timing out notification work.
</common_pitfalls>

## Validation Architecture

Use a two-part validation model for this phase:

1. **Seam extraction proof**
   - `watch` request/service code lives in `src/app/watch.rs`
   - `main.rs` dispatch becomes thin request assembly and service invocation
   - existing watch contract tests stay green

2. **Responsiveness proof**
   - slow or hanging notify command tests show the loop still progresses or
     exits under explicit timeout behavior
   - slow webhook tests show the watch loop does not stall indefinitely and
     emits explicit failure reporting
   - full validation stays green after the extraction

Recommended evidence targets for this phase:
- `watch` app-service module plus thin `main.rs` dispatch
- async timeout handling for notify commands and webhooks
- integration coverage for slow notification side effects

<sources>
## Sources

### Primary (HIGH confidence)
- `.planning/phases/08-watch-runtime-hardening/08-CONTEXT.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/STACK.md`
- `.planning/research/FEATURES.md`
- `.planning/codebase/CONCERNS.md`
- `src/main.rs`
- `src/app/status.rs`
- `tests/cli_integration.rs`

### Secondary (MEDIUM confidence)
- `src/server/mod.rs` — related watch/runtime patterns, though not the main
  Phase 8 migration target
</sources>
