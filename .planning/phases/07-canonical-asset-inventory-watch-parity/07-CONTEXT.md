# Phase 7: Canonical Asset Inventory & Watch Parity - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Autonomous discuss (`--auto`)

<domain>
## Phase Boundary

Define one canonical managed skill asset inventory and use it as the source of
truth for watch invalidation across native-event and polling modes. This phase
establishes correctness for nested managed assets and parity between watch
backends; it does not yet tackle webhook timeouts, install allowlist
enforcement, or broad command-family extraction.

</domain>

<decisions>
## Implementation Decisions

### Canonical managed asset surface
- **D-01:** The canonical inventory for this phase should include `SKILL.md`
  plus recursively discovered managed files under `scripts/` and
  `references/`.
- **D-02:** The inventory should be represented once in `src/skills/` and then
  reused by parsing, watch fingerprinting, and later phases for install, serve,
  and scan policy.
- **D-03:** This phase should treat nested files as first-class managed assets;
  top-level-only hashing or indexing is not acceptable for parity.

### Watch parity expectations
- **D-04:** Native-event and polling modes must observe the same managed asset
  set and produce the same correctness outcome for nested changes.
- **D-05:** When polling cannot match native-event behavior exactly, Sumac must
  surface an explicit degraded-state or rescan path instead of silently missing
  changes.
- **D-06:** Phase 7 should prioritize correctness and parity harnesses over
  optimization; incremental reload classification belongs later.

### Phase boundaries
- **D-07:** Keep this phase focused on inventory modeling, fingerprinting, and
  parity tests. Async notify-command/webhook isolation is Phase 8 work.
- **D-08:** Do not broaden this phase into install allowlist enforcement or
  serve filtering beyond what is necessary to define the canonical managed
  asset inventory; those hardening steps belong to Phases 9 and 10.

### the agent's Discretion
- Exact Rust type shapes for the canonical asset inventory.
- Whether to introduce a shared recursive traversal helper now or keep it local
  to the phase implementation, as long as one managed-asset definition becomes
  authoritative.
- Test fixture naming, helper placement, and internal module boundaries used to
  prove native/polling parity.

</decisions>

<specifics>
## Specific Ideas

- Prefer one recursive managed-asset definition over parallel notions of
  "scripts", "references", and "files on disk".
- Make parity visible with fixtures that mutate nested managed assets and prove
  the same detection result in both watch modes.
- Avoid bundling runtime performance work into this phase unless a correctness
  fix strictly requires it.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone and phase contract
- `.planning/ROADMAP.md` — Phase 7 goal, dependency order, and success
  criteria.
- `.planning/REQUIREMENTS.md` — `WATCH-01` and `WATCH-02` define the required
  user-visible outcomes for this phase.
- `.planning/PROJECT.md` — milestone constraints, compatibility rules, and the
  product-preserving rewrite strategy.
- `.planning/STATE.md` — current milestone position and prior migration
  decisions to preserve.

### Milestone research
- `.planning/research/SUMMARY.md` — recommended sequencing and the reasons
  canonical asset inventory comes first.
- `.planning/research/ARCHITECTURE.md` — recommended ownership split between
  `src/skills/` and `src/server/`.
- `.planning/research/PITFALLS.md` — watch-contract drift and file-model
  divergence to avoid.
- `.planning/codebase/CONCERNS.md` — repo-specific bugs and fragility around
  shallow polling, duplicate reload builds, and mismatched file handling.

### Current implementation touchpoints
- `src/skills/parser.rs` — current top-level-only script/reference discovery.
- `src/server/mod.rs` — current watch reload path, fingerprinting, and polling
  behavior.
- `src/server/handler.rs` — current recursive file exposure behavior that
  already reaches beyond parser indexing.
- `src/security/skill_scanner.rs` — current scan model that later phases must
  align with the same managed asset inventory.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `src/skills/parser.rs`: already owns `parse_skill`, so it is the natural
  place to centralize canonical skill asset discovery instead of duplicating
  traversal logic in watch code.
- `src/server/mod.rs`: already contains the polling fingerprint and reload
  plumbing, so Phase 7 can improve correctness there without moving the full
  runtime seam yet.
- `src/server/handler.rs`: already resolves recursive file paths for MCP reads,
  which is useful evidence that the current serve surface is broader than the
  parser model.

### Established Patterns
- `src/app/` is the preferred destination for later command-family seams, but
  this phase can stay mostly inside `src/skills/` and `src/server/` because the
  requirement is parity, not command extraction.
- The milestone keeps `1.x` CLI and JSON behavior stable, so Phase 7 should
  favor additive internal modeling and tests over user-facing contract changes.

### Integration Points
- `compute_skill_fingerprint()` and `hash_directory_files()` in
  `src/server/mod.rs` are the current polling integration points that need a
  canonical managed-asset source.
- `scan_scripts()` and `scan_references()` in `src/skills/parser.rs` currently
  stop at one directory level and therefore do not match recursive server file
  access.
- Future phases depend on this inventory model to align install, scan, and
  serve policy, so the chosen representation should be reusable outside watch.

</code_context>

<deferred>
## Deferred Ideas

- Async timeout handling for notify commands and webhooks — Phase 8.
- Install allowlist enforcement, git materialization cleanup, and atomic
  activation — Phase 9.
- Enforced served/scanned inventory alignment and fail-closed scanner behavior
  — Phase 10.
- Broader `watch` and `skills` command-family extraction — Phase 11.

</deferred>

---

*Phase: 07-canonical-asset-inventory-watch-parity*
*Context gathered: 2026-04-04*
