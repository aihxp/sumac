# Feature Landscape

**Domain:** Product-preserving rewrite program for a mature `1.x` CLI/MCP tool
**Researched:** 2026-04-04

## Table Stakes

Features maintainers and existing users expect from the rewrite itself. Missing these makes the rewrite feel unsafe, not ambitious.

| Feature | Why Expected | Complexity | Dependencies | Notes |
|---------|--------------|------------|--------------|-------|
| Published interface inventory | A shipped `1.x` CLI already has a real contract: commands, flags, env vars, exit codes, JSON fields, generated files, and MCP-facing behavior. The rewrite must know what it is preserving. | Medium | Command census, fixture corpus, owner review | This is the baseline for every later parity decision. Treat undocumented-but-relied-on behavior as part of the contract until proven otherwise. |
| Command and flag parity gates | Users expect scripts, docs, and muscle memory to keep working through the rewrite. | High | Interface inventory, CLI integration tests, smoke tests | Include help text shape where relied on, argument parsing, error wording where automation depends on it, and stable aliases if they already exist. |
| Machine-output and artifact parity | Mature CLI products are consumed by other tools. `--json`, generated config, baked MCP outputs, and install artifacts must remain stable. | High | Snapshot/golden tests, fixture comparisons, artifact diffing | For Sumac, this directly includes generated host artifacts and MCP-related materialization, not just human-readable stdout. |
| Incremental cutover path | A rewrite program is expected to migrate slice by slice, not pause shipping until every subsystem is rebuilt. | High | Core/app boundary, adapters, routing seams | The maintained golden path should move first, then remaining command families. This keeps the rewrite landable. |
| Characterization and contract harness | Existing behavior must be captured before internals change. Mature rewrite programs are expected to prove parity, not claim it. | High | Existing test suite, portable fixtures, golden outputs | Reuse real subprocess tests, fixture directories, and smoke scripts before inventing a new test style. |
| Release-preserving rollout | A 1.x CLI cannot disappear into a rewrite branch for months. Maintainers are expected to keep shipping fixes while migration continues. | High | Incremental cutover path, CI gating, branch discipline | The rewrite program should support mixed old/new internals in released builds. |
| Explicit deprecation and retirement rules | If temporary compatibility shims are introduced, maintainers need clear rules for when they are added, how they are observed, and when they can be removed. | Medium | Interface inventory, migration scorecard, release notes discipline | Without this, “temporary” compatibility code becomes permanent sprawl. |
| Regression budgets for performance, reliability, and security | Existing users will accept internal change only if startup time, watch behavior, install safety, and platform stability do not degrade. | Medium | Benchmarks, smoke tests, targeted bug fixtures | This is especially important when the rewrite claims architectural cleanup as its core value. |

## Differentiators

Capabilities that make the rewrite unusually credible and low-risk. Not strictly required to begin, but they materially improve trust and migration speed.

| Feature | Value Proposition | Complexity | Dependencies | Notes |
|---------|-------------------|------------|--------------|-------|
| Shadow execution and diff mode for migrated paths | Lets maintainers run legacy and rewritten paths against the same fixture or command and compare outputs before switching traffic. | High | Interface inventory, deterministic fixtures, diff tooling | Strong differentiator for command families with rich JSON or generated-file outputs. |
| Migration scorecard per subsystem | Makes progress legible: inventory complete, tests captured, new core live, rollback proven, old path retired. | Medium | Interface inventory, CI metadata, ownership boundaries | Prevents the rewrite from becoming a vague “ongoing refactor.” |
| Core/app migration kit | Shared adapters, routing conventions, and test helpers reduce one-off migration patterns across command families. | Medium | Core boundary design, module ownership | This is what turns one successful slice into a repeatable rewrite program. |
| Cross-platform parity dashboard | Continuously tracks whether rewritten slices still pass Linux, macOS, and Windows CLI and smoke coverage. | Medium | CI matrix, smoke scripts, artifact diffing | Important for mature CLIs where platform drift is often discovered late. |
| Built-in rollback and kill-switch routing | If a migrated path regresses in a release, maintainers can route that command family back to the legacy implementation quickly. | High | Incremental routing seam, release flags, telemetry or failure signals | Valuable during active shipping when confidence is high but not absolute. |
| Risk-targeted hardening folded into the migration | Uses the rewrite to fix known architectural hazards that sit on the migration path, such as over-broad file exposure or under-tested install flows, without broad product churn. | Medium | Parity harness, scoped design review, regression fixtures | This is the right kind of “improvement during rewrite”: narrow, verified, and attached to known risk. |

## Anti-Features

Things the rewrite program should explicitly avoid, even if they sound attractive in isolation.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Big-bang cutover | High rewrite risk, long-lived branch drift, and no safe place to validate behavior incrementally. | Use expand/migrate/contract seams and land slices continuously. |
| Opportunistic CLI redesign during the rewrite | Turns a trust-preserving internal migration into a product reset and makes regressions impossible to classify. | Preserve `1.x` behavior first; queue UX cleanups behind explicit post-parity decisions. |
| Release freeze until rewrite completion | Breaks user trust and creates a “parallel universe” codebase that is hard to merge back. | Keep the release train moving with mixed old/new internals. |
| Migrating every subsystem at once | Removes isolation, hides ownership, and makes rollback impossible. | Pick a stable golden path first, then migrate one command family or subsystem at a time. |
| Permanent dual implementations | Carrying both paths indefinitely recreates the complexity the rewrite was meant to remove. | Add retirement gates and delete legacy code once parity and rollback confidence are proven. |
| Dependency churn unrelated to migration goals | Simultaneous crate, runtime, packaging, and architecture churn makes failures ambiguous. | Upgrade only where the migration needs it or where a known risk justifies it. |
| Rewriting tests around mocked internals | Mature CLI contracts live at the binary, file, and subprocess boundary; overly mocked tests miss real regressions. | Prefer characterization via real CLI invocations, fixtures, local servers, and smoke scripts. |
| “Fix every historical quirk” cleanup | Mature products accumulate behavior that users silently depend on; removing it during rewrite causes accidental breakage. | Classify quirks as contract, bug, or deprecation candidate, then handle each intentionally. |
| Broad product narrowing disguised as architecture work | A rewrite program for a shipped CLI is not the right vehicle for re-deciding the product identity. | Keep scope on internal coherence and migration safety. |

## Feature Dependencies

```text
Published interface inventory -> Characterization/contract harness -> Command + artifact parity gates -> Incremental cutover path -> Legacy retirement

Core/app boundary -> Core/app migration kit -> Per-subsystem migration scorecard -> Release-preserving rollout

Portable fixtures + smoke scripts -> Cross-platform parity dashboard -> Rollback/kill-switch routing

Known risk inventory -> Risk-targeted hardening -> Safe retirement of fragile legacy paths
```

## MVP Recommendation

Prioritize:
1. Published interface inventory
2. Characterization and contract harness for the maintained golden path
3. Incremental cutover path with release-preserving rollout

Defer: Full shadow execution across every command family. It is valuable, but the first milestone should prove parity and cutover on the golden path before building a universal diff framework.

## Sources

- Project context: [/Users/hprincivil/Projects/sxmc/.planning/PROJECT.md](/Users/hprincivil/Projects/sxmc/.planning/PROJECT.md) (HIGH)
- Codebase concerns: [/Users/hprincivil/Projects/sxmc/.planning/codebase/CONCERNS.md](/Users/hprincivil/Projects/sxmc/.planning/codebase/CONCERNS.md) (HIGH)
- Testing patterns: [/Users/hprincivil/Projects/sxmc/.planning/codebase/TESTING.md](/Users/hprincivil/Projects/sxmc/.planning/codebase/TESTING.md) (HIGH)
- Semantic Versioning 2.0.0: https://semver.org/ (HIGH)
- Martin Fowler, “Parallel Change” (expand/migrate/contract): https://martinfowler.com/bliki/ParallelChange.html (HIGH)
- Martin Fowler, “Strangler Fig Application”: https://martinfowler.com/bliki/StranglerFigApplication.html (MEDIUM)
- `assert_cmd` crate docs, binary-level CLI testing patterns: https://docs.rs/crate/assert_cmd/2.0.17 (MEDIUM)

