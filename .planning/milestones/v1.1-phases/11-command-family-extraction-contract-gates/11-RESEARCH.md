# Phase 11: Command-Family Extraction & Contract Gates - Research

**Researched:** 2026-04-04
**Domain:** App-service extraction for remaining CLI command families with
real-fixture contract gates
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- `skills` and top-level `serve` orchestration should move behind clearer app
  service seams.
- The extraction must preserve current CLI behavior, JSON output, exit codes,
  and file side effects.
- Real-fixture integration tests are the gate that proves the extraction is
  safe.
- No new framework or architectural layer should be introduced.

### the agent's Discretion
- Exact request enums/structs for `app::skills` and `app::serve`.
- Which shared helpers remain in `main.rs` versus move into the new app
  modules.
- The precise contract-test set as long as it covers the migrated command
  families on real fixtures.

### Deferred Ideas (OUT OF SCOPE)
- Rollback-retirement decisions and soak evidence review
- Extraction of unrelated command families
- Any public CLI redesign
</user_constraints>

<research_summary>
## Summary

Phase 11 should follow the same pattern already proven by `app::watch` and the
golden-path services: keep `main.rs` responsible for parsing CLI args and
assembling typed requests, then hand off the behavior to focused app services.
The main code hotspot left by v1.1 is the large inline `Commands::Skills`
match arm plus the top-level `serve` wrapper logic that still performs
registration setup and transport dispatch directly in `main.rs`.

The strongest implementation direction is to introduce `app::skills` and
`app::serve` as thin orchestration seams that reuse existing command-handler,
install, generator, and server modules rather than rewriting them. The right
proof mechanism is already present in the repo: `tests/cli_integration.rs` has
real-fixture `skills install/update` tests, stdio hybrid MCP coverage, and a
`serve --watch` reload flow. Phase 11 should treat those tests as contract
gates and add any missing end-to-end cases needed to pin the extracted surface.

**Primary recommendation:** first extract `skills` and `serve` into app
services with minimal behavior changes, then tighten the CLI integration suite
so the migrated families have explicit contract coverage before Phase 12 soak
work.
</research_summary>

<standard_stack>
## Standard Stack

The established tools already present in this repo for this domain:

### Core
| Tool | Purpose | Why Standard Here |
|------|---------|-------------------|
| `src/app/*` service pattern | Command-family orchestration seam | Already used successfully for golden-path commands and `watch` |
| `src/command_handlers.rs` | Stable `skills` list/info/run behavior | Lets Phase 11 move ownership without re-implementing behavior |
| `src/server::serve_stdio/http` | Embeddable serve library boundary | Keeps the new `app::serve` seam CLI-focused rather than duplicating server logic |
| `tests/cli_integration.rs` + `assert_cmd` | Real CLI contract proof | Already contains the right fixture-driven safety checks |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `.planning/research/ARCHITECTURE.md` | App-service and `serve` seam guidance | To keep extraction aligned with milestone architecture |
| `.planning/codebase/CONCERNS.md` | Main-file hotspot justification | To ensure the phase closes the documented orchestration risk |
| `app::watch::WatchService` | Local reference implementation | To mirror request/result structure and ownership patterns |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `app::skills` and `app::serve` seams | Leave large inline branches in `main.rs` | Preserves the hotspot and makes contract reasoning harder |
| Real-fixture CLI integration gates | Only unit tests around new services | Would miss the observable CLI contract the milestone promises to preserve |
| Reusing command handlers and server entrypoints | Rewriting behavior inside the new app modules | Adds risk without helping the phase goal |
</standard_stack>

<architecture_patterns>
## Architectural Patterns

### Pattern 1: Thin Main, Typed Requests
**What:** `main.rs` resolves flags and shared inputs, then hands typed requests
to app services.
**Why here:** This is the repo's established migration pattern for reducing
top-level orchestration hotspots.
**Recommended here:** yes, for both `skills` and `serve`.

### Pattern 2: Reuse Existing Domain Helpers
**What:** the new app services orchestrate existing command handlers,
install/update logic, and server entrypoints.
**Why here:** Phase 11 is about boundaries and contract safety, not rewriting
already-hardened domain code.
**Recommended here:** yes.

### Pattern 3: Contract Gates via Real Fixtures
**What:** rely on CLI integration tests over real fixtures and temp installs as
the migration proof.
**Why here:** `ROL-05` explicitly requires parity or contract coverage before
full cutover.
**Recommended here:** yes, with explicit focus on migrated `skills` and `serve`
flows.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Command extraction | A new framework or command bus | Existing `src/app/*` request/service seam | The repo already has the right migration pattern |
| Serve wrapper migration | A duplicate server implementation inside `app::serve` | Reuse `server::serve_stdio/http` | Keeps the app seam focused on CLI orchestration only |
| Contract proof | New synthetic mocks for CLI behavior | Existing real-fixture `assert_cmd` coverage | The user-facing contract is what must stay stable |
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Moving logic without preserving print/exit behavior
**What goes wrong:** the new service seam compiles but subtly changes stdout,
stderr, or exit codes.
**How to avoid:** keep printing and exit-policy behavior explicit in the app
service and pin it with integration tests.

### Pitfall 2: Rewriting lower-level helpers during extraction
**What goes wrong:** the phase balloons into behavior change instead of seam
cleanup.
**How to avoid:** reuse `command_handlers`, install/update, and server entry
points rather than re-implementing them.

### Pitfall 3: Treating existing tests as enough without naming the gap
**What goes wrong:** migrated families look tested, but the specific contract
risks from extraction are not actually pinned.
**How to avoid:** add or tighten tests specifically around `skills`
install/update/list/info and `serve` hybrid/watch behavior as the migration
gate.
</common_pitfalls>

## Validation Architecture

Use a two-part validation model for this phase:

1. **Service extraction proof**
   - `skills` dispatch moves from the large inline `main.rs` branch into
     `app::skills`
   - `serve` wrapper orchestration moves behind `app::serve`
   - `main.rs` stays a thin request-assembly layer for those families

2. **Contract gate proof**
   - real-fixture `skills list`, `info`, `install`, and `update` flows still
     pass with the same user-visible outcomes
   - stdio hybrid `serve` skill exposure still works
   - `serve --watch` or equivalent migrated serve flow still passes its real
     fixture regression

Recommended evidence targets for this phase:
- focused integration tests for extracted `skills` flows
- focused integration tests for extracted `serve` flows
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

<sources>
## Sources

### Primary (HIGH confidence)
- `.planning/phases/11-command-family-extraction-contract-gates/11-CONTEXT.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/STACK.md`
- `.planning/codebase/CONCERNS.md`
- `src/main.rs`
- `src/app/mod.rs`
- `src/app/watch.rs`
- `src/command_handlers.rs`
- `tests/cli_integration.rs`

### Secondary (MEDIUM confidence)
- `src/skills/install.rs` — install/update behavior that must remain stable
- `src/server/mod.rs` and `src/server/handler.rs` — serve entrypoints and
  behavior preserved by the wrapper extraction
</sources>
