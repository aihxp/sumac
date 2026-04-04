# Architecture Patterns: Sumac Core/App Rewrite

**Domain:** Brownfield Rust CLI/MCP rewrite with stable `1.x` command contracts
**Researched:** 2026-04-04
**Overall confidence:** HIGH for boundary direction, MEDIUM for exact cut lines inside current `main.rs`

## Executive Recommendation

Sumac should become a **modular monolith with a thin stable CLI shell**, not a multi-crate reset and not a second product hidden beside the first. Keep `clap` command definitions and public output contracts stable at the edge, introduce a new **internal app layer** as the orchestration boundary, and let that app layer depend on a **core domain/policy layer** plus a small set of infrastructure ports. Existing modules (`cli_surfaces`, `skills`, `server`, `client`, `bake`, `paths`) should initially be treated as infrastructure behind those ports.

The migration seam is not "old binary vs new binary". The seam is:

`stable CLI contract -> command adapter -> app use case -> ports -> new implementation or legacy shim -> stable presenter`

That shape allows Sumac to move one command family at a time while preserving help text, flags, JSON output, generated files, exit behavior, and release cadence.

## Recommended Architecture

```text
User / scripts / CI
        |
        v
  clap CLI surface (`cli_args.rs`)
        |
        v
CLI adapters (`src/cli/`)
  - parse already done
  - map args to typed request DTOs
  - choose human/json presenter
  - set exit codes
        |
        v
App layer (`src/app/`)
  - command-family orchestration
  - transactions / sequencing / retries
  - "setup", "add", "status", "sync" use cases
        |
        v
Core layer (`src/core/`)
  - domain models
  - planning / diff / reconciliation rules
  - stable internal request/result contracts
  - ports for IO
        |
        +-------------------------------+
        |                               |
        v                               v
Infra adapters (`src/infra/`)      Legacy shims (`src/legacy/`)
  - cli_surfaces                        - extracted no-behavior-change
  - paths/cache/output                    wrappers over old logic
  - bake/store                          - temporary fallback path
  - skills/server/client
  - fs/network/process
        |
        v
filesystem / env / subprocess / MCP / HTTP / AI host files
```

## Opinionated Structure

Use **new modules inside the existing crate first**. Do not begin with a workspace split. The current rewrite risk is orchestration sprawl, not package-management failure. A crate split too early would add churn at the exact moment the team needs cheap refactors and mixed old/new code paths.

Suggested shape:

```text
src/
├── main.rs                  # composition root only
├── cli_args.rs              # stable clap surface
├── cli/
│   ├── mod.rs
│   ├── onboarding.rs        # setup/add/status/sync command adapters
│   ├── inspect.rs
│   ├── skills.rs
│   └── mcp.rs
├── app/
│   ├── mod.rs
│   ├── onboarding/
│   │   ├── add.rs
│   │   ├── setup.rs
│   │   ├── status.rs
│   │   ├── sync.rs
│   │   └── reconcile.rs
│   └── shared/
├── core/
│   ├── mod.rs
│   ├── contracts/
│   ├── onboarding/
│   │   ├── model.rs
│   │   ├── policy.rs
│   │   ├── diff.rs
│   │   └── ports.rs
│   └── shared/
├── infra/
│   ├── mod.rs
│   ├── cli_surfaces.rs
│   ├── install_paths.rs
│   ├── bake_store.rs
│   ├── skill_runtime.rs
│   └── mcp_runtime.rs
├── legacy/
│   ├── mod.rs
│   └── onboarding.rs
└── runtime/
    ├── mod.rs
    └── app_runtime.rs
```

## Component Boundaries

| Component | Responsibility | Must Not Own | Communicates With |
|-----------|----------------|--------------|-------------------|
| `cli` | Command entrypoints, arg-to-request mapping, output selection, exit codes | Business policy, filesystem traversal, direct host mutation logic | `app`, presenters |
| `app` | Use-case orchestration and sequencing for one command family | `clap`, raw stdout printing, protocol transport details | `core`, `runtime` |
| `core` | Domain rules, diff/reconcile logic, typed request/result models, port traits | Filesystem, env, network, subprocess, `serde_json::Value`-driven flow control | `app`, `infra` via ports |
| `infra` | Adapters to current modules and external systems | CLI parsing, command-family sequencing decisions | `core` ports, existing `src/*` modules |
| `legacy` | Temporary wrappers over extracted old flows, normalized to new contracts | New policy or new feature work | `app`, old helpers |
| `runtime` | Dependency wiring, cancellation, task ownership, config assembly | Command logic | `cli`, `app`, `infra` |

## Migration Seams

### Seam 1: Stable Command Contracts

Each migrated command should get an internal request/result pair that is independent of `clap`.

Example:

```rust
pub struct StatusRequest {
    pub install_scope: InstallScope,
    pub only_hosts: Vec<AiClientProfile>,
    pub compare_hosts: Vec<AiClientProfile>,
    pub include_health: bool,
}

pub struct StatusResult {
    pub drift: DriftSummary,
    pub startup_files: Vec<StartupFileState>,
    pub baked_health: Option<BakedHealthSummary>,
}
```

`clap` types stay at the edge. Presenters render these results into the existing human and JSON contracts.

### Seam 2: Legacy/New Router Per Command Family

The app layer should route a command to either:

- a new implementation that uses `core` + `infra`, or
- a legacy shim that calls extracted current logic and normalizes its result

This lets the command path move one slice at a time without branching at the CLI surface.

### Seam 3: Presenter Boundary

Today much of the public contract is assembled as ad hoc `serde_json::Value` in `main.rs`. Move that responsibility into presenters. The presenter becomes the sole owner of:

- JSON field names
- human output formatting
- exit-code interpretation

This is the critical seam for protecting `1.x` compatibility while internals change.

### Seam 4: Port-Based Infrastructure

Only add traits where the dependency is unstable or stateful. Good first ports:

- `CliInspector`
- `ArtifactPlanner`
- `ArtifactWriter`
- `InstallRootStore`
- `SyncStateStore`
- `BakedHealthProbe`
- `Clock`

Do not trait-abstract every function. The goal is migration seams, not maximal indirection.

### Seam 5: Parity Harness

For each migrated command family, keep a parity harness that can compare:

- structured JSON output
- generated file set
- write/no-write behavior in preview/apply modes
- exit codes

For the golden path, this matters more than unit-test purity.

## Data And Control Flow

### Normal migrated path

1. `clap` parses the stable command.
2. `src/cli/<family>.rs` maps args into a typed request.
3. `src/app/<family>/<command>.rs` orchestrates the use case.
4. `src/core/<family>/` computes policies, diffs, plans, and target actions.
5. `src/infra/` calls current modules or external systems to inspect, read, write, or execute.
6. The app returns a typed result.
7. The presenter renders the existing JSON/human contract.
8. CLI adapter prints output and sets exit code.

### Transitional legacy-backed path

1. `clap` parses the stable command.
2. CLI adapter builds the same typed request.
3. App router delegates to `src/legacy/<family>.rs`.
4. Legacy shim calls extracted old helpers from current `main.rs`/adjacent modules.
5. Legacy output is normalized into the same typed result or presenter-ready envelope.
6. Presenter renders the existing external contract.

The user should not be able to tell which path executed except through improved internal maintainability.

## Onboarding/Reconciliation Slice

The first migrated family should be **onboarding/reconciliation**, because current evidence shows `setup`, `add`, `status`, `sync`, `doctor`, and `watch` are tightly related but scattered across `main.rs`.

Recommended internal grouping:

| App family | Commands | Shared core concepts |
|------------|----------|----------------------|
| `onboarding` | `add`, `setup` | CLI inspection, host selection, artifact plan, materialization |
| `reconciliation` | `status`, `sync`, later `doctor` and `watch` | drift model, sync state, health evaluation, write plan |

These two can live under one `app/onboarding/` family at first, then split later if needed.

## Suggested Build Order

This should be dependency-first, not user-marketing-order.

1. **Foundation shell**
   - Create `cli`, `app`, `core`, `runtime`, and `legacy` modules.
   - Reduce `main.rs` to wiring and command dispatch only.
   - Add presenter ownership for `status`, `sync`, `add`, `setup` without changing behavior.

2. **Read-only status slice**
   - Migrate `status` first.
   - It is mostly read/evaluate/render, so it is the cheapest way to prove the architecture.
   - Introduce typed contracts, install-scope modeling, drift summary models, and parity tests.

3. **Shared reconciliation core**
   - Extract sync-state loading, drift comparison, health calculation, and recovery-plan logic into `core/onboarding/`.
   - Keep file IO behind ports.
   - This becomes the shared engine for `status`, `sync`, `watch`, and parts of `doctor`.

4. **`sync` write path**
   - Migrate `sync` after `status`.
   - It reuses the same domain model but adds write planning and application.
   - This is the first meaningful proof that the new app/core layer can mutate state safely.

5. **`add` onboarding path**
   - Move `add` onto the new app layer using ports over `cli_surfaces` inspection/materialization.
   - Preserve preview/apply semantics exactly.

6. **`setup` composition**
   - Rebuild `setup` as orchestration over the same `add`/inspection/materialization services.
   - Avoid a second standalone implementation; `setup` should become "multi-tool onboarding orchestration", not its own world.

7. **`doctor` and `watch` convergence**
   - Rehome them onto the same `status`/`sync` services.
   - `watch` should use Tokio cancellation coordination and task ownership instead of blocking sleeps in the command path.

8. **Apply the same pattern to the next families**
   - `inspect/materialize`
   - `skills/install/serve`
   - `mcp client/server bridge`
   - `discover/api/graphql/db/codebase/traffic`

## Anti-Patterns To Avoid

### Anti-pattern: Big-bang app-layer rewrite
Build the seam first, then move one command family at a time. A second universe of rewritten code with no production traffic is the fastest way to stall.

### Anti-pattern: Early workspace split
Separate crates before the boundaries are proven and every move gets slower. Sumac needs migration speed more than package purity right now.

### Anti-pattern: `serde_json::Value` as the app contract
`Value` is acceptable at the presenter boundary, not as the main orchestration model. Keep internal results typed.

### Anti-pattern: New and old logic selected in `main.rs`
`main.rs` should not become the permanent migration switchboard. Route through family adapters and app routers so the seam survives after migration.

### Anti-pattern: Duplicate implementations for `setup` and `add`
`setup` should compose onboarding services; it should not re-encode its own inspection/materialization rules.

## Runtime Considerations

- Keep `main.rs` as the composition root only.
- Centralize app dependency wiring in `runtime/app_runtime.rs`.
- For long-running server/watch flows, use explicit cancellation coordination and wait-for-completion ownership rather than blocking loops. This matches current Tokio guidance for graceful shutdown and task tracking.
- Keep MCP transport and session details in infrastructure. `rmcp` and `axum` already provide the transport/session primitives; the app layer should only express intent like "serve this tool surface" or "stop watching now".

## Roadmap Implications

- The first rewrite milestone should target **one family, one seam, one parity harness**: onboarding/reconciliation.
- The first externally invisible deliverable is not "new architecture exists". It is "`status` runs through the new app/core boundary with unchanged output".
- `sync` should follow immediately, because it forces the write path and validates that the new boundary can safely own side effects.
- `add` and `setup` should land after the shared reconciliation/artifact-planning seams are stable, not before.
- `doctor` and `watch` should be treated as downstream consumers of the same family, not separate architectures.

## Confidence Notes

- **HIGH:** Sumac should use a modular-monolith migration inside the current crate rather than an early workspace split. Repo evidence points to orchestration concentration, not package-boundary failure.
- **HIGH:** Stable command contracts need a presenter boundary. Current `main.rs` mixes orchestration and contract rendering heavily, so output preservation needs an explicit owner.
- **HIGH:** Onboarding/reconciliation is the right first family. Current `setup`, `add`, `status`, `sync`, `doctor`, and `watch` already share install-scope, host, artifact, and sync-state concerns.
- **MEDIUM:** Exact module names and family partitioning may shift once more helpers are extracted from `main.rs`, but the seam pattern should remain the same.

## Sources

- Repo planning context: `/Users/hprincivil/Projects/sxmc/.planning/PROJECT.md`
- Repo codebase architecture map: `/Users/hprincivil/Projects/sxmc/.planning/codebase/ARCHITECTURE.md`
- Repo structure map: `/Users/hprincivil/Projects/sxmc/.planning/codebase/STRUCTURE.md`
- Repo concern map: `/Users/hprincivil/Projects/sxmc/.planning/codebase/CONCERNS.md`
- Current command surface and dependency context: `/Users/hprincivil/Projects/sxmc/src/cli_args.rs`, `/Users/hprincivil/Projects/sxmc/src/main.rs`, `/Users/hprincivil/Projects/sxmc/Cargo.toml`
- `clap` derive docs on subcommand delegation and `flatten`: https://docs.rs/clap/latest/clap/_derive/ and https://docs.rs/clap/latest/clap/trait.Subcommand.html
- Tokio graceful shutdown guidance: https://tokio.rs/tokio/topics/shutdown
- Axum graceful shutdown surface: https://docs.rs/axum/latest/axum/serve/struct.WithGracefulShutdown.html
- `rmcp` transport and session docs: https://docs.rs/rmcp/latest/rmcp/transport/index.html and https://docs.rs/rmcp/latest/rmcp/transport/streamable_http_server/session/index.html
