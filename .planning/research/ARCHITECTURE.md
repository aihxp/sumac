# Architecture Research

**Domain:** Milestone v1.1 integration architecture for an existing Rust CLI/MCP product
**Researched:** 2026-04-04
**Confidence:** HIGH

## Standard Architecture

### System Overview

```text
┌──────────────────────────────────────────────────────────────────────┐
│ Stable CLI Surface                                                  │
├──────────────────────────────────────────────────────────────────────┤
│ `src/cli_args.rs`        `src/main.rs`                              │
│ keep flags/contracts     thin dispatch + request assembly           │
└───────────────┬───────────────────────────────┬──────────────────────┘
                │                               │
                v                               v
┌──────────────────────────────────────────────────────────────────────┐
│ App Services (`src/app/`)                                           │
├──────────────────────────────────────────────────────────────────────┤
│ existing: `golden_path`, `add`, `setup`, `status`, `sync`           │
│ new: `watch`, `skills`, optional CLI-only `serve` wrapper           │
│ shared: reconciliation/status facade and presenters                 │
└───────────────┬───────────────────────────────┬──────────────────────┘
                │                               │
                v                               v
┌──────────────────────────────────────────────────────────────────────┐
│ Domain/Runtime Library Modules                                      │
├──────────────────────────────────────────────────────────────────────┤
│ `src/server/`      reload runtime, transports, inventory swap       │
│ `src/skills/`      catalog, parser, install pipeline, metadata      │
│ `src/security/`    scan policy over canonical skill assets          │
│ `src/output/`      stable human/JSON rendering                      │
│ `src/paths/`       install scope and filesystem roots               │
└───────────────┬───────────────────────────────┬──────────────────────┘
                │                               │
                v                               v
┌──────────────────────────────────────────────────────────────────────┐
│ External Effects                                                    │
├──────────────────────────────────────────────────────────────────────┤
│ filesystem   git repos   subprocesses   notify/poll   HTTP/MCP      │
└──────────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility | Typical Implementation |
|-----------|----------------|------------------------|
| `src/main.rs` | Preserve `clap` contract and delegate by command family | Thin match arms that build typed request structs and call app services |
| `src/app/` | Own command-family orchestration and exit behavior | Request/result services matching the existing `add`/`status`/`sync` pattern |
| `src/server/` | Own skill-server construction, reload runtime, and HTTP/stdio transport | Keep public `serve_*` entrypoints stable, split internals into smaller modules |
| `src/skills/` | Own canonical skill tree model, install/update staging, and metadata | Shared catalog/policy layer used by parser, install, serve, and scan |
| `src/security/` | Enforce scanner coverage over served/installable assets | Scan canonical asset inventory and report skips as findings, not silent drops |

## Recommended Project Structure

```text
src/
├── main.rs                    # composition root and command dispatch only
├── cli_args.rs                # stable clap surface
├── app/
│   ├── mod.rs
│   ├── golden_path.rs         # existing stable seam
│   ├── add.rs                 # existing
│   ├── setup.rs               # existing
│   ├── status.rs              # existing, move shared status helpers behind facade
│   ├── sync.rs                # existing, same facade
│   ├── watch.rs               # new: `sxmc watch` service
│   ├── skills.rs              # new: skills command-family service
│   └── serve.rs               # optional: CLI-only serve registration/validation
├── server/
│   ├── mod.rs                 # keep public API, delegate internally
│   ├── handler.rs             # serve only allowlisted assets
│   ├── reload.rs              # new: watch backend + debounce + atomic swap
│   └── inventory.rs           # new: inventory summary derived from built server/catalog
├── skills/
│   ├── mod.rs
│   ├── models.rs              # extend with canonical asset model
│   ├── parser.rs              # build recursive asset inventory
│   ├── install.rs             # keep public entrypoints, delegate to pipeline stages
│   ├── source.rs              # new: source resolution/materialization
│   ├── policy.rs              # new: allowlist and validation rules
│   └── staging.rs             # new: atomic install/update application
└── security/
    └── skill_scanner.rs       # scan asset inventory, not ad hoc file subsets
```

### Structure Rationale

- **Extend `src/app/`, do not invent a second orchestration layer:** the golden-path service pattern already exists and should become the norm for the remaining command families.
- **Keep `src/server/` as the embeddable library boundary:** `serve_stdio`, `serve_http`, and `build_server` are useful crate-level APIs and should stay there even if CLI-only prep work moves into `src/app/serve.rs`.
- **Make `src/skills/` the source of truth for what a skill contains:** install, serve, scan, and watch are currently each operating on slightly different file models. v1.1 should end that divergence.

## Architectural Patterns

### Pattern 1: Command-Family App Service

**What:** Mirror the existing `src/app/add.rs` and `src/app/status.rs` shape for the remaining hotspots instead of keeping those branches in `main.rs`.
**When to use:** Any command family that has non-trivial sequencing, notifications, install scope handling, or exit-code policy.
**Trade-offs:** Adds request/result structs and some adapter code, but it removes the current `main.rs` hotspot and makes parity testing practical.

**Example:**
```rust
pub(crate) struct WatchRequest {
    pub(crate) install_paths: InstallPaths,
    pub(crate) only_hosts: Vec<AiClientProfile>,
    pub(crate) compare_hosts: Vec<AiClientProfile>,
    pub(crate) health: bool,
    pub(crate) interval: Duration,
    pub(crate) notify: WatchNotifyOptions,
    pub(crate) exit_on_change: bool,
    pub(crate) exit_on_unhealthy: bool,
    pub(crate) pretty: bool,
    pub(crate) format: Option<output::StructuredOutputFormat>,
}

pub(crate) struct WatchService;

impl WatchService {
    pub(crate) async fn run(&self, request: WatchRequest) -> Result<CommandOutcome> {
        // status snapshot -> change detection -> notify fanout -> exit policy
        Ok(CommandOutcome::default())
    }
}
```

### Pattern 2: Canonical Skill Asset Inventory

**What:** Extend `Skill` with a canonical asset inventory that describes every installable/servable file and its role, while keeping `scripts` and `references` as derived compatibility views.
**When to use:** Skill install, update, scan, server resource exposure, and reload fingerprinting.
**Trade-offs:** Requires touching `skills/models.rs`, `parser.rs`, `server/handler.rs`, and `security/skill_scanner.rs`, but it eliminates the current mismatch where parser, handler, and watch each see a different tree.

**Example:**
```rust
pub enum SkillAssetKind {
    Prompt,
    Script,
    Reference,
    Metadata,
}

pub struct SkillAsset {
    pub relative_path: String,
    pub absolute_path: PathBuf,
    pub kind: SkillAssetKind,
    pub readable: bool,
}

pub struct Skill {
    pub name: String,
    pub base_dir: PathBuf,
    pub assets: Vec<SkillAsset>,
    pub scripts: Vec<SkillScript>,
    pub references: Vec<SkillReference>,
    // existing fields unchanged
}
```

### Pattern 3: Single-Build Reload Pipeline

**What:** For `serve --watch`, build one updated server snapshot per change, derive inventory from that same snapshot, then swap atomically.
**When to use:** Both filesystem-event and poll fallback reload paths.
**Trade-offs:** Slightly more internal structure in `src/server/`, but it removes the current duplicate build and gives one place to improve debounce, polling, and metrics.

**Example:**
```rust
struct ReloadResult {
    server: SkillsServer,
    inventory: SkillInventorySummary,
}

fn rebuild(paths: &[PathBuf], snapshots: &[PathBuf], manifests: &[PathBuf]) -> Result<ReloadResult> {
    let server = build_server(paths, snapshots, manifests)?;
    let inventory = SkillInventorySummary::from_server(&server);
    Ok(ReloadResult { server, inventory })
}
```

## Data Flow

### Request Flow

```text
User command
    ↓
`clap` in `src/cli_args.rs`
    ↓
`src/main.rs` builds typed request
    ↓
`src/app/<family>.rs`
    ↓
shared library modules (`server`, `skills`, `security`, `paths`, `output`)
    ↓
filesystem / git / notify / HTTP / MCP / subprocess
    ↓
typed result + stable presenter
    ↓
stdout + exit code
```

### State Management

```text
Install roots (`src/paths.rs`)
    ↓
skill roots / AI host files / baked config / cache
    ↓
app services read current state
    ↓
library modules compute changes or serve runtime state
    ↓
presenters render stable output
```

### Key Data Flows

1. **`sxmc watch` flow:** `main.rs` -> new `app::watch::WatchService` -> shared status snapshot collector -> render diff/change detector -> async notification fanout -> exit policy.
2. **`sxmc serve --watch` flow:** `main.rs` or `app::serve` -> `server::serve_stdio/http` -> `server::reload` watch backend -> canonical skill catalog rebuild -> atomic `ReloadableSkillsServer` swap.
3. **`sxmc skills install/update` flow:** `main.rs` or `app::skills` -> install pipeline -> source materialization -> allowlist/filter -> scanner coverage -> staging dir -> atomic rename -> metadata write.

## New Vs Modified Components

| Component | New or Modified | Why |
|-----------|-----------------|-----|
| `src/app/watch.rs` | New | `sxmc watch` is currently a large `main.rs` loop with blocking sleep, inline notifications, and exit handling. It should become a first-class service beside `status` and `sync`. |
| `src/app/skills.rs` | New | The skills family still splits between `main.rs`, `command_handlers.rs`, and `skills/install.rs`. A family service is the right extraction seam. |
| `src/app/serve.rs` | New, optional | Keep only CLI-only serve concerns here: registration, auth flag validation, and transport selection. Leave transport runtime in `src/server/`. |
| `src/app/status.rs` and `src/app/sync.rs` | Modified | They should consume a shared reconciliation/status facade instead of helper functions living in `main.rs`. |
| `src/main.rs` | Modified | Shrink to dispatch and request assembly. No new watch/install/serve helpers should be added here. |
| `src/command_handlers.rs` | Modified, transitional | Freeze it as a compatibility adapter for list/info/run while `app::skills` comes online. Do not grow it further. |
| `src/server/reload.rs` | New | Isolate notify-vs-poll watching, debounce, rebuild-once, and atomic swap logic from `src/server/mod.rs`. |
| `src/server/inventory.rs` | New | Inventory summary should be derived from the current built server or catalog, not by building a second time. |
| `src/server/mod.rs` | Modified | Keep public entrypoints stable; delegate internals to `reload.rs` and `inventory.rs`. |
| `src/server/handler.rs` | Modified | Serve only allowlisted files from the canonical asset inventory; stop exposing arbitrary files under the skill root. |
| `src/skills/models.rs` | Modified | Add the canonical skill asset model. Existing `scripts` and `references` stay for compatibility. |
| `src/skills/parser.rs` | Modified | Build a recursive asset inventory and stop limiting watch-relevant modeling to one directory level. |
| `src/skills/source.rs` | New | Separate git/local source resolution and temp materialization from install application. |
| `src/skills/policy.rs` | New | Centralize what is installable and servable: `SKILL.md`, `scripts/**`, `references/**`, managed metadata, and explicit exclusions. |
| `src/skills/staging.rs` | New | Own staging, atomic rename, cleanup, and update replacement behavior. |
| `src/skills/install.rs` | Modified | Keep `install_skill`/`update_skills` as stable entrypoints, but turn them into orchestration over source/policy/staging. |
| `src/security/skill_scanner.rs` | Modified | Scan the canonical asset set and emit findings for unreadable/non-UTF-8/skipped files instead of silently continuing. |

## Internal Boundaries To Keep Stable

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `src/cli_args.rs` -> `src/main.rs` | direct | Preserve public flags, subcommands, and JSON/human contract selection. |
| `src/main.rs` -> `src/app/*` | direct typed requests | This should be the default integration seam for migrated command families. |
| `src/app/*` -> `sxmc::server` | direct API | Keep `serve_stdio`, `serve_http`, and `build_server` as the embeddable library boundary. |
| `src/app/*` -> `src/skills/*` | direct API | `app::skills` should orchestrate lifecycle; `skills` owns install/source/policy details. |
| `src/server/*` -> `src/skills/*` | direct API over canonical catalog | Server reload and handler logic should not re-discover its own file model. |
| `src/security/*` -> `src/skills/*` | direct API over canonical catalog | Scan coverage must follow the same asset selection rules as install/serve. |

## Boundaries That Should Remain Stable In v1.1

- `src/cli_args.rs` should remain the public CLI contract owner.
- `src/lib.rs` should continue exporting `server`, `skills`, `client`, `cli_surfaces`, `security`, and `paths` without a crate split.
- `src/server::serve_stdio`, `src/server::serve_http`, and `src/server::build_server` should stay callable from outside the CLI.
- `src/output/mod.rs`, `src/error.rs`, and `src/paths.rs` should remain shared infrastructure, not be rehomed during this milestone.
- The existing golden-path route seam in `src/app/golden_path.rs` should not be expanded into a generic second router for every new command. Use parity tests, not more env-flag routing, unless evidence forces a rollback seam.

## Data Flow Changes Required By This Milestone

### Watch Reliability

**`sxmc watch`:**
- Reuse the existing status/sync domain, do not build a second status engine.
- Move the loop into `src/app/watch.rs`.
- Replace blocking `std::thread::sleep` with `tokio::time::sleep`.
- Run notify command execution and webhook delivery behind async adapters with explicit timeouts and isolated failures.
- Keep rendering and exit policy in the app layer so public behavior stays stable.

**`sxmc serve --watch`:**
- Keep runtime ownership in `src/server/`.
- Replace `summarize_inputs_with_manifests()` + second `build_server()` with a single rebuild result.
- Fingerprint or enumerate the canonical recursive asset inventory, not just top-level `scripts/` and `references/`.
- Treat notify-based and poll-based watch backends as interchangeable event sources feeding one reload coordinator.

### Skill Install/Serve Hardening

- Install and serve must share one allowlist policy.
- `install_skill` should never copy the whole source tree blindly.
- The install pipeline should be: resolve source -> materialize source tree -> parse/catalog -> scan/validate -> copy allowlisted assets into staging -> write managed metadata -> atomic replace -> cleanup.
- `server/handler.rs` should read only assets that the policy marked servable. `get_skill_related_file()` should deny dotfiles, VCS directories, build outputs, unmanaged metadata, and anything not in the asset index.
- Scanner coverage should run over the same asset index and emit findings for skipped files so "clean scan" actually means "fully evaluated managed surface."

### Command-Family Extraction

- `watch` and `skills` should join the existing `src/app/` pattern before more logic is added.
- `serve` only needs an app wrapper for CLI-only concerns. The transport/server runtime remains a library concern.
- `command_handlers.rs` should become a shrinking adapter, not a new home for milestone work.

## Sensible Build Order

1. **Create command-family seams without behavior changes**
   - Add `src/app/watch.rs` and `src/app/skills.rs`.
   - Move `main.rs` branches for `watch` and `skills` into typed request/service calls.
   - Keep `command_handlers.rs` as a temporary adapter for list/info/run.
   - This lowers integration risk before changing deeper behavior.

2. **Extract shared reconciliation/status helpers out of `main.rs`**
   - Move `status_value_with_health`, `render_status_output`, and related watch-status helpers behind a shared facade used by `status`, `sync`, and `watch`.
   - This gives `sxmc watch` a stable dependency instead of depending on `main.rs` internals.

3. **Introduce the canonical skill asset model**
   - Extend `src/skills/models.rs` and `parser.rs` so one catalog describes recursive scripts/references and managed metadata.
   - Keep current public behavior but expose the new inventory internally.
   - This is the core dependency for both reload reliability and serve/install hardening.

4. **Split server reload internals**
   - Add `src/server/reload.rs` and `src/server/inventory.rs`.
   - Remove the double-build reload path.
   - Fix nested-asset polling by fingerprinting the canonical asset inventory.
   - Verify `serve --watch` parity before touching install behavior.

5. **Refactor install/update onto source/policy/staging**
   - Add `src/skills/source.rs`, `policy.rs`, and `staging.rs`.
   - Fix git clone base-path handling and tempdir cleanup.
   - Enforce allowlisted copy and scanner coverage before serving those installs.

6. **Harden serve/read paths**
   - Update `src/server/handler.rs` to read only the indexed servable assets.
   - Make `get_skill_related_file()` operate on asset lookup, not raw recursive filesystem access.
   - This closes the install/serve security loop.

7. **Reduce transitional glue**
   - Trim `command_handlers.rs`.
   - Keep `main.rs` as composition only.
   - Re-evaluate whether the golden-path rollback seam should stay isolated to the onboarding commands or be retired after soak evidence.

## Anti-Patterns

### Anti-Pattern 1: One "watch" abstraction for everything

**What people do:** Try to unify `sxmc watch` status polling and `sxmc serve --watch` server reload into one generic watcher module.
**Why it's wrong:** They observe different things, have different outputs, and fail differently. One is CLI status monitoring; the other is hot-reload infrastructure.
**Do this instead:** Share only low-level pieces like debounce utilities or notification adapters. Keep separate app/runtime owners.

### Anti-Pattern 2: Separate allowlists for install, serve, and scan

**What people do:** Fix install copying in one place, handler exposure in another, and scanner coverage in a third.
**Why it's wrong:** The system drifts again and security guarantees remain impossible to reason about.
**Do this instead:** Put the policy in `src/skills/policy.rs` and make install, serve, scan, and watch all consume it.

### Anti-Pattern 3: More logic in `main.rs` during extraction

**What people do:** Add a few more helper functions in `main.rs` "just for now" while migrating.
**Why it's wrong:** It extends the hotspot and makes later extraction harder.
**Do this instead:** New milestone logic goes into `src/app/`, `src/server/`, or `src/skills/` even if the first version is only a thin wrapper.

## Integration Points

### External Services

| Service | Integration Pattern | Notes |
|---------|---------------------|-------|
| local filesystem | direct through `paths`, `skills`, and staging modules | All install/serve/watch correctness depends on one canonical view of managed files. |
| git CLI | subprocess in `src/skills/source.rs` | Keep clone/materialization isolated so cleanup and subpath resolution are testable. |
| notify crate | `src/server/reload.rs` backend | Use as an event source only; debounce and rebuild logic should be separate. |
| HTTP/webhooks | async notifier adapter in `src/app/watch.rs` | Add timeouts and isolate failures from the main loop. |
| MCP transports | remain in `src/server/mod.rs` | Do not move transport details into app services. |

### Internal Boundaries

| Boundary | Communication | Notes |
|----------|---------------|-------|
| `main.rs` ↔ `app::watch` | typed request/result | New app seam for `sxmc watch`. |
| `main.rs` ↔ `app::skills` | typed request/result | New app seam for skills family extraction. |
| `app::watch` ↔ shared status facade | direct API | Watch should consume the same status data model as `status` and `sync`. |
| `app::skills` ↔ `skills::install` | direct API | CLI service owns orchestration; install module owns pipeline details. |
| `server::reload` ↔ `skills::parser/models` | direct API | Reload must use canonical inventory, not ad hoc directory hashing. |
| `server::handler` ↔ `skills::policy` | direct API | Serve-time file access must follow the same policy as install-time copying. |
| `security::skill_scanner` ↔ `skills::models` | direct API | Scanner coverage should be computed from the managed asset set. |

## Sources

- `.planning/PROJECT.md`
- `.planning/codebase/ARCHITECTURE.md`
- `.planning/codebase/CONCERNS.md`
- `.planning/codebase/STRUCTURE.md`
- `src/main.rs`
- `src/app/mod.rs`
- `src/app/golden_path.rs`
- `src/app/status.rs`
- `src/app/sync.rs`
- `src/command_handlers.rs`
- `src/server/mod.rs`
- `src/server/handler.rs`
- `src/skills/install.rs`
- `src/skills/parser.rs`
- `src/skills/models.rs`
- `src/security/skill_scanner.rs`
- `tests/cli_integration.rs`

---
*Architecture research for: v1.1 Platform Hardening and Core Expansion*
*Researched: 2026-04-04*
