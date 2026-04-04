# Domain Pitfalls

**Domain:** Hardening a shipped Rust CLI/MCP tool without breaking `1.x` contracts
**Researched:** 2026-04-04
**Overall confidence:** HIGH for Sumac-specific pitfalls from repo context and code paths; HIGH for watcher/tempdir/process/archive constraints from official Rust crate docs; MEDIUM for rollout heuristics inferred from current shipped-CLI practice

## Critical Pitfalls

### Pitfall 1: Treating watch hardening as a backend swap instead of a contract problem

**What goes wrong:**
Watch appears fixed on one machine but still misses real edits in production. Typical failures are nested skill assets not triggering reloads in polling mode, rename-on-save editor behavior not matching expected event kinds, deleted/recreated directories going unnoticed, and large trees hitting backend limits or dropping events.

**Why it happens:**
Teams trust filesystem events too literally or only harden the `notify` path. The current Sumac code already shows split behavior: `run_notify_watch_loop()` is recursive event-driven, while `compute_skill_fingerprint()` only hashes one directory level under `scripts/` and `references/`. Upstream `notify` docs also explicitly warn that editor save behavior differs, parent-folder deletion requires watching the parent, large directories can miss events, and Linux watcher limits are real.

**How to avoid:**
- Define one canonical watched-input model and reuse it for install, parse, serve, and poll fingerprinting.
- Make parity a requirement between event mode and poll fallback; both must detect the same nested asset set.
- Debounce and coalesce reloads, but validate rename-on-save, delete/recreate, and nested-directory edits explicitly.
- Add fallback logic for backend limits and pseudo-filesystem cases instead of assuming the recommended watcher is sufficient everywhere.
- Treat watch behavior as a public reliability contract for automation, not an implementation detail.

**Warning signs:**
- Poll fallback only catches top-level file edits.
- `serve --watch` passes simple tests but misses changes under nested `scripts/` or `references/`.
- Bug reports differ by editor, OS, or container environment.
- Linux users hit inotify limit errors or report "watch is flaky on big trees."

**Phase to address:**
Phase 1: Canonical skill file model and watch parity harness.

---

### Pitfall 2: Leaving blocking or unbounded side effects inside the watch loop

**What goes wrong:**
`watch` stops feeling live under load. A slow notify command, hanging webhook, or expensive rebuild delays subsequent checks, causes missed state transitions, or makes shutdown and cancellation lag badly.

**Why it happens:**
The current watch path runs side effects inline. `run_watch_notify_command()` uses blocking `status()`, and `send_watch_webhook()` creates a fresh `reqwest::Client` with no timeout and posts sequentially. This is the classic mistake of making the loop "feature complete" before isolating backpressure.

**How to avoid:**
- Move notify-command execution to async process handling or a bounded worker queue.
- Reuse an HTTP client with explicit connect/request timeouts.
- Run webhook fan-out with bounded concurrency and isolate failures from the main watch loop.
- Measure and cap reload/notification latency as part of the watch contract.
- Keep the watch loop responsible for detection and state transition only; delivery should be downstream work.

**Warning signs:**
- One slow webhook delays all later notifications.
- `watch` appears idle while a child process or network request is still running.
- Ctrl-C or cancellation takes much longer than the polling interval.
- Reliability fixes add more side effects directly to the loop body.

**Phase to address:**
Phase 2: Watch runtime hardening and notification isolation.

---

### Pitfall 3: Materializing untrusted skill sources before deciding what is safe to keep

**What goes wrong:**
Remote installs succeed, but the installed skill root contains `.git`, build outputs, hidden files, credential material, or symlinked content that later becomes readable through MCP. The install path "works" but silently widens the readable surface.

**Why it happens:**
The current pipeline clones or resolves a source, recursively copies the whole tree, and only checks that `SKILL.md` exists. `copy_dir_recursive()` copies every regular file it sees, while serving only checks that a requested file stays inside the canonicalized base directory. That is a traversal guard, not an allowlist.

**How to avoid:**
- Split skill install into four stages: materialize source, validate source, select allowed files, install selected files.
- Persist only an explicit allowlist such as `SKILL.md`, `scripts/`, `references/`, and a tightly scoped manifest if needed.
- Reject symlinks, VCS directories, hidden metadata, and build artifacts during install.
- Validate the final installed tree before metadata is written or the skill is served.
- Make the served file set a strict subset of the installed allowlist, not an independent recursive walk.

**Warning signs:**
- Installed skill directories contain `.git`, `.env`, temp outputs, or files that are not part of the resource model.
- Security reviews rely on "path stays inside root" as the main protection.
- A local path and a git source produce different persisted trees for the same logical skill.
- The team discusses filtering at serve time instead of before persistence.

**Phase to address:**
Phase 3: Secure skill materialization and install allowlist enforcement.

---

### Pitfall 4: Fixing the git install bug by patching symptoms instead of the lifecycle boundary

**What goes wrong:**
The immediate "missing `SKILL.md`" bug gets fixed, but tempdir leakage, repo-root/subpath confusion, reference handling drift, and cleanup failures remain. The system keeps accumulating one-off source-resolution rules that are hard to test and easy to break again.

**Why it happens:**
`materialize_source_dir()` currently clones into `temp.path().join("repo")` and then resolves from `temp.keep()` rather than the cloned repo directory, which is exactly the kind of lifecycle confusion that survives when source materialization and install semantics are tangled together. Official `tempfile` docs also make clear that `TempDir::keep()` disables automatic deletion; using it casually turns staging into persistent state.

**How to avoid:**
- Keep staging ephemeral until validation passes; only the final installed tree should persist.
- Return an explicit staged source object that owns cleanup behavior instead of leaking raw paths.
- Resolve repo subpaths relative to the actual clone root, not the temp container directory.
- Prefer `TempDir::close()` when cleanup success matters, and test cleanup failures explicitly.
- Add end-to-end tests for local path, git URL, and GitHub tree URL flows with repo-root, subpath, and reference variants.

**Warning signs:**
- Temp directories accumulate after installs or updates.
- Fixes mention "just use the kept temp path" or "just canonicalize later."
- GitHub tree URLs and raw repo URLs behave differently for the same skill location.
- Cleanup errors are ignored because install already "succeeded."

**Phase to address:**
Phase 3: Source materialization boundary cleanup and git install regression coverage.

---

### Pitfall 5: Letting scanner policy, install policy, and serve policy diverge

**What goes wrong:**
`sxmc scan` reports a skill as clean while install persists files the scanner skipped and MCP later serves them. This creates the worst possible security posture: apparent safety with real blind spots.

**Why it happens:**
The codebase already shows separate models: parser indexes only some paths, serving walks recursively, and scanner coverage can silently skip non-UTF-8 or unreadable files. If each subsystem keeps its own notion of "skill files," every hardening pass leaves gaps at the boundaries.

**How to avoid:**
- Create one canonical inventory of skill files with per-file classification: allowed, denied, executable, reference, metadata, skipped-with-error.
- Make scanner results authoritative for what may be installed and served.
- Fail closed for unreadable or non-decodable files under allowed roots unless explicitly supported and scanned by bytes.
- Surface skipped files as findings, not debug noise.
- Add a single invariant test: `served_files == scanned_allowed_files == installed_allowed_files`.

**Warning signs:**
- Scan results do not list skipped files.
- Security hardening PRs touch install or serve but not scanner logic.
- A file can be requested over MCP even though it never appears in scan output.
- Binary or unreadable files are logged and ignored instead of blocked.

**Phase to address:**
Phase 4: Unified scan/serve policy and fail-closed enforcement.

---

### Pitfall 6: Preserving command names while changing `1.x` semantics

**What goes wrong:**
The refactor is described as "internal only," but scripts break because stdout/stderr separation changed, exit codes moved, JSON fields reordered or disappeared, default scope changed, or generated files are rewritten differently.

**Why it happens:**
Shipped CLIs rarely have a public API limited to flags. SemVer for `1.x` only helps if the public API is explicitly treated as behavior, not just parser shape. In Sumac, the risk is concentrated in moving orchestration out of `src/main.rs` while keeping command families stable.

**How to avoid:**
- Freeze a contract matrix for stable commands: stdout, stderr, exit codes, JSON shape, generated file edits, aliases, env toggles, and recovery messages.
- Add characterization tests before refactoring, then run old/new implementations side by side on the same fixtures.
- Treat output ordering as contractual when users diff or parse it.
- Keep changes additive in `1.x`; if a behavior must change, deprecate it explicitly and document it.

**Warning signs:**
- PRs say "no user-facing changes" while touching renderers, defaults, or generated files.
- Tests only assert `.success()` or loose substring matches.
- Refactor branches remove existing flags or aliases because "nobody should be using that."
- The new code path passes unit tests but lacks output/file snapshots.

**Phase to address:**
Phase 5: Command-family refactor with contract harness and parity diffing.

---

### Pitfall 7: Removing the rollback seam before release-soak evidence exists

**What goes wrong:**
The old route is deleted because the new path feels cleaner, then field regressions show up in packaging, wrappers, or long-tail environments with no cheap escape hatch.

**Why it happens:**
Cleanup pressure makes temporary seams feel like technical debt to delete early. But in a shipped CLI, rollback is part of migration safety, not just dead code management.

**How to avoid:**
- Define retirement criteria up front: parity results, release soak length, support incidents, and cross-platform/package validation.
- Keep the seam narrow and explicit so rollback remains cheap and observable.
- Remove it only after evidence says the old path is unnecessary, not because the new architecture is nicer.
- Capture route-selection telemetry or structured debug output during soak to make mismatch diagnosis fast.

**Warning signs:**
- The rollback seam is described as embarrassing or temporary without a deletion gate.
- Compatibility bugs are still being found while seam-removal work is underway.
- There is no documented way to reproduce old-vs-new behavior on the same input.
- Packaging and wrapper paths have not been soaked on the new route yet.

**Phase to address:**
Phase 6: Release soak review and rollback-seam retirement decision.

---

### Pitfall 8: Cross-platform path and shell assumptions sneaking into hardening work

**What goes wrong:**
A fix works on macOS/Linux but breaks on Windows or in packaged environments. Typical cases are relative command paths resolving differently under changed working directories, shell quoting behaving differently across `sh -lc` and `cmd /C`, and path canonicalization crossing unexpected boundaries.

**Why it happens:**
Hardening work often adds subprocesses, temp paths, canonicalization, and recursive walking. Official `std::process::Command` docs warn that relative program paths with `current_dir()` are platform-specific, and Sumac already spans wrappers, shells, and multiple OSes.

**How to avoid:**
- Normalize subprocess invocation through explicit program + args where possible instead of shell strings.
- If shell commands remain supported, treat them as a compatibility feature with OS-specific tests.
- Use absolute program paths when changing working directories.
- Keep Windows/macOS/Linux coverage in the definition of done for watch, install, and serve flows.

**Warning signs:**
- New code introduces more `sh -lc` or `cmd /C` without argument-level alternatives.
- A refactor changes `current_dir()` behavior around subprocesses.
- CI coverage for packaging or Windows is treated as optional for internal refactors.
- Bugs reproduce only through npm/Homebrew wrappers or on one OS family.

**Phase to address:**
Phase 2 for watch-side subprocesses; Phase 5 for broader orchestration refactors.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Keep separate file models for parser, watcher, scanner, and server | Faster local fixes in one subsystem | Guarantees recurring parity bugs and security gaps | Never for v1.1; this milestone needs a canonical model |
| Filter dangerous files only at serve time | Smaller code diff in install pipeline | Unsafe content is still persisted, scanned inconsistently, and harder to reason about | Never |
| Patch the git install bug in place without splitting materialize/validate/install | Quick bug closure | Reintroduces tempdir, cleanup, and subpath bugs later | Only as an emergency patch ahead of the real boundary split |
| Leave notify commands and webhooks inline in the watch loop | Easier implementation | Backpressure, hangs, and poor cancellation | Never for long-running automation |
| Remove the rollback seam as soon as the new path passes tests | Cleaner codebase | Expensive recovery when field regressions appear | Only after explicit soak criteria are met |
| Use loose integration assertions like `.success()` for stable commands | Faster tests | Misses contract drift in JSON, stderr, file edits, and exit codes | Never for shipped `1.x` surfaces |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Filesystem backends via `notify` | Assuming one backend's event stream defines the contract | Define backend-independent watch semantics and prove poll/event parity |
| Editor save behavior | Assuming `Write` always means "same file changed in place" | Test rename-on-save, truncate/write, delete/recreate, and parent-dir deletion patterns |
| Git skill sources | Assuming the cloned temp root is the skill root | Resolve subpaths from the actual clone directory and validate before persistence |
| MCP skill file serving | Treating traversal prevention as equivalent to file allowlisting | Serve only from the post-validation allowlist and deny everything else by default |
| Security scanning | Reporting only findings, not skipped/unreadable files | Make skipped files explicit and fail closed for allowed roots |
| Watch notification commands | Running shell strings inline with inherited environment and no timeout | Prefer program+args forms, isolate execution, and bound runtime |
| Webhooks | Posting sequentially with fresh clients and no timeout | Reuse a client, add timeouts, and isolate delivery failures from the watch loop |
| Cross-platform wrappers | Validating only the Rust binary path | Exercise npm/Homebrew/smoke paths for any refactor that changes subprocesses or filesystem behavior |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Rebuilding the skill server twice per watch-triggered reload | Watch feels slower than expected; CPU spikes during rapid edits | Build once, derive summary from the built result, then swap atomically | Noticeable during frequent edits or larger skill sets |
| Full recursive rescans on every poll tick without a canonical incremental model | Poll mode burns CPU and still misses some nested changes | Keep an explicit file inventory and hash the real allowed tree incrementally | Breaks first on repos with many skills or deep references |
| Sequential notification delivery in the main loop | State changes queue behind slow webhooks or child commands | Decouple detection from delivery with bounded async workers | Breaks as soon as one integration is slow |
| Watching very large trees with raw recursive watchers only | Missed events, inotify-limit errors, or unstable memory/file-descriptor usage | Scope watched roots carefully, surface backend limits, and keep a poll fallback strategy | Large skill trees, monorepos, or heavily nested workspaces |

## Security Mistakes

| Mistake | Risk | Prevention |
|---------|------|------------|
| Blind recursive copy of remote/local skill trees | Secrets, VCS metadata, or junk files become installed and possibly served | Persist only an allowlisted subset after validation |
| Allowing symlinks or root-link traversal into installed skills | Reads or serves files outside the intended trust boundary | Reject symlinks during install and validate canonical paths before persistence |
| Serving any file under the skill root | Path-traversal defense exists, but sensitive in-root files are still exposed | Make serving strictly allowlist-based |
| Silently skipping unreadable or non-UTF-8 files during scanning | Malicious or risky files bypass scan while remaining installed/served | Surface skips as findings and fail closed |
| Keeping temp staging directories around after install | Sensitive transient contents linger on disk and confuse future updates | Use ephemeral staging with explicit cleanup verification |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Watch says it is active but silently misses nested edits in poll mode | Users do extra restarts and stop trusting automation | Make watch mode report backend/fallback behavior clearly and prove nested parity |
| Install failures collapse to "does not contain SKILL.md" for git sources | Users cannot tell whether the URL, subpath, ref, or clone root is wrong | Surface source-resolution stage, clone root, and checked subpath in the error |
| Scan output says "clean" without mentioning skipped files | Users assume a stronger security guarantee than exists | Print explicit skipped-file findings and install blocks |
| Refactors preserve `--help` but change machine-facing output | Scripts break in ways that feel random or undocumented | Preserve and diff stdout/stderr/JSON/file contracts as first-class behavior |

## "Looks Done But Isn't" Checklist

- [ ] **Watch hardening:** Verified nested `scripts/` and `references/` edits in both notify mode and poll fallback.
- [ ] **Watch hardening:** Verified rename-on-save, delete/recreate, parent-directory deletion, and backend-limit behavior.
- [ ] **Watch notifications:** Verified slow command and slow webhook behavior does not stall later state detection.
- [ ] **Git skill install:** Verified repo root, GitHub tree subpath, explicit ref, and cleanup behavior end to end.
- [ ] **Skill security:** Verified installed files, scanned files, and served files are the same allowlisted set.
- [ ] **Skill security:** Verified symlinks, hidden files, unreadable files, and binary payloads are rejected or surfaced as findings.
- [ ] **Refactor parity:** Verified stdout, stderr, exit codes, JSON shape, aliases, and generated file diffs against the old path.
- [ ] **Rollback readiness:** Verified the rollback seam still works until soak criteria are met.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Watch parity bugs after rollout | MEDIUM | Re-enable known-good route or backend, capture missed-path cases, add parity fixtures, and only re-promote after poll/event equivalence is proven |
| Slow or hanging watch notifications | LOW | Disable delivery side effects behind flags, add timeouts/queueing, and keep core detection running |
| Unsafe installed skill contents | HIGH | Stop serving affected skills, purge installed trees, tighten allowlist, rescan, and require reinstall from validated sources |
| Git materialization/tempdir regressions | MEDIUM | Disable affected source flavor if necessary, patch lifecycle boundary, clean leaked staging dirs, and ship end-to-end coverage before reopening |
| Refactor contract drift | HIGH | Route traffic back through the rollback seam, diff old/new outputs on captured fixtures, then patch the specific semantic mismatch |
| Scanner/serve policy mismatch | HIGH | Treat as a security issue: deny serving unclassified files immediately, block installs until policy is unified, and notify users if exposure occurred |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Watch contract differs between notify and poll backends | Phase 1: Canonical skill file model and watch parity harness | Same nested-edit fixture passes in both backends |
| Blocking notification side effects stall watch | Phase 2: Watch runtime hardening and notification isolation | Slow command/webhook tests show state detection continues and cancellation remains prompt |
| Untrusted skill sources are persisted before validation | Phase 3: Secure skill materialization and install allowlist enforcement | Installed tree contains only allowlisted files for local, git, and GitHub-tree installs |
| Git install bug is fixed without fixing lifecycle ownership | Phase 3: Source materialization boundary cleanup and git regression coverage | End-to-end tests cover repo root, subpath, ref, and temp cleanup |
| Scanner, install, and serve policies diverge | Phase 4: Unified scan/serve policy and fail-closed enforcement | `served_files == scanned_allowed_files == installed_allowed_files` |
| Internal refactor changes `1.x` behavior | Phase 5: Command-family refactor with contract harness and parity diffing | Old/new diff suite passes for stable commands and generated artifacts |
| Rollback seam is removed too early | Phase 6: Release soak review and rollback-seam retirement decision | Retirement checklist shows parity, soak duration, packaging pass, and low incident rate |
| Cross-platform subprocess/path behavior drifts | Phase 2 and Phase 5 | Windows/macOS/Linux smoke and wrapper paths pass with the new route |

## Sources

- Repo context:
  - `/Users/hprincivil/Projects/sxmc/.planning/PROJECT.md`
  - `/Users/hprincivil/Projects/sxmc/.planning/codebase/CONCERNS.md`
  - `/Users/hprincivil/Projects/sxmc/.planning/codebase/TESTING.md`
  - `/Users/hprincivil/Projects/sxmc/src/skills/install.rs`
  - `/Users/hprincivil/Projects/sxmc/src/server/mod.rs`
  - `/Users/hprincivil/Projects/sxmc/src/server/handler.rs`
  - `/Users/hprincivil/Projects/sxmc/src/main.rs`
- Official docs:
  - `notify` crate docs, "Known Problems" and watcher guidance: https://docs.rs/notify/latest/notify/
  - `notify::poll::PollWatcher` docs: https://docs.rs/notify/latest/notify/poll/struct.PollWatcher.html
  - `tempfile::TempDir` docs, especially `keep()` and `close()`: https://docs.rs/tempfile/latest/tempfile/struct.TempDir.html
  - Rust `std::process::Command` docs, especially argument handling, env inheritance, and `current_dir()` caveats: https://doc.rust-lang.org/std/process/struct.Command.html
  - Tokio `sleep` docs for non-blocking timing semantics: https://docs.rs/tokio/latest/tokio/time/fn.sleep.html
  - `walkdir::WalkDir` docs for recursion, sorting, and symlink behavior: https://docs.rs/walkdir/latest/walkdir/struct.WalkDir.html
  - Semantic Versioning 2.0.0: https://semver.org/

---
*Pitfalls research for: v1.1 Platform Hardening and Core Expansion*
*Researched: 2026-04-04*
