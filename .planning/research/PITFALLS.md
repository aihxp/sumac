# Domain Pitfalls

**Domain:** Brownfield rewrite of a stable Rust CLI/MCP developer tool
**Researched:** 2026-04-04
**Overall confidence:** HIGH for rewrite strategy and contract-preservation guidance; HIGH for Sumac-specific contract risks from repo docs and tests

## Critical Pitfalls

### Pitfall 1: Running a parallel-universe rewrite instead of a slice-by-slice migration
**What goes wrong:** The rewrite becomes a second product tree that tries to catch up with the shipping one. Bug fixes, release work, and contract updates land on the old path while the new path drifts farther from reality.
**Why it happens:** Teams treat internal cleanup as a greenfield opportunity and optimize for architectural purity instead of continuous replacement.
**Consequences:** Long-lived drift, painful merges, frozen releases, and a final cutover that is too large to validate safely.
**Warning signs:**
- The plan requires "moving everything first" before any production cutover.
- The new core duplicates command orchestration instead of sitting behind a narrow seam.
- Releases or bug fixes are deferred until the rewrite lands.
- Old and new implementations cannot coexist behind the same command boundary.
**Prevention:**
- Use branch-by-abstraction at the command-family boundary, not a parallel tree.
- Migrate one stable slice at a time, starting with the maintained golden path.
- Keep trunk releasable throughout the rewrite.
- Define explicit replacement seams before writing replacement logic.
**Address in migration:** Rewrite strategy and seam design, before the first command family moves.

### Pitfall 2: Starting the rewrite without characterization tests for public behavior
**What goes wrong:** The team preserves internal intent, not external behavior. JSON shape, exit codes, stderr noise, generated files, and help output drift because nobody captured what users and scripts actually depend on.
**Why it happens:** Mature CLIs accumulate contract behavior across docs, shell smoke tests, integration fixtures, and automation scripts; it is rarely all encoded in one module.
**Consequences:** Silent breakage for CI jobs, wrappers, AI-host onboarding flows, and any automation parsing `sxmc` output.
**Warning signs:**
- Tests assert only `.success()` instead of stdout/stderr/exit-code details.
- There is no snapshot or golden-master coverage for stable commands.
- Maintainers cannot answer which fields, aliases, or file edits are contract versus incidental.
- Generated artifacts are validated manually instead of diffed.
**Prevention:**
- Build a contract matrix from `docs/PRODUCT_CONTRACT.md`, `docs/STABILITY.md`, `tests/cli_integration.rs`, and smoke scripts.
- Snapshot stdout, stderr, exit codes, and generated files for `setup`, `add`, `doctor`, `status`, and `sync`.
- Keep platform-normalized snapshots for Linux, macOS, and Windows-sensitive output.
- Add characterization tests before moving logic, not after.
**Address in migration:** Baseline capture, before the first refactor that changes command orchestration.

### Pitfall 3: Preserving command names while breaking semantics
**What goes wrong:** Commands and flags still exist, but their meaning changes. Typical breakages are JSON field removal or renaming, enum/state meaning drift, different default scopes, altered alias behavior, changed exit semantics, or new stderr chatter contaminating machine-readable flows.
**Why it happens:** Rewrite teams treat parser compatibility as sufficient and overlook that a CLI's real API includes stdout, stderr, file side effects, aliases, and failure modes.
**Consequences:** Consumers that depended on "stable enough" semantics break even though `--help` still looks familiar.
**Warning signs:**
- Review language says "internals only" even though rendering, defaults, or file writes changed.
- New code swaps output models without contract review.
- Stable aliases like `--client`/`--host` or `--global`/`--local` are routed differently.
- Non-zero statuses or recovery hints differ from the shipped contract.
**Prevention:**
- Treat `setup`, `add`, `doctor`, `status`, and `sync` structured outputs as public APIs.
- Make parity review cover stdout/stderr separation, exit codes, aliases, defaults, and generated-file edits.
- Require additive evolution only for stable `1.x` JSON surfaces.
- Gate merges on contract-level tests, not only unit tests.
**Address in migration:** Command-slice design and every cutover PR touching stable workflow behavior.

### Pitfall 4: Accidentally "fixing" legacy quirks that users rely on
**What goes wrong:** The rewrite cleans up odd behavior that was never officially celebrated but is now relied upon by scripts, wrappers, or recovery workflows.
**Why it happens:** Mature tools often have compatibility quirks that look like bugs in code review but function like contracts in the field.
**Consequences:** Regressions are framed as cleanup, rollbacks become politically harder, and users lose trust because behavior changed without a major version or migration note.
**Warning signs:**
- PRs describe user-visible drift as simplification or cleanup.
- Known quirks are not tagged as preserve/fix/deprecate decisions.
- A refactor removes special-case handling without an explicit contract call.
- Reviewers debate whether behavior is a bug after code is already rewritten.
**Prevention:**
- Maintain a compatibility ledger for every observed quirk: preserve now, fix with note, or deprecate for a later major.
- Require explicit release-note treatment for intentional drift in stable commands.
- Snapshot current behavior first, then decide which mismatches are acceptable.
- Separate bug-fix work from architectural migration unless the contract decision is already made.
**Address in migration:** Baseline triage and design review for each migrated slice.

### Pitfall 5: Rewriting orchestration and persistent artifacts at the same time
**What goes wrong:** The new core changes not only execution flow but also state layout, generated file edits, cache behavior, or install metadata. Users see contract breakage through changed host files, unreadable old state, or different sync/doctor results.
**Why it happens:** Teams bundle control-flow cleanup with "while we're here" state and file-format normalization.
**Consequences:** Hard-to-debug drift in `status`, `sync`, startup-file reconciliation, skill metadata, and cache reuse.
**Warning signs:**
- The rewrite introduces new state schemas or file writers before parity is proven.
- Round-trip tests for existing `.sxmc` state or generated host files are missing.
- Generated file diffs show reordering, block replacement, or path changes unrelated to the intended slice.
- Cache and metadata errors are treated as best-effort during migration.
**Prevention:**
- Keep compatibility readers and writers until the new path has shipped safely.
- Add round-trip and file-diff tests using real fixtures from the current product.
- Migrate orchestration first; postpone format normalization unless required for safety.
- Define exact invariants for generated files, state files, and managed metadata before cutover.
**Address in migration:** Seam design and cutover planning for `setup`, `doctor`, `status`, `sync`, and managed skill flows.

### Pitfall 6: Proving only the happy path
**What goes wrong:** The rewritten path works on ideal fixtures but fails when integrations are partial, slow, missing, or malformed. Stable products usually fail most visibly in recovery and degraded modes, not in the golden demo flow.
**Why it happens:** Rewrite programs bias toward clean inputs and nominal success cases because they are easier to implement and validate.
**Consequences:** Crashes, misleading health reports, bad remediation guidance, hanging watch loops, or incorrect partial success behavior.
**Warning signs:**
- Negative-path coverage is thinner in the new slice than the old one.
- Missing files, auth failures, promptless/resource-less servers, slow hooks, and unreadable assets are not exercised.
- Recovery hints or warnings disappear because the new code path is "cleaner."
- Existing known-bad cases are deferred until after cutover.
**Prevention:**
- Preserve a graceful-failure catalog alongside the happy-path contract matrix.
- Add regression cases for absent tools, malformed configs, permission failures, transport quirks, and partial installs.
- Keep recovery guidance additive and explicit.
- Test timeouts, cancellation, and partial failures as first-class contract behavior.
**Address in migration:** Characterization phase and every slice readiness review.

### Pitfall 7: Cutting over without side-by-side diffing and rollback
**What goes wrong:** Teams switch the stable command over to the new implementation without proving output equivalence on the same inputs or keeping a fast retreat path.
**Why it happens:** Once a new architecture compiles and passes a subset of tests, teams assume remaining mismatches are minor.
**Consequences:** Edge-case regressions escape to users, diagnosis is slow, and rollback becomes manual surgery instead of a controlled switch.
**Warning signs:**
- There is no way to run old and new implementations against the same fixtures.
- CI validates only the new path.
- Cutover removes the old implementation immediately.
- Mismatch reporting is ad hoc instead of systematic.
**Prevention:**
- Add a compare mode or hidden test harness that executes both implementations and diffs JSON, files, and exit behavior.
- Keep rollback simple: one routing switch, not a revert marathon.
- Require burn-in across at least one release cycle or equivalent internal soak before deleting the old path.
- Promote only after mismatch counts are understood and intentionally accepted.
**Address in migration:** Before the first real cutover and during every staged rollout.

## Moderate Pitfalls

### Pitfall 1: Cross-platform and packaging regressions appear late
**What goes wrong:** The Rust path passes locally, but Windows path handling, npm/Homebrew wrappers, shell scripts, quoting, temp paths, or line endings drift.
**Prevention:** Keep `cargo test`, portable smoke scripts, packaging checks, and release certification wired into every migrated slice. Treat cross-platform parity as part of the contract, not release polish.
**Warning signs:** CI for the new path skips Windows or packaging lanes; snapshot tests normalize Unix only; wrapper install flows are not exercised.
**Address in migration:** Continuous validation during every slice, especially before user-facing cutovers.

### Pitfall 2: Output becomes nondeterministic after refactoring
**What goes wrong:** Hash map iteration, filesystem walk order, async fan-out, or parallel rendering changes output order. The product "works," but diff-based automation and snapshots become flaky.
**Prevention:** Canonically sort command listings, JSON collections, and generated file blocks before rendering. Preserve deterministic traversal in file-backed features.
**Warning signs:** Flaky snapshots, platform-specific ordering diffs, or reviewers dismissing order changes as cosmetic.
**Address in migration:** Implementation details inside every renderer and file writer.

### Pitfall 3: Operational regressions hide behind behavioral parity
**What goes wrong:** The new internals return the same answer but with worse startup time, more blocking work, slower watch loops, or more resource usage.
**Prevention:** Track startup and hot-path benchmarks for stable commands; keep timeout and cancellation behavior under test; treat latency regressions on developer workflows as product regressions.
**Warning signs:** New abstraction layers add repeated parsing or duplicate rebuilds; async paths regain blocking calls; no benchmark or smoke timing comparison is kept.
**Address in migration:** Before broad rollout of a new slice and again before release.

### Pitfall 4: Docs and support boundaries drift away from the product
**What goes wrong:** The code, docs, stability statement, and release bar stop agreeing. Users then discover behavior changes by trial and error.
**Prevention:** Make contract docs part of the cutover checklist. If behavior changes intentionally, update `PRODUCT_CONTRACT.md`, `STABILITY.md`, usage docs, and validation notes in the same change window.
**Warning signs:** "We will document it later," release notes omit user-visible drift, or smoke/contract docs are stale relative to the branch.
**Address in migration:** Every user-visible slice and release review.

## Minor Pitfalls

### Pitfall 1: Compatibility shims stay forever
**What goes wrong:** Temporary adapters become permanent complexity because no removal criteria were defined.
**Prevention:** Put an expiration condition on every shim: parity proven, soak complete, rollback window closed, docs updated.
**Warning signs:** Adapters have no owner or deletion milestone.
**Address in migration:** When introducing the seam, not after cleanup becomes urgent.

### Pitfall 2: The rewrite focuses on code structure, not failure observability
**What goes wrong:** The new core is cleaner internally but harder to diagnose because mismatch logs, structured warnings, and cutover telemetry were never added.
**Prevention:** Add explicit mismatch logging, artifact diffs, and parity metrics during migration. Keep diagnostics on stderr and machine-readable output on stdout.
**Warning signs:** Regressions are discovered only from user reports, not during internal compare runs.
**Address in migration:** During seam creation and compare-mode implementation.

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|---------------|------------|
| Core/app boundary extraction from `src/main.rs` | Creating a new orchestration layer that changes stable defaults, aliases, or stderr/stdout behavior | Freeze contract tests for `setup`, `add`, `doctor`, `status`, and `sync` before moving dispatch logic |
| Golden-path migration (`setup -> add -> status -> sync`) | Preserving commands but drifting JSON shape, exit codes, or generated host-file edits | Use file-diff and JSON snapshot tests plus old/new compare mode on the same fixtures |
| Skill install rewrite | Fixing the current git/tempdir bug while also changing exposure rules or install metadata semantics | Split bug fix from broader model cleanup; add end-to-end git install tests and explicit compat decisions |
| Skill file model consolidation | Changing parser, server, and watch semantics at once for nested assets | Define one canonical skill file model first, then migrate install/discovery/serve/watch behind that model |
| Watch/reload migration | Reintroducing blocking work, hanging notifications, or silent missed reloads while refactoring | Add timeouts, cancellation tests, nested-asset change tests, and parity checks for event vs polling modes |
| Cache and state handling | Silent cache mutation errors or unreadable old state after internal cleanup | Add structured warnings, backward-compatible readers, and corruption/permission tests before swapping implementations |
| Transport boundaries (`serve`, `stdio`, `http`, `mcp`) | Behavioral drift hidden by unit tests that never exercise real subprocess and HTTP flows | Keep subprocess and local-listener integration tests as release gates; do not reduce transport realism during migration |
| Release and packaging automation | New internal core passes `cargo test` but breaks npm/Homebrew/release certification | Keep `scripts/certify_release.sh`, smoke scripts, and packaging syntax checks in the definition of done for each slice |

## Sources

- Repo contract and stability references: `docs/PRODUCT_CONTRACT.md`, `docs/STABILITY.md`, `docs/VALIDATION.md`, `.planning/codebase/CONCERNS.md`, `.planning/codebase/TESTING.md`
- Martin Fowler, "Branch By Abstraction" (2014, updated page dated 2024): https://martinfowler.com/bliki/BranchByAbstraction.html
- Martin Fowler, "Strangler Fig" (updated 2024): https://martinfowler.com/bliki/StranglerFigApplication.html
- Joel Spolsky, "Things You Should Never Do, Part I" (2000): https://www.joelonsoftware.com/2000/04/06/things-you-should-never-do-part-i/
- Semantic Versioning 2.0.0: https://semver.org/
- Approval Tests overview: https://approvaltests.com/
- Insta snapshot testing docs: https://insta.rs/docs/
- Insta CLI testing docs: https://insta.rs/docs/cmd/
- `assert_cmd` crate docs: https://docs.rs/assert_cmd/latest/assert_cmd/
- Pact contract testing introduction: https://docs.pact.io/

