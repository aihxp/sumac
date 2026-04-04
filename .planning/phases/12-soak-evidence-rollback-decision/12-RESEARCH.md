# Phase 12: Soak Evidence & Rollback Decision - Research

**Researched:** 2026-04-04
**Domain:** Release-soak evidence synthesis and rollback-seam governance
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- The rollback decision must be based on explicit evidence rather than default
  inertia.
- The evidence record must distinguish between passing verification and actual
  post-release soak.
- No new rollback seams should be introduced for migrated command families.
- The result should be documented clearly enough that maintainers can point to
  it later.

### the agent's Discretion
- Exact structure of the evidence matrix and decision notes.
- Which phase/project docs carry the final retained-or-retired answer.
- How much repo-local evidence is sufficient to support keeping `watch` and
  `skills` as sole-route while retaining the older golden-path seam.

### Deferred Ideas (OUT OF SCOPE)
- A future seam-retirement code change after more release soak exists
- New feature work beyond documenting the current decision
</user_constraints>

<research_summary>
## Summary

Phase 12 should not pretend the repo has evidence it does not have. The local
evidence is strong for migration correctness: Phases 8 through 11 all ended
with passing verification, and the repo contains targeted regressions plus
real-fixture contract tests for `watch`, `skills`, and `serve`. That is enough
to say the migrated `watch` and `skills` route is operating as the sole route
with explicit local proof behind it. It is not enough to claim prolonged
release-soak across shipped versions after the new extraction work.

The strongest decision, based on the available evidence, is therefore:
1. keep `watch` and `skills` on the migrated sole route with no added rollback
   seam, because their phase-level verification and contract gates are strong;
2. retain the older `SXMC_GOLDEN_PATH_ROUTE=legacy` seam intentionally for now,
   with documented justification that true release-soak evidence for retirement
   is still pending.

**Primary recommendation:** produce an explicit soak-evidence record spanning
the completed v1.1 phases, then update project/milestone docs to say the
golden-path legacy seam is retained intentionally pending a later release-soak
review, not by neglect.
</research_summary>

<standard_stack>
## Standard Stack

The established tools already present in this repo for this domain:

### Core
| Tool | Purpose | Why Standard Here |
|------|---------|-------------------|
| Phase verification docs | Ground-truth evidence for each migrated slice | Already record the exact validation that passed |
| `tests/cli_integration.rs` parity and contract tests | Concrete route-safety evidence | Already proves golden-path parity and migrated `skills`/`serve` behavior |
| `.planning/PROJECT.md` and milestone docs | Official place to record retention vs retirement decisions | Maintainers already look here for milestone intent and constraints |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `.planning/research/ARCHITECTURE.md` | Rollback seam governance guidance | To avoid inventing new toggles instead of making a decision |
| `.planning/STATE.md` | Current handoff and milestone position | To record the post-decision next step cleanly |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Explicitly retaining the legacy seam with justification | Retiring it immediately | Would overstate the current evidence and conflate test proof with release soak |
| Evidence synthesis from existing artifacts | Ad hoc reasoning in chat only | Would leave no durable decision record for maintainers |
| No new rollback seam for `watch`/`skills` | Adding another env toggle "just in case" | Conflicts with the architecture guidance and expands complexity without evidence |
</standard_stack>

<architecture_patterns>
## Architectural Patterns

### Pattern 1: Evidence Matrix Before Decision
**What:** summarize what was verified, where, and what that evidence does or
does not prove.
**Why here:** the phase goal is specifically decision-by-evidence.
**Recommended here:** yes.

### Pattern 2: Sole Route for New Families, Explicit Retention for Old Seam
**What:** keep migrated `watch`/`skills` as the only route while separately
documenting the status of the older onboarding rollback seam.
**Why here:** that matches the actual codebase and avoids false symmetry.
**Recommended here:** yes.

### Pattern 3: Documentation as Operational Control
**What:** use project and phase docs to make the rollback status explicit.
**Why here:** the current requirement is governance and release confidence, not
another behavioral feature.
**Recommended here:** yes.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Phase 12 evidence | A new soak framework or runtime metrics system | Existing verification artifacts and contract tests | The repo already has the evidence needed for the current decision |
| Migration fallback | A new env-flag route for `watch` or `skills` | Keep them sole-route and document the decision | The architecture explicitly warns against more routing toggles without evidence |
| Rollback decision | Implicit status quo | Explicit retained-or-retired language in milestone docs | The requirement is about avoiding inertia |
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Treating green tests as equivalent to release soak
**What goes wrong:** the seam is retired on evidence the repo does not actually
contain.
**How to avoid:** distinguish validation confidence from post-release soak
confidence in the decision record.

### Pitfall 2: Forgetting that `watch`/`skills` and the golden-path seam are different
**What goes wrong:** the phase discusses rollback as if all command families
share the same route toggle.
**How to avoid:** explicitly note that `watch` and `skills` already run as sole
route, while the older legacy env seam applies elsewhere.

### Pitfall 3: Leaving the seam in place without saying why
**What goes wrong:** the exact inertia this phase was meant to prevent.
**How to avoid:** record the reason for retention and the future trigger for
re-evaluation.
</common_pitfalls>

## Validation Architecture

Use a two-part validation model for this phase:

1. **Evidence synthesis proof**
   - the soak report points to specific verification artifacts and contract
     tests from the migrated phases
   - the report clearly states what those checks prove and what they do not

2. **Decision clarity proof**
   - project or milestone docs state whether the rollback seam is retained or
     retired
   - the rationale is explicit and discoverable
   - `ROL-06` is marked complete only once that decision is recorded

Recommended evidence targets for this phase:
- direct references to Phase 8-11 verification docs
- direct references to legacy/core parity tests and migrated contract tests
- `git diff --check`

<sources>
## Sources

### Primary (HIGH confidence)
- `.planning/phases/12-soak-evidence-rollback-decision/12-CONTEXT.md`
- `.planning/PROJECT.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/08-watch-runtime-hardening/08-VERIFICATION.md`
- `.planning/phases/09-secure-skill-materialization-atomic-activation/09-VERIFICATION.md`
- `.planning/phases/10-unified-scan-serve-enforcement/10-VERIFICATION.md`
- `.planning/phases/11-command-family-extraction-contract-gates/11-VERIFICATION.md`
- `tests/cli_integration.rs`

### Secondary (MEDIUM confidence)
- `.planning/research/ARCHITECTURE.md`
- `src/app/mod.rs`
- `src/app/golden_path.rs`
</sources>
