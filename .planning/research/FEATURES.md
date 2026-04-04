# Feature Landscape

**Domain:** `v1.1 Platform Hardening and Core Expansion` for a mature `1.x` CLI/MCP tool
**Researched:** 2026-04-04
**Confidence:** HIGH

## Feature Landscape

### Table Stakes (Users Expect These)

These are the hardening features mature CLI tooling is expected to have when it already ships watch, install, and extension-like behavior. Missing them makes the milestone feel unsafe rather than incomplete.

| Feature | Why Expected | Complexity | Dependencies | Notes |
|---------|--------------|------------|--------------|-------|
| Watch: settled, nested-aware change detection | Mature watchers consolidate bursts and wait for the tree to settle before acting; they also recurse consistently instead of missing nested assets. | HIGH | Canonical skill file model; event coalescing; reload tests | This is the direct fix for Sumac's known poll-mode blind spot on nested `scripts/` and `references/` content. The same file model should drive parsing, serving, and watch invalidation. |
| Watch: degraded-mode parity for native events and polling | Mature tooling treats polling as a supported fallback, not a second-class mode with different correctness. | HIGH | Watch backend abstraction; rescan/rebuild path; diagnostics surface | When native backends overflow or are unavailable, behavior should degrade to polling with explicit signaling, not silent misses. Treat "loss of sync" as a first-class event that triggers a bounded rescan. |
| Watch: bounded side effects for webhooks, notify commands, and long-running automation | Long-running watch commands are expected to stay responsive even when children or webhooks are slow. | MEDIUM | Async child execution; timeout policy; concurrency limits | Notify paths must not stall the main loop. Timeouts, cancellation, and isolated failures are expected behavior in mature watch workflows. |
| Skills: allowlisted install payload instead of recursive tree copy | Mature plugin managers install a declared payload, not an arbitrary repository tree. | HIGH | Canonical skill file model; install planner; serve-surface contract | For this milestone, the allowlist should be narrow and predictable: `SKILL.md`, declared scripts, `references/`, and a small set of explicit metadata files. Reject hidden/VCS/build output by default. |
| Skills: staged validation before activation or serving | Mature extension systems validate downloaded content before exposing it to execution or discovery surfaces. | HIGH | Temp staging; atomic replace; scanner integration | Install and update should materialize into a staging area, validate there, then commit atomically. A failed validation must leave the existing installed copy intact. |
| Skills: reliable git/remote install and update semantics | Existing users expect git-backed installs to resolve the right source, pin cleanly, and clean up after themselves. | HIGH | Source normalization; tempdir lifecycle; install metadata | This is net-new hardening over already shipped flows, not a new product surface. Capture origin URL, ref or commit, resolved subpath, and installed file inventory so updates are reproducible. |
| Security: fail-closed scan coverage for served assets | Mature tooling does not silently skip unreadable or non-UTF-8 content that can still be exposed or executed later. | HIGH | Byte-level scanner behavior; install-time policy; reporting | If a file is in the allowed serve surface, scan coverage for that file must be explicit. "Skipped" should be surfaced as a blocking or policy-driven finding, not a quiet omission. |
| Orchestration: module-scoped watch and skills command families with parity gates | Mature CLIs keep core contracts stable while moving internal command families behind explicit seams and tests. | HIGH | Command-family service boundaries; CLI contract fixtures; exit-code and JSON tests | For Sumac, this means moving more behavior out of top-level dispatch hotspots without changing `1.x` CLI behavior. Watch and skills are the right slices because they currently combine the most reliability and security risk. |

### Differentiators (Competitive Advantage)

These are not required for the milestone to be credible, but they materially improve operator trust and make Sumac's hardening better than a basic cleanup pass.

| Feature | Value Proposition | Complexity | Dependencies | Notes |
|---------|-------------------|------------|--------------|-------|
| Watch health and degraded-mode diagnostics | Operators can see whether Sumac is on native events or polling, when the last successful reload happened, and why a full rescan occurred. | MEDIUM | Degraded-mode parity; structured status events | Mature watch tools expose when they are stressed. This reduces "watch seems flaky" support churn and makes long-running automation debuggable. |
| Skill trust tiers or policy file | Lets teams restrict what can be installed or served: local-only, allowlisted git hosts, pinned refs only, or curated indexes. | HIGH | Source metadata capture; allowlisted payload; staged validation | This maps well to enterprise extension policy patterns without forcing a full marketplace build in v1.1. |
| Install preview and provenance report | Shows what files will be installed and exposed, where the skill came from, what ref was used, and what the scanner found. | MEDIUM | Install planner; metadata capture; scanner reporting | The value is reviewability. Users can inspect the exact serve surface before activation instead of trusting an opaque install step. |
| Shadow routing or parity-diff mode for migrated command families | Allows maintainers to compare old and new watch or skills behavior before fully cutting traffic over. | HIGH | Module-scoped services; contract fixtures; diff harness | Strong differentiator for a brownfield rewrite because it turns "refactor confidence" into a measurable artifact. |
| Incremental reload classification | Distinguishes between metadata-only changes, nested asset changes, and install-surface changes so Sumac can avoid unnecessary full rebuilds. | MEDIUM | Canonical skill file model; settled event pipeline | This is a useful second step once correctness is fixed. It improves throughput without compromising reliability. |

### Anti-Features (Commonly Requested, Often Problematic)

These are tempting shortcuts or scope-expansion ideas that would weaken this milestone.

| Feature | Why Requested | Why Problematic | Alternative |
|---------|---------------|-----------------|-------------|
| Instant reaction to every raw filesystem event | Sounds "more real-time" and simpler than debouncing or settling. | Produces duplicate reloads, editor-save noise, and race conditions against partially written files. | Use event coalescing plus a short settle window and then reload from a canonical file inventory. |
| Recursive copy of full git or local skill trees | Feels convenient because it preserves "everything" from the source. | Pulls in `.git`, dotfiles, build outputs, secrets, and other files that Sumac may later expose over MCP. | Install from an explicit allowlist or manifest-like plan and validate the resulting tree before activation. |
| Warn-only scanning that still installs unreadable or binary served files | Keeps installs permissive and reduces short-term friction. | Quietly widens the trust boundary; the exact files most likely to hide malicious content bypass enforcement. | Fail closed for install-surface files unless policy explicitly allows the exception and reports it. |
| Unbounded watch retries, webhook waits, or child execution | Seems resilient because the system "keeps trying." | One bad endpoint or child process can stall the whole watch loop and create false confidence that automation is still healthy. | Use timeouts, bounded concurrency, cancellation, and per-event failure isolation. |
| Big-bang orchestration rewrite across all remaining commands | Appeals to maintainers who want to "clean it all up" in one pass. | Removes rollback safety, makes regressions ambiguous, and blocks releases behind one giant cutover. | Migrate watch and skills as separate command families behind explicit service seams and parity tests. |
| Auto-serving newly installed remote skills by default | Feels convenient because install and availability happen in one step. | Blurs trust boundaries between acquisition, validation, and exposure. | Separate install from activation or serve exposure, and make trust state explicit in metadata or policy. |

## Feature Dependencies

```text
Canonical skill file model
  -> Allowlisted install payload
  -> Predictable serve surface
  -> Nested-aware watch invalidation
  -> Incremental reload classification

Settled event pipeline
  -> Degraded-mode parity
  -> Bounded notify/webhook execution
  -> Reliable long-running watch automation

Git or remote source normalization
  -> Staged validation
  -> Atomic install or update
  -> Source provenance capture
  -> Trust tiers or policy file

Module-scoped watch and skills services
  -> CLI and JSON parity gates
  -> Shadow routing or parity diff
  -> Evidence-based rollback seam decision
```

### Dependency Notes

- **Canonical skill file model requires parser, installer, server, and watch alignment:** if each subsystem sees a different file set, Sumac will keep reintroducing mismatches between install, serve, and reload behavior.
- **Allowlisted install payload should come before broader trust policy:** a policy engine is not useful if the installed tree is still unconstrained.
- **Degraded-mode parity depends on a real rescan path:** polling fallback is only reliable if Sumac can rebuild state deterministically after event loss or backend failure.
- **Bounded side effects depend on the settled watch pipeline:** without coalesced events, timeout and concurrency logic still thrashes under noisy editors or bursty tree updates.
- **Shadow routing depends on module-scoped command seams:** it is hard to compare legacy and rewritten behavior when the code paths still collapse into the same top-level orchestration hotspot.

## MVP Definition

### Launch With (v1.1)

- [ ] Settled, nested-aware watch invalidation across the actual skill serve surface
- [ ] Polling/native parity with explicit degraded-mode signaling and bounded rescan behavior
- [ ] Async, timeout-bounded webhook and notify-command execution
- [ ] Allowlisted, staged skill install or update with atomic replace and temp cleanup
- [ ] Fail-closed scan coverage for all install-surface files that can later be served
- [ ] Watch and skills command-family seams with contract tests preserving `1.x` CLI, JSON, and exit-code behavior

### Add After Validation (v1.1.x)

- [ ] Watch health diagnostics and structured reload or rescan reporting
- [ ] Install preview and provenance report
- [ ] Incremental reload classification once correctness is stable

### Future Consideration (v1.2+)

- [ ] Team-level trust policy file for allowed origins, pinning rules, and activation restrictions
- [ ] Generalized parity-diff framework for additional command families beyond watch and skills

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
|---------|------------|---------------------|----------|
| Settled, nested-aware watch invalidation | HIGH | HIGH | P1 |
| Polling/native degraded-mode parity | HIGH | HIGH | P1 |
| Bounded notify/webhook execution | HIGH | MEDIUM | P1 |
| Allowlisted staged skill install/update | HIGH | HIGH | P1 |
| Fail-closed scan coverage for served assets | HIGH | HIGH | P1 |
| Watch and skills command-family seams with parity gates | HIGH | HIGH | P1 |
| Watch health diagnostics | MEDIUM | MEDIUM | P2 |
| Install preview and provenance report | MEDIUM | MEDIUM | P2 |
| Incremental reload classification | MEDIUM | MEDIUM | P2 |
| Skill trust tiers or policy file | MEDIUM | HIGH | P3 |
| Generalized parity-diff framework for more command families | MEDIUM | HIGH | P3 |

**Priority key:**
- P1: Must have for this milestone to count as hardening
- P2: Strong follow-on once core correctness is in place
- P3: Valuable, but should not delay the v1.1 safety baseline

## Competitor Feature Analysis

| Feature | Mature Tooling Pattern | Our Approach |
|---------|------------------------|--------------|
| Watch correctness under noisy filesystems | Watchman settles events before dispatch and treats loss-of-sync as a recrawl event; Watchexec documents backend limits and the need to rescan on some platforms. | Sumac should coalesce events, detect degraded conditions, and rebuild from a canonical file model instead of trusting every raw event. |
| Secure install payload | Krew installs from manifest-defined archives, verifies integrity, and extracts explicit files; VS Code packaging supports `.vscodeignore` to keep unwanted files out of extension payloads. | Sumac should move from recursive copy to an allowlisted install plan that defines the exact serve surface. |
| Trust and validation | VS Code separates installation from trust decisions and supports restricted mode and allowed-extension policy; npm provenance links published artifacts back to a build source but does not claim code is safe by itself. | Sumac should separate acquisition, validation, and activation, and record enough source metadata to make trust decisions reviewable. |
| Core CLI contract stability during extension growth | GitHub CLI protects core commands from extension override, supports pinning for extension installs, and documents stable exit-code behavior. | Sumac should preserve `1.x` contracts while moving watch and skills onto module-scoped services behind parity gates. |

## Sources

- Project context: [/Users/hprincivil/Projects/sxmc/.planning/PROJECT.md](/Users/hprincivil/Projects/sxmc/.planning/PROJECT.md) (HIGH)
- Codebase concerns: [/Users/hprincivil/Projects/sxmc/.planning/codebase/CONCERNS.md](/Users/hprincivil/Projects/sxmc/.planning/codebase/CONCERNS.md) (HIGH)
- Product surface and shipped behavior: [/Users/hprincivil/Projects/sxmc/README.md](/Users/hprincivil/Projects/sxmc/README.md) (HIGH)
- Watchman overview: https://facebook.github.io/watchman/ (HIGH)
- Watchman settled trigger model: https://facebook.github.io/watchman/docs/watchman-make (HIGH)
- Watchman recrawl behavior: https://facebook.github.io/watchman/docs/troubleshooting (HIGH)
- Watchexec backend limitations on macOS: https://watchexec.github.io/docs/macos-fsevents.html (MEDIUM)
- Watchexec Linux queue and watch limits: https://watchexec.github.io/docs/inotify-limits.html (MEDIUM)
- Krew plugin manifest and explicit install files: https://krew.sigs.k8s.io/docs/developer-guide/plugin-manifest/ (HIGH)
- Krew install integrity verification: https://krew.sigs.k8s.io/docs/user-guide/installing-plugins/ (HIGH)
- Krew local install validation flow: https://krew.sigs.k8s.io/docs/developer-guide/testing-locally/ (HIGH)
- GitHub CLI extension lifecycle: https://cli.github.com/manual/gh_extension and https://cli.github.com/manual/gh_extension_install (HIGH)
- GitHub CLI exit-code contract: https://cli.github.com/manual/gh_help_exit-codes (HIGH)
- VS Code extension runtime security: https://code.visualstudio.com/docs/configure/extensions/extension-runtime-security (HIGH)
- VS Code workspace trust guide: https://code.visualstudio.com/api/extension-guides/workspace-trust (HIGH)
- VS Code enterprise extension allowlist policy: https://code.visualstudio.com/docs/enterprise/extensions (HIGH)
- VS Code extension packaging and `.vscodeignore`: https://code.visualstudio.com/api/working-with-extensions/publishing-extension (HIGH)
- npm trusted publishing: https://docs.npmjs.com/trusted-publishers/ (HIGH)
- npm provenance limits: https://docs.npmjs.com/generating-provenance-statements/ (HIGH)
- Rust CLI testing guidance: https://rust-cli.github.io/book/tutorial/testing.html (MEDIUM)
- Clap subcommand composition: https://docs.rs/clap/latest/clap/_derive/index.html (MEDIUM)

---
*Feature research for: `v1.1 Platform Hardening and Core Expansion`*
*Scoped to new watch reliability, skill lifecycle hardening, and orchestration-hardening work only*
