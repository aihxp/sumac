# Technology Stack

**Project:** Sumac v1.1 Platform Hardening and Core Expansion
**Researched:** 2026-04-04
**Confidence:** HIGH

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why |
|------------|---------|---------|-----|
| Rust std `fs`/`path` + `std::process::Command` | stable std | Canonical path checks, allowlist installs, symlink rejection, lightweight git invocation | Sumac does not need a new systems abstraction layer for v1.1. The missing piece is disciplined use of `canonicalize`, `symlink_metadata`, and staged copy rules, not a new filesystem or Git stack. |
| `tokio` | Keep current `1.x` | Non-blocking watch loop, subprocess execution, deadlines, bounded concurrency | The repo already ships Tokio. The watch path should move from blocking `std::thread::sleep` and blocking child execution to `tokio::time::sleep`, `tokio::time::timeout`, `tokio::process::Command`, `JoinSet`, and `Semaphore`. That fixes reliability without a runtime change. |
| `notify` | Keep current `8.x` | Filesystem events plus poll-based fallback | Sumac already depends on `notify`, and `notify` already provides both `RecommendedWatcher` and `PollWatcher`. v1.1 should use those explicitly instead of maintaining a separate shallow fingerprint-only polling model. |
| `reqwest` | Keep current `0.13.x` in repo | Webhook notifications and other outbound HTTP in watch flows | The current stack is sufficient. The change needed is to stop creating ad hoc clients and instead use a shared `ClientBuilder` with total, connect, and read timeouts so one slow webhook cannot stall the loop. |
| `tempfile` | Keep current `3.x` | Clone/install staging and cleanup verification | `tempfile::TempDir` already gives Sumac the right install primitive. The bug is from persisting tempdirs with `keep()` in a transient path; v1.1 should keep clones scoped and call `close()` when cleanup success matters. |

### Supporting Libraries

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `walkdir` | `2.x` optional add | One canonical recursive tree walker for install, scan, serve, and polling fingerprints | Add this only if v1.1 consolidates skill-tree traversal into a shared utility. It is the one new crate that materially reduces duplicated recursion bugs across `install`, `server`, and `scanner`. |
| `regex::bytes` | Existing `regex` crate | Scan non-UTF-8 or binary-ish skill assets without silently skipping them | Use in `skill_scanner` when switching from `read_to_string` to raw-byte reads for scripts and references. No new dependency is required. |
| `tokio-util::sync::CancellationToken` | Already present | Coordinated shutdown for watch tasks, child processes, and notification fan-out | Keep using it as the boundary between the long-running watch supervisor and spawned work. It already fits the repo. |

### Development Tools

| Tool | Purpose | Notes |
|------|---------|-------|
| Rust test harness + Tokio tests | End-to-end watch reliability and async timeout coverage | Add tests for slow webhooks, hanging notify commands, and cancellation. Use real async timing behavior instead of unit-only mocks. |
| `assert_cmd` + `tempfile` + real `git` fixtures | CLI and skill install/update regression coverage | Build temp git repositories during tests to cover clone, repo-subpath resolution, cleanup, and allowlist behavior. No extra testing crate is required. |

## Installation

```bash
# No new runtime crates are required for v1.1 if Sumac keeps recursive traversal bespoke.

# Optional but recommended if v1.1 introduces one shared skill-tree walker:
cargo add walkdir@2
```

## Recommended Usage Changes

### 1. Watch Reliability

- Keep `notify` and replace the custom shallow polling fallback with `notify::PollWatcher`.
- Use `notify::Config::with_poll_interval(...)` explicitly instead of relying on defaults.
- For installed skill trees, configure watch traversal so symlinks are not followed.
- Enable `with_compare_contents(true)` only for roots that are known to have unreliable mtimes or event delivery; it is more expensive and should not be the default for all roots.
- Stop blocking the Tokio runtime in `Commands::Watch`: replace `std::thread::sleep` with `tokio::time::sleep`.
- Run `notify_command` through `tokio::process::Command`, wrap it in `tokio::time::timeout`, and set `kill_on_drop(true)` so cancellation or timeout does not leak child processes.
- Fan out webhook notifications concurrently with bounded concurrency (`JoinSet` + `Semaphore`) so one slow endpoint does not serialize the loop.
- Reuse one `reqwest::Client` with configured timeouts instead of creating a new client per event.

### 2. Secure Skills Lifecycle Hardening

- Keep using system `git`; do not switch to `git2` for this milestone.
- Materialize remote skills into a scoped `TempDir`, resolve repo subpaths from the actual clone directory, then copy only an explicit allowlist into the final installed skill root.
- The allowlist should be product-defined, not source-defined: `SKILL.md`, `scripts/`, `references/`, and any explicitly supported Sumac metadata file.
- Reject symlinks during install and validate the staged tree before the final rename. Use `std::fs::symlink_metadata` so checks do not follow links.
- Reuse the same canonical file model for install, MCP file exposure, recursive file listing, and polling fingerprints. Today those rules drift across modules.
- Change the scanner to read raw bytes and report skipped/unreadable files as findings; do not silently continue on unreadable or non-UTF-8 script/reference content.

### 3. CLI Orchestration Refactor

- Do not add a new framework. Keep the current `clap` + `src/app/*` request/service seam that v1.0 already established.
- Move `watch` and `skills` onto the same pattern as `setup`/`status`: request structs, small services, and thin dispatch in `main.rs`.
- Keep async only at IO boundaries. Parsing, policy validation, allowlist selection, and install planning should stay synchronous, testable functions.
- Introduce small internal policy types instead of cross-cutting booleans: `WatchSupervisor`, `NotificationDispatcher`, `SkillSourceMaterializer`, `SkillFilePolicy`, and `SkillInstallPlanner`.
- This is a boundary cleanup, not a rewrite. Existing crates are sufficient for the orchestration work.

## Alternatives Considered

| Category | Recommended | Alternative | Why Not |
|----------|-------------|-------------|---------|
| File watching | `notify` `8.x` with explicit `PollWatcher` fallback | `watchexec`, `notify-debouncer-*`, or a separate watcher stack | Sumac already has the correct primitive in-tree. Another watcher layer would add churn before the current `notify` usage is fixed. |
| Git source materialization | System `git` + `tempfile` staging | `git2` / libgit2 | `git2` adds native dependency and behavior complexity without addressing the real bugs: wrong clone root handling, missing allowlist enforcement, and cleanup discipline. |
| Recursive skill traversal | Shared `walkdir` utility or disciplined std recursion | Hand-rolled recursion per module | Current hand-rolled recursion is already inconsistent. If Sumac wants one additive crate, this is the right one. |
| Command architecture | Existing `src/app/*` service pattern | DI framework / command bus / wholesale rewrite | v1.1 needs narrower seams, not another architecture layer. |

## What NOT to Add

| Avoid | Why | Use Instead |
|-------|-----|-------------|
| `git2` for v1.1 | It increases native and behavioral complexity while the current failures are lifecycle bugs around tempdirs, repo-root resolution, and copy policy | Keep system `git`, but stage in `TempDir`, validate, then atomically install |
| `.gitignore`-style filtering as a security boundary | Ignore rules express developer convenience, not what Sumac is safe to expose over MCP | Enforce an explicit allowlist owned by Sumac |
| A new async/runtime/web framework | Sumac already standardizes on Tokio, Axum, Reqwest, and Clap | Refactor into more `src/app/*` services and smaller domain modules |
| `TempDir::keep()` for transient clone paths | It intentionally disables automatic cleanup and is the direct shape of the current tempdir leak | Keep clone dirs scoped, and use `close()` when cleanup success must be observed |
| `notify` content hashing everywhere by default | It is materially more expensive and should be reserved for unreliable filesystems | Use normal event mode first, and targeted `compare_contents` only on fallback roots that need it |

## Stack Patterns by Variant

**If the watched roots are normal local directories:**
- Use `notify::RecommendedWatcher` for primary events.
- Coalesce bursts with a short Tokio debounce window.
- Rebuild once per burst and update the in-memory server from that single build result.

**If the watched roots are unreliable, remote, or event delivery fails:**
- Fall back to `notify::PollWatcher`.
- Set an explicit poll interval.
- Use `compare_contents` only when metadata-based polling is not trustworthy.

**If the code path installs or updates a skill from git or a local source:**
- Materialize into `TempDir`.
- Enumerate a canonical allowlist.
- Reject symlinks and hidden/VCS/build artifacts before copy.
- Scan staged files before final `rename`.

**If the code path is a new command-family migration:**
- Parse args in `main.rs`.
- Convert to a request struct.
- Run a focused service under `src/app/`.
- Keep rollback seams only where release-soak evidence still justifies them.

## Version Compatibility

| Package A | Compatible With | Notes |
|-----------|-----------------|-------|
| `tokio 1.x` | `reqwest` current | `reqwest` timeouts rely on a Tokio runtime with timers enabled, which Sumac already has. |
| `notify 8.x` | `tokio 1.x` | The watcher callback can bridge into Tokio channels; no runtime change is needed. |
| `walkdir 2.x` | Current Rust CLI code | Pure traversal utility; if added, keep it behind one shared internal file-policy module rather than scattered direct use. |
| `tempfile 3.x` | Current install pipeline | Safe fit for atomic staging as long as transient paths are not persisted with `keep()`. |

## Sources

- Local codebase: `/Users/hprincivil/Projects/sxmc/.planning/PROJECT.md` — milestone scope and constraints
- Local codebase: `/Users/hprincivil/Projects/sxmc/.planning/codebase/CONCERNS.md` — current bugs and risk surfaces
- Local codebase: `/Users/hprincivil/Projects/sxmc/.planning/codebase/STACK.md` — existing runtime/dependency baseline
- Local codebase: `/Users/hprincivil/Projects/sxmc/Cargo.toml` — current crate graph used by Sumac
- Local codebase: `/Users/hprincivil/Projects/sxmc/src/skills/install.rs` — current git install/tempdir flow
- Local codebase: `/Users/hprincivil/Projects/sxmc/src/server/mod.rs` — current watch/event/poll implementation
- Local codebase: `/Users/hprincivil/Projects/sxmc/src/security/skill_scanner.rs` — current UTF-8-only scanner behavior
- https://docs.rs/notify/latest/notify/struct.Config.html — verified `with_poll_interval`, `with_compare_contents`, and `with_follow_symlinks`
- https://docs.rs/notify/latest/notify/poll/struct.PollWatcher.html — verified first-party polling watcher support
- https://docs.rs/tokio/latest/tokio/process/struct.Command.html — verified async subprocess execution and `kill_on_drop`
- https://docs.rs/tokio/latest/tokio/time/fn.timeout.html — verified cancellation/deadline behavior
- https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html — verified task-set fan-out and abort-on-drop behavior
- https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html — verified request/connect/read timeout support
- https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html — verified `TempDir` cleanup, `keep`, and `close`
- https://doc.rust-lang.org/std/fs/fn.symlink_metadata.html — verified non-following symlink metadata checks
- https://docs.rs/walkdir/latest/walkdir/struct.WalkDir.html — verified recursive traversal options for the optional shared walker

---
*Stack research for: Sumac v1.1 Platform Hardening and Core Expansion*
*Researched: 2026-04-04*
