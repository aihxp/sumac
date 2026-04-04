# Phase 1: Contract Baseline & Parity Harness - Research

**Researched:** 2026-04-04
**Domain:** Brownfield rewrite baselining, contract publication, and parity validation
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Preserve the existing `1.x` CLI behavior and JSON contracts while documenting
  the golden path.
- Treat `setup`, `add`, `status`, and `sync` as the canonical rewrite target
  for the first milestone.
- Capture parity at the command, artifact, and validation-report levels rather
  than relying on informal spot checks.
- Keep this phase focused on baseline publication and proof harnesses, not on
  moving runtime business logic yet.

### the agent's Discretion
- Exact file placement for the published contract inventory and rewrite-focused
  validation artifacts.
- Whether parity coverage lives primarily in Rust integration tests, shell
  harnesses, or a split model, as long as maintainers get deterministic,
  repeatable proof.
- How to summarize golden-path behavior most usefully for later migration
  phases, provided the inventory stays concise and versionable.

### Deferred Ideas (OUT OF SCOPE)
- Any new public command design or broader product reshaping.
- Core/app seam implementation work (belongs to Phase 2).
- Migration of adjacent families like `doctor`, `watch`, `discover`, or
  `skills` (belongs to later milestones or phases).
</user_constraints>

<research_summary>
## Summary

Phase 1 should not invent a new framework. The repo already has the three
ingredients needed for a stable rewrite baseline: a documented `1.x` contract
surface, broad Rust and shell characterization coverage, and cross-platform
validation scripts that are already release-gated. The best plan is therefore
to tighten and focus those existing assets around the golden path instead of
introducing a separate rewrite-only testing stack.

The strongest current sources of truth are split across `docs/PRODUCT_CONTRACT.md`,
`docs/STABILITY.md`, `docs/VALIDATION.md`, `docs/USAGE.md`, `tests/cli_integration.rs`,
and `scripts/test-sxmc.sh`. Planning should consolidate those into a single
golden-path inventory document plus explicit parity checks for `setup`, `add`,
`status`, and `sync`. The contract inventory becomes the migration reference,
while the parity harness makes later cutovers prove they are still honoring it.

**Primary recommendation:** publish one rewrite-focused golden-path contract
document, then extend the existing Rust + shell validation stack with explicit
golden-path parity checks and a rewrite-oriented validation report.
</research_summary>

<standard_stack>
## Standard Stack

The established tools already present in this repo for this domain:

### Core
| Tool | Purpose | Why Standard Here |
|------|---------|-------------------|
| `docs/PRODUCT_CONTRACT.md` | Stable support boundary | Already defines promised behavior and out-of-scope edges for `1.x` |
| `docs/STABILITY.md` | Stability rules | Already states additive JSON evolution and stable lifecycle expectations |
| `tests/cli_integration.rs` | Binary-level characterization | Already covers structured stdout/stderr, HTTP/stdio flows, and host lifecycle behavior |
| `scripts/test-sxmc.sh` | User-path regression suite | Already exercises the maintained golden path and global/local scope behavior |
| `scripts/certify_release.sh` + portable smoke scripts | Release-grade validation | Already provides cross-platform and packaging gates |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `docs/USAGE.md` | Canonical user workflow | To inventory command shape, examples, and maintained lifecycle guidance |
| `docs/VALIDATION.md` | Validation narrative | To align rewrite reporting with current release validation expectations |
| `.planning/codebase/CONVENTIONS.md` | Repo conventions | To keep new docs/tests aligned with established patterns |
| `.planning/codebase/TESTING.md` | Test landscape | To place new parity checks in the right layers |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Reuse current Rust + shell validation | Build a separate rewrite harness | More churn, duplicated effort, and higher risk of drift |
| Publish a focused golden-path inventory | Rely on scattered docs/tests | Harder for later phases to use as a single migration source of truth |
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Pattern 1: Published contract inventory plus executable parity checks
**What:** keep human-readable contract documentation paired with machine-checkable
tests that enforce the same behaviors.
**When to use:** rewrite baselines and stable `1.x` migrations.
**Recommended here:** document the golden path in one dedicated phase artifact
and back it with tests for command shape, JSON fields, generated artifacts, and
install-scope behavior.

### Pattern 2: Extend existing validation layers instead of forking them
**What:** add focused rewrite checks to the existing Rust integration suite,
portable smoke scripts, and broad shell suite.
**When to use:** brownfield rewrites where production behavior is already being
validated.
**Recommended here:** keep Phase 1 changes inside the established validation
stack so later phases can ship continuously without a separate rewrite branch.

### Pattern 3: Separate source-of-truth docs from migration implementation
**What:** Phase 1 publishes inventory and proof only; migration phases consume it.
**When to use:** when the current code path must remain stable while planning a
new internal seam.
**Recommended here:** avoid mixing Phase 1 with actual core/app extraction work.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Rewrite validation | New standalone framework | Current Rust tests + shell suite + portable smoke | Existing stack already gates releases and covers behavior close to users |
| Contract source of truth | Ad hoc comments in plans | Dedicated golden-path inventory doc | Later migration phases need one stable reference |
| Cross-platform proof | One-off developer notes | Current CI/portable smoke matrix plus rewrite reporting | Repeatable proof matters more than memory |

**Key insight:** the risk in this phase is fragmentation, not lack of tooling.
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Documenting behavior without executable proof
**What goes wrong:** later migration phases treat the contract as prose and
accidentally regress details that nobody rechecks.
**How to avoid:** every important contract point should map to either an
integration assertion, shell check, or validation-report gate.

### Pitfall 2: Building a rewrite-only test path
**What goes wrong:** maintainers have to remember two validation systems and the
rewrite harness drifts from real release behavior.
**How to avoid:** extend existing validation layers instead of creating a
parallel one.

### Pitfall 3: Letting the contract inventory become a changelog dump
**What goes wrong:** the document is too noisy to help later migration phases.
**How to avoid:** inventory the stable golden path only: commands, flags,
aliases, env/install scope, JSON fields, generated files, exit behavior, and
MCP-facing expectations.
</common_pitfalls>

## Validation Architecture

Use a three-part rewrite validation model:

1. **Golden-path inventory artifact**
   - one doc that inventories `setup`, `add`, `status`, and `sync`
   - include flags, aliases, env/install-scope behavior, JSON contracts,
     generated files, and exit behavior

2. **Executable parity checks**
   - Rust integration checks for stable structured outputs and key fields
   - shell suite checks for golden-path human flows, global/local behavior, and
     artifact generation
   - reuse portable smoke and release certification where they already cover
     the same contract

3. **Rewrite-oriented validation reporting**
   - update validation docs so maintainers can see the rewrite baseline, not
     just generic release health
   - later migration phases should add their parity results to the same story

Recommended evidence targets for this phase:
- a dedicated golden-path contract doc
- explicit parity tests for `setup`, `add`, `status`, and `sync`
- validation docs that say how rewrite parity is measured and where to read it

<open_questions>
## Open Questions

1. **Where should the golden-path inventory live long-term?**
   - What we know: `docs/PRODUCT_CONTRACT.md` already exists, but it covers much
     more than the rewrite target.
   - Recommendation: create a dedicated phase/rewrite contract artifact and
     link to it from the broader contract docs.

2. **How much shell-vs-Rust duplication is acceptable?**
   - What we know: both layers already exist and catch different failure modes.
   - Recommendation: keep only the minimal duplication needed to cover both
     machine-readable and user-path parity.
</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)
- `.planning/PROJECT.md` — rewrite intent and non-negotiables
- `.planning/REQUIREMENTS.md` — PAR-01, PAR-02, ROL-04 requirements
- `.planning/ROADMAP.md` — Phase 1 goal and success criteria
- `.planning/codebase/CONVENTIONS.md` — repo conventions
- `.planning/codebase/TESTING.md` — current testing stack
- `docs/PRODUCT_CONTRACT.md` — current stable contract statement
- `docs/STABILITY.md` — current `1.x` stability promises
- `docs/VALIDATION.md` — current validation narrative
- `tests/cli_integration.rs` — structured behavior checks
- `scripts/test-sxmc.sh` — broad golden-path regression coverage
</sources>
