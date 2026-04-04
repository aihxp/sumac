# Codebase Concerns

**Analysis Date:** 2026-04-04

## Tech Debt

**Monolithic CLI orchestration:**
- Issue: command parsing, dispatch, network/file helpers, watch logic, bundle publishing, and many subcommand implementations live in one file.
- Files: `src/main.rs`, `src/cli_args.rs`, `src/command_handlers.rs`
- Impact: changes to unrelated commands collide in the same hotspots, reviews are expensive, and regression isolation is poor because behavior is concentrated instead of module-scoped.
- Fix approach: split `src/main.rs` by command family (`inspect`, `watch`, `skills`, `bake`, `doctor`, `sync`) and move subcommand-specific helpers beside their domain modules.

**Skill install pipeline is over-coupled:**
- Issue: source resolution, git clone, temp staging, recursive copying, and metadata persistence are handled together in `src/skills/install.rs`.
- Files: `src/skills/install.rs`
- Impact: bugs in one step are hard to test in isolation, and install-time security rules are difficult to enforce because there is no narrow “allowed file set” abstraction.
- Fix approach: separate source materialization, source validation, file selection, and target install into distinct functions with focused tests.

## Known Bugs

**Git-based skill installs resolve the wrong directory and leak temp directories:**
- Symptoms: installs from git URLs or GitHub tree URLs can fail with “does not contain SKILL.md”, and successful clones leave persistent temp directories behind.
- Files: `src/skills/install.rs`
- Trigger: `materialize_source_dir()` clones into `temp.path().join("repo")`, then resolves from `temp.keep()` instead of `repo_dir`, so repo-root and subpath resolution both point at the wrong base. `temp.keep()` also prevents automatic cleanup.
- Workaround: install from a local path instead of a git source.

**Poll-based watch can miss nested skill asset changes:**
- Symptoms: edits inside nested paths under a skill’s `scripts/` or `references/` tree may not trigger reloads when the server falls back to polling.
- Files: `src/server/mod.rs`, `src/skills/parser.rs`
- Trigger: `compute_skill_fingerprint()` only hashes one directory level via `hash_directory_files()`, while nested files are otherwise reachable through skill file APIs.
- Workaround: restart the server after nested asset changes, or rely on filesystem-event mode where available.

## Security Considerations

**Installed skills expose more files than the resource model suggests:**
- Risk: arbitrary files inside a skill directory become listable and readable over MCP, not just `SKILL.md`, `references/`, or declared script files.
- Files: `src/skills/install.rs`, `src/server/handler.rs`
- Current mitigation: `resolve_skill_file_path()` prevents path traversal outside the skill base directory.
- Recommendations: restrict install copies to an allowlist (`SKILL.md`, `scripts/`, `references/`, optional manifest), and make `get_skill_related_file()` deny dotfiles, VCS directories, metadata files, and other non-resource content by default.

**Remote or local skills can copy unexpected files into readable skill roots:**
- Risk: `copy_dir_recursive()` copies the full source tree, so `.git`, build outputs, config files, and symlinked content can be pulled into the installed skill and then exposed through `get_skill_related_file()`.
- Files: `src/skills/install.rs`, `src/server/handler.rs`
- Current mitigation: `ensure_skill_dir()` only checks that `SKILL.md` exists.
- Recommendations: reject symlinks during install, ignore hidden/VCS/build directories, and validate the installed tree before serving it.

**Security scanning silently skips non-UTF-8 and unreadable files:**
- Risk: malicious binary, encoded, or unreadable script/reference files can bypass `sxmc scan` and still be installed or served.
- Files: `src/security/skill_scanner.rs`
- Current mitigation: text files are scanned against regex-based secret, injection, and dangerous-command patterns.
- Recommendations: surface skipped files as findings, scan raw bytes where possible, and fail closed for unreadable script/reference content during install or scan.

## Performance Bottlenecks

**Skill reload work is duplicated on every filesystem-triggered refresh:**
- Problem: the watch reload path parses and rebuilds the server twice per change.
- Files: `src/server/mod.rs`
- Cause: `reload_skills_from_watch()` calls `summarize_inputs_with_manifests()` which internally calls `build_server()`, then immediately calls `build_server()` again for the real replacement.
- Improvement path: build once, derive the inventory summary from the already-built server, then swap it in.

**`watch` mixes blocking work into async command handling:**
- Problem: the watch loop can stall other Tokio work and delay notifications or shutdown.
- Files: `src/main.rs`
- Cause: the async `main()` path uses `std::thread::sleep(interval)` inside `Commands::Watch`, and `run_watch_notify_command()` uses blocking `child.status()` instead of async subprocess handling.
- Improvement path: replace blocking sleeps with `tokio::time::sleep`, move notify command execution to async process handling, and avoid long synchronous sections in the loop.

**Webhook notifications can hang the watch loop indefinitely:**
- Problem: a slow or dead webhook endpoint can block status polling and subsequent notifications.
- Files: `src/main.rs`
- Cause: `send_watch_webhook()` uses `reqwest::Client::new()` without a timeout, and webhooks are awaited sequentially inside the main watch loop.
- Improvement path: set explicit timeouts, fan out webhook sends concurrently with bounded concurrency, and downgrade webhook failures to isolated event errors instead of loop-wide stalls.

## Fragile Areas

**Skill file handling is inconsistent across parsing, serving, and watching:**
- Files: `src/skills/parser.rs`, `src/server/handler.rs`, `src/server/mod.rs`
- Why fragile: parsing only indexes top-level `scripts/` and `references/`, file listing is recursive, and poll-based change detection only fingerprints one directory level. Nested assets therefore behave differently depending on which subsystem touches them.
- Safe modification: define one canonical skill file model and reuse it for install, discovery, serving, and watch invalidation.
- Test coverage: gaps remain around nested directory behavior; current tests cover recursive file listing and basic fingerprint change detection, but not the full cross-module behavior.

**Cache maintenance failures are mostly silent:**
- Files: `src/cache.rs`, `src/cli_surfaces/inspect.rs`
- Why fragile: cache writes, removals, and cleanup paths frequently ignore filesystem errors, so stale or corrupt cache state can linger without a surfaced failure.
- Safe modification: make cache mutation APIs return structured warnings or errors, and have CLI commands report partial cleanup failures.
- Test coverage: unit tests cover happy-path cache behavior, but not permission errors, corrupted files, or partial cleanup failures.

## Scaling Limits

**Watch/status processing scales with full recomputation, not incremental diffing:**
- Current capacity: acceptable for the current small test corpus; no repo-local evidence of optimization for large numbers of skills, host artifacts, or webhook targets.
- Limit: `Commands::Watch` recomputes full status on every interval and performs notifications serially, so latency grows with host count and external notification latency.
- Scaling path: cache stable status inputs, diff host state incrementally, and decouple notifications from the main polling loop with a queue or background worker.

## Dependencies at Risk

**High-risk dependency issue not detected from repo-local evidence:**
- Risk: no single package currently stands out as abandoned or already breaking the build; `Cargo.lock` is present and `cargo test --locked` passes.
- Impact: dependency risk exists mainly through feature breadth rather than one clearly failing package.
- Migration plan: keep concern tracking focused on integration boundaries (`rmcp`, `reqwest`, `notify`) and add compatibility tests before major upgrades.

## Missing Critical Features

**No install-time allowlist for served skill contents:**
- Problem: the system lacks a first-class rule for which files are safe to persist and expose from installed skills.
- Blocks: secure remote skill installs and predictable MCP exposure for third-party skill sources.

**No end-to-end validation for git-backed skill installs:**
- Problem: tests do not exercise the actual clone-materialize-install path for git or GitHub tree sources.
- Blocks: reliable regression detection for the current install-path bug in `src/skills/install.rs`.

## Test Coverage Gaps

**Git skill installation path is under-tested:**
- What's not tested: cloning, repo-root vs subpath resolution, tempdir cleanup, and install behavior for real git-backed sources.
- Files: `src/skills/install.rs`
- Risk: the current path-resolution bug survived because tests only cover URL parsing and metadata round-tripping, not the end-to-end install flow.
- Priority: High

**Watch loop reliability under slow notifications is untested:**
- What's not tested: slow or hanging `notify_command` processes, webhook timeouts, and event backlog behavior.
- Files: `src/main.rs`
- Risk: watch can appear healthy in tests while still hanging in production-facing automation.
- Priority: Medium

**Scanner behavior for binary/non-UTF-8 skill assets is untested:**
- What's not tested: unreadable files, binary scripts, and symlink-heavy skill trees.
- Files: `src/security/skill_scanner.rs`, `src/skills/install.rs`
- Risk: security scans can report clean results while skipping exactly the files most likely to hide malicious content.
- Priority: High

---

*Concerns audit: 2026-04-04*
