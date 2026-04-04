# Phase 12: Soak Evidence & Rollback Decision - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Autonomous discuss (`--auto`)

<domain>
## Phase Boundary

Turn the milestone's accumulated verification into an explicit soak-evidence
record and make a documented rollback-seam decision. This phase is not about
adding another migration seam or another code rewrite. It is about answering
two operational questions with evidence: whether the migrated `watch` and
`skills` route is safe to keep as the sole route now, and whether the older
golden-path rollback seam should be retired or intentionally retained pending
real release soak.

</domain>

<decisions>
## Implementation Decisions

### Evidence scope
- **D-01:** Phase 12 should summarize concrete evidence from completed phases:
  watch runtime regressions, skill install/serve/security hardening, command
  extraction contract tests, and full-suite validation.
- **D-02:** The evidence record must distinguish between strong test/contract
  proof and true post-release soak evidence; those are not the same.

### Rollback decision
- **D-03:** The migrated `watch` and `skills` route should be evaluated as the
  active sole route because no separate rollback seam was introduced for them.
- **D-04:** The older `SXMC_GOLDEN_PATH_ROUTE=legacy` seam must be either
  retired or explicitly retained with documented justification rather than
  drifting forward silently.
- **D-05:** If the available evidence is strong for migration correctness but
  insufficient for true release soak, retention with explicit justification is
  the safer documented outcome than premature seam removal.

### Phase boundaries
- **D-06:** This phase may update project and milestone docs to record the
  evidence and decision, but it should not invent new command behavior.
- **D-07:** Milestone archival and cleanup can follow once the decision is
  recorded and verified.

### the agent's Discretion
- Exact format of the soak-evidence record and decision summary.
- Which project-level docs should carry the final retained-or-retired decision,
  as long as the answer is discoverable and explicit.
- Whether the decision updates belong only in phase artifacts or also in
  `.planning/PROJECT.md` and milestone summary docs.

</decisions>

<specifics>
## Specific Ideas

- Create a concise soak evidence matrix that references the exact regression
  and contract checks already passing in Phases 8 through 11.
- Record that `watch` and `skills` are already running as the sole route with
  explicit contract evidence, while the golden-path legacy seam remains a
  separate older rollback path.
- Update project docs so maintainers can see whether the legacy seam is being
  retained intentionally or retired now.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone and phase contract
- `.planning/ROADMAP.md` — Phase 12 goal, dependency order, and success
  criteria.
- `.planning/REQUIREMENTS.md` — `ROL-06` defines the required outcome for this
  phase.
- `.planning/PROJECT.md` — current statement of the rollback seam and milestone
  intent.
- `.planning/STATE.md` — current milestone position after Phase 11.

### Evidence inputs
- `.planning/phases/08-watch-runtime-hardening/08-VERIFICATION.md`
- `.planning/phases/09-secure-skill-materialization-atomic-activation/09-VERIFICATION.md`
- `.planning/phases/10-unified-scan-serve-enforcement/10-VERIFICATION.md`
- `.planning/phases/11-command-family-extraction-contract-gates/11-VERIFICATION.md`
- `tests/cli_integration.rs` — concrete contract tests, including legacy/core
  parity for the golden path and real-fixture contract gates for the migrated
  command families.

### Current implementation touchpoints
- `src/app/mod.rs` and `src/app/golden_path.rs` — current rollback seam owner
  through `SXMC_GOLDEN_PATH_ROUTE`.
- `.planning/research/ARCHITECTURE.md` — guidance that parity tests, not more
  env-flag routing, should drive the rollback decision.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- The milestone already produced verification docs for each migrated slice, so
  Phase 12 does not need to reconstruct evidence from scratch.
- The repo already contains legacy/core parity tests for the golden-path
  commands and real-fixture contract tests for the migrated `skills` and
  `serve` flows.

### Established Patterns
- Earlier phases closed with explicit verification artifacts and requirement
  coverage notes. Phase 12 should treat the rollback decision the same way:
  explicit, inspectable, and tied to evidence.
- The architecture research explicitly warned against expanding env-flag route
  seams without evidence, so Phase 12 should favor documentation plus a clear
  decision over creating new toggles.

### Integration Points
- `SXMC_GOLDEN_PATH_ROUTE=legacy` still exists for the older onboarding
  commands, but `watch` and `skills` now run only through the migrated route.
- There is strong local verification evidence, but no repo-local proof of
  multi-release soak after Phase 11's extraction.

</code_context>

<deferred>
## Deferred Ideas

- Actual seam retirement after a later release window if this phase concludes
  that more soak time is still required.
- Broader milestone-summary polishing outside the specific rollback decision.

</deferred>

---

*Phase: 12-soak-evidence-rollback-decision*
*Context gathered: 2026-04-04*
