# Project Research Summary

**Project:** Sumac v1.1 Platform Hardening and Core Expansion
**Domain:** Hardening a shipped Rust CLI/MCP product without breaking `1.x` contracts
**Researched:** 2026-04-04
**Confidence:** HIGH

## Executive Summary

Sumac v1.1 is not a stack-reset milestone. It is a hardening milestone for a shipped `1.x` Rust CLI/MCP product whose weak points are watch correctness, skill install/serve safety, and remaining orchestration hotspots. The research is consistent: experts would keep the existing Rust/Tokio/Clap/Notify/Reqwest foundation, add little or no new runtime surface, and fix correctness by creating one canonical skill asset model that install, serve, scan, and watch all share.

The recommended approach is dependency-light and sequence-sensitive. First establish the canonical recursive asset inventory and watch parity harness, then harden reload/runtime behavior, then fix skill materialization and allowlisted staging, then close the security loop so installed files, scanned files, and served files are the same set, and only then finish the watch and skills command-family extraction behind stable app-service seams. v1.1 should add almost no new technology; the main optional addition is `walkdir` if the team wants one shared recursive traversal utility.

The main risks are contract drift and false confidence. Watch bugs will persist if polling and event modes do not follow the same file model. Security gaps will persist if install, scan, and serve each keep separate policies. CLI regressions will slip through if `1.x` behavior is treated as just flag compatibility instead of output, exit-code, and generated-artifact compatibility. Mitigation is explicit: parity tests, fail-closed policy enforcement, atomic staging, bounded async side effects, and a rollout that keeps the rollback seam until soak evidence justifies removal.

## Key Findings

### Recommended Stack

v1.1 should mostly be a non-addition milestone. Keep the current stack and use it more rigorously rather than introducing new frameworks or switching libraries. The core fixes come from disciplined use of `std::fs`/`std::path`, Tokio async process and timing primitives, first-party `notify` event and poll backends, shared `reqwest::Client` timeouts, and proper `tempfile::TempDir` staging semantics.

**Core technologies:**
- `std::fs` / `std::path` / `std::process::Command`: canonical path checks, symlink rejection, scoped copy rules, and lightweight git invocation without a new systems layer.
- `tokio 1.x`: non-blocking watch loops, timeout-bounded subprocesses, cancellation, and bounded concurrency for notify commands and webhooks.
- `notify 8.x`: primary event watching plus `PollWatcher` fallback, with explicit parity between native and degraded modes.
- `reqwest 0.13.x`: shared outbound HTTP client with connect/request/read timeouts so slow webhooks do not stall watch flows.
- `tempfile 3.x`: ephemeral staging for install/update and cleanup verification; do not persist transient clone dirs with `keep()`.

**Stack additions or non-additions for v1.1:**
- Do not add `git2`, a new runtime, or a new framework.
- Do not use `.gitignore`-style filtering as a security boundary.
- Add `walkdir 2.x` only if Sumac consolidates recursive traversal into one shared internal utility.
- Reuse existing `regex` with `regex::bytes` for byte-level scanner coverage instead of adding a new scanner dependency.

### Expected Features

The milestone table stakes are reliability and safety features users already assume a mature CLI/MCP product has. If these are missing, v1.1 feels unsafe rather than merely incomplete.

**Must have (table stakes):**
- Settled, nested-aware watch invalidation across the actual skill serve surface.
- Native-event and polling parity, with explicit degraded-mode signaling and bounded rescan behavior.
- Async, timeout-bounded notify-command and webhook execution.
- Allowlisted, staged skill install/update with atomic replace, temp cleanup, and reproducible git-source handling.
- Fail-closed scan coverage for all install-surface files that can later be served.
- `watch` and `skills` command-family seams with contract tests preserving `1.x` CLI, JSON, and exit-code behavior.

**Should have (competitive):**
- Watch health and degraded-mode diagnostics.
- Install preview and provenance reporting.
- Incremental reload classification once correctness is stable.

**Defer (v1.2+):**
- Team trust tiers or policy-file restrictions for allowed origins and activation rules.
- Generalized parity-diff or shadow-routing infrastructure beyond the `watch` and `skills` families.

### Architecture Approach

The architecture recommendation is a modular monolith inside the current crate. Keep `src/main.rs` thin, move `watch` and `skills` into `src/app/` services, keep runtime ownership for serve/reload in `src/server/`, and make `src/skills/` the source of truth for the canonical asset inventory, install policy, source materialization, and staging. The key pattern is single-source-of-truth boundaries: one asset model, one reload build per change, one allowlist policy, and one contract-preserving CLI shell.

**Major components:**
1. `src/app/watch.rs` and `src/app/skills.rs` — command-family orchestration, exit behavior, and stable CLI-facing results.
2. `src/server/reload.rs` and `src/server/inventory.rs` — one-build reload coordination, backend parity, and atomic server snapshot swaps.
3. `src/skills/models.rs`, `parser.rs`, `policy.rs`, `source.rs`, and `staging.rs` — canonical asset inventory, allowlist enforcement, git/local materialization, and atomic install/update.
4. `src/security/skill_scanner.rs` — byte-aware, fail-closed scan coverage over the same managed asset set used by install and serve.

### Critical Pitfalls

1. **Treating watch hardening as a backend swap** — fix the contract, not just the watcher backend; poll and event modes must follow the same canonical asset inventory and parity tests.
2. **Leaving blocking or unbounded side effects in the watch loop** — move notify commands and webhook delivery behind async, timeout-bounded, failure-isolated execution.
3. **Persisting untrusted skill trees before validation** — materialize sources ephemerally, reject symlinks and junk, copy only an explicit allowlist, then atomically activate.
4. **Letting install, scan, and serve policies diverge** — make one policy authoritative and enforce the invariant `served_files == scanned_allowed_files == installed_allowed_files`.
5. **Changing internals while drifting `1.x` behavior** — freeze command contracts, run parity diffs, and keep the rollback seam until soak evidence says it is safe to retire.

## Implications for Roadmap

Based on the research, suggested phase structure:

### Phase 1: Canonical Asset Model and Watch Parity Harness
**Rationale:** This is the shared dependency under watch reliability, secure install, serve filtering, and scanner coverage. Without it, every later fix drifts.
**Delivers:** Recursive canonical asset inventory, nested-asset fixtures, poll/event parity harness, and the baseline for a single watched-input model.
**Addresses:** Settled nested-aware watch invalidation, degraded-mode parity groundwork, shared file model for install/serve/scan/watch.
**Avoids:** Watch-contract drift, repeated file-model divergence, and false positives from shallow polling.

### Phase 2: Watch Runtime Hardening
**Rationale:** Once the watched surface is defined, runtime behavior can be fixed without guesswork.
**Delivers:** `notify` event plus `PollWatcher` fallback feeding one reload coordinator, single-build reload path, async notify-command execution, shared timeout-configured `reqwest::Client`, and bounded webhook fan-out.
**Uses:** Existing `tokio`, `notify`, and `reqwest` stack; no framework additions.
**Implements:** `src/server/reload.rs`, `src/server/inventory.rs`, and `src/app/watch.rs` runtime boundaries.
**Avoids:** Blocking side effects, double-build reload cost, backend-specific semantics, and cross-platform subprocess drift.

### Phase 3: Secure Skill Materialization and Staged Install
**Rationale:** Hardening install/update should happen after the canonical asset and watch model is in place, because the same policy drives what may be persisted.
**Delivers:** Source normalization, ephemeral staging, repo-root/subpath correctness for git sources, explicit allowlisted copy, temp cleanup, and reproducible install metadata.
**Uses:** `std::fs`/`std::path`, system `git`, `tempfile`, and optionally `walkdir` if recursive traversal is consolidated.
**Implements:** `src/skills/source.rs`, `src/skills/policy.rs`, `src/skills/staging.rs`, and a thinner `src/skills/install.rs`.
**Avoids:** Recursive copy of unsafe trees, tempdir leakage, repo-root confusion, and git fixups that only patch symptoms.

### Phase 4: Unified Scan and Serve Enforcement
**Rationale:** After install/update is constrained, close the security loop so Sumac only serves what it can prove it scanned and approved.
**Delivers:** Allowlist-based serving, byte-level scanner coverage, explicit skipped-file findings, and invariant tests across installed/scanned/served sets.
**Addresses:** Fail-closed scan coverage and predictable MCP exposure.
**Avoids:** Silent scanner blind spots, traversal-only defenses, and mismatches between persisted and exposed content.

### Phase 5: Command-Family Extraction with Contract Gates
**Rationale:** Move the remaining hotspot families only after correctness and security behavior are defined, so extraction is mostly boundary cleanup rather than semantic change.
**Delivers:** Thin `main.rs`, first-class `app::watch` and `app::skills` services, shared status/reconciliation facade, contract snapshots, and parity-diff tests for stable commands.
**Implements:** App-service seams recommended by architecture research while preserving `1.x` behavior.
**Avoids:** Internal refactors that accidentally change stdout/stderr, exit codes, JSON, generated files, or wrapper behavior.

### Phase 6: Soak, Diagnostics, and Targeted v1.1.x Enhancements
**Rationale:** Diagnostics and seam retirement should follow proven correctness, not precede it.
**Delivers:** Watch health diagnostics, install preview/provenance reporting, soak evidence for rollback-seam retirement, and validation on npm/Homebrew/wrapper paths.
**Addresses:** Differentiators that improve operator trust without delaying the v1.1 safety baseline.
**Avoids:** Premature rollback-seam deletion and support churn from invisible degraded behavior.

### Phase Ordering Rationale

- The canonical asset model comes first because it is the dependency beneath watch, install, serve, and scan.
- Runtime watch hardening comes before command extraction so the team fixes real behavior before moving files around.
- Install hardening must precede serve/scan enforcement because the safest place to constrain trust is before persistence.
- Command-family extraction is late in the sequence because v1.1’s biggest risk is semantic drift during refactor.
- Diagnostics and provenance are useful, but they should ride on top of a proven safety baseline instead of delaying it.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 2:** Cross-platform subprocess and wrapper behavior should be validated explicitly if notify commands remain shell-capable.
- **Phase 3:** GitHub tree URL, repo-subpath, ref pinning, and cleanup behavior need targeted fixture planning even though the high-level pattern is clear.
- **Phase 6:** Rollback-seam retirement needs explicit soak criteria and packaging-path validation, not just code-level parity.

Phases with standard patterns (skip research-phase):
- **Phase 1:** Canonical inventory and parity harness work is well defined from current repo concerns and official watcher docs.
- **Phase 4:** Unified allowlist-based serve/scan enforcement follows a clear pattern once the asset model exists.
- **Phase 5:** App-service extraction is structurally straightforward after contract harnesses are in place; the risk is validation, not design ambiguity.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | Based on current repo dependencies plus official docs for `notify`, `tokio`, `reqwest`, `tempfile`, and Rust std APIs; recommendations are mostly non-additions. |
| Features | HIGH | Table stakes and deferrals are tightly grounded in shipped product scope and mature CLI/plugin/watch patterns. |
| Architecture | HIGH | Strong alignment between current repo structure, brownfield constraints, and the recommended app/server/skills boundaries. |
| Pitfalls | HIGH | Highest-risk failures are directly evidenced in current code paths and reinforced by official watcher/tempdir/process semantics. |

**Overall confidence:** HIGH

### Gaps to Address

- Final allowlist shape: confirm the exact managed metadata files v1.1 will install and serve beyond `SKILL.md`, `scripts/**`, and `references/**`.
- `walkdir` decision: choose early whether to add it as the single traversal primitive or keep bespoke recursion under one shared utility.
- Diagnostics scope: decide whether watch health diagnostics and provenance reporting are in-core v1.1 or explicitly moved to v1.1.x.
- Cross-platform validation: include npm/Homebrew/wrapper-path smoke coverage in planning for any phase that changes subprocess or filesystem behavior.

## Sources

### Primary (HIGH confidence)
- `.planning/research/STACK.md` — runtime and library recommendations, non-additions, and version constraints
- `.planning/research/FEATURES.md` — table stakes, differentiators, anti-features, and MVP boundaries
- `.planning/research/ARCHITECTURE.md` — command-family seams, canonical asset model, reload pipeline, and build order
- `.planning/research/PITFALLS.md` — failure modes, prevention phases, contract risks, and rollout hazards
- `.planning/PROJECT.md` — milestone goals and active requirements for v1.1

### Secondary (MEDIUM confidence)
- `notify`, `tokio`, `reqwest`, `tempfile`, `walkdir`, and Rust std documentation cited in the underlying research files — used to verify watcher, timeout, cleanup, and traversal behavior
- Mature-tool references cited in `FEATURES.md` such as Watchman, Watchexec, Krew, GitHub CLI, and VS Code extension docs — used to validate table-stakes expectations and safe install patterns

### Tertiary (LOW confidence)
- None. The remaining uncertainty is about milestone scoping choices, not unsupported technical claims.

---
*Research completed: 2026-04-04*
*Ready for roadmap: yes*
