# Phase 7: Canonical Asset Inventory & Watch Parity - Research

**Researched:** 2026-04-04
**Domain:** Canonical managed skill asset modeling and watch parity for a
shipped Rust CLI/MCP tool
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Keep this phase focused on one canonical managed skill asset inventory plus
  parity between native-event and polling watch modes.
- Treat `SKILL.md` plus recursively discovered managed files under `scripts/`
  and `references/` as the managed asset surface for this phase.
- Do not broaden this phase into notify/webhook runtime hardening, install
  allowlist enforcement, or broad command-family extraction.
- Surface degraded polling behavior explicitly instead of allowing silent
  misses.

### the agent's Discretion
- Exact Rust type shapes and helper/module placement for the canonical asset
  inventory.
- Whether to introduce a shared recursive traversal helper now or keep
  recursion in a local internal utility, as long as one authoritative asset
  definition exists.
- Exact fixture structure and test helper layout for proving parity.

### Deferred Ideas (OUT OF SCOPE)
- Async timeout and backpressure handling for watch side effects.
- Install/update hardening, atomic staging, and allowlist enforcement.
- Serve/scanner fail-closed enforcement over the managed asset set.
- Broader `watch`/`skills` command-family extraction.
</user_constraints>

<research_summary>
## Summary

Phase 7 should establish a single recursive skill asset inventory in
`src/skills/` and make `src/server/mod.rs` consume that inventory for polling
fingerprints and parity tests. The current mismatch is structural: parser logic
only indexes top-level files in `scripts/` and `references/`, while the MCP
server already allows recursive file access and the watch fingerprint only
hashes one directory level. That means nested managed files are part of the
effective serve surface but not part of the parser or polling model.

The strongest implementation direction is to make `Skill` own a canonical asset
collection, keep `scripts` and `references` as compatibility views derived from
that collection, and use the same recursive traversal rules for watch
fingerprinting. This phase does not need a new runtime or framework. The only
optional additive dependency is `walkdir`, but the primary value is the shared
definition, not the crate itself.

**Primary recommendation:** introduce one canonical recursive managed-asset
inventory under `src/skills/`, then use it to drive polling fingerprints and
parity tests that cover nested `scripts/` and `references/` changes.
</research_summary>

<standard_stack>
## Standard Stack

The established tools already present in this repo for this domain:

### Core
| Tool | Purpose | Why Standard Here |
|------|---------|-------------------|
| `src/skills/models.rs` | Skill representation | Natural home for a canonical asset inventory shared by parser, watch, and later install/serve/scan work |
| `src/skills/parser.rs` | Skill discovery/parsing | Already owns skill structure derivation, so it should stop emitting a shallow model |
| `src/server/mod.rs` | Watch reload and polling fingerprinting | Already owns the native-event and fallback watch logic that must become parity-aware |
| `src/server/handler.rs` | Served file access | Shows the real reachable skill file surface already extends beyond the parser's top-level model |
| `tests` + `tempfile` | Behavior proof | Existing project testing stack is enough to add nested-asset parity checks without new infrastructure |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `.planning/research/SUMMARY.md` | Milestone sequencing | To keep Phase 7 focused on the shared asset model and parity harness |
| `.planning/codebase/CONCERNS.md` | Repo-local failure map | To anchor the plan in known polling, serve-surface, and file-model mismatches |
| `notify 8.x` | Native and poll watch backends | To keep both paths inside the existing stack while aligning correctness |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Canonical asset inventory in `src/skills/` | Separate watch-only recursive scanner | Would duplicate file-model logic and reintroduce drift in later phases |
| Reusing current watch fingerprint helpers | Ad hoc nested hashing patch | Faster short-term, but still leaves parser and served-file models inconsistent |
| Local recursion utility | `walkdir` | `walkdir` is fine if it reduces duplication, but the authoritative asset definition matters more than the traversal crate |
</standard_stack>

<architecture_patterns>
## Architecture Patterns

### Pattern 1: Canonical inventory with derived views
**What:** store one recursive list of managed assets on `Skill`, then derive
`scripts` and `references` from it for existing callers.
**When to use:** when several subsystems need the same file-scope definition but
legacy call sites still expect narrower typed lists.
**Recommended here:** this lets Phase 7 fix watch correctness without forcing
Phase 9/10 to redesign the model again.

### Pattern 2: One fingerprint source, many backends
**What:** native-event and polling modes use different watch triggers but
compare against the same managed asset set.
**When to use:** when a fallback backend should degrade operationally, not
semantically.
**Recommended here:** keep `RecommendedWatcher` for events, but make polling
fingerprints walk the same canonical assets the parser/server already know.

### Pattern 3: Parity fixtures before runtime hardening
**What:** prove that nested asset changes are detected in both modes before
optimizing reload timing or side-effect isolation.
**When to use:** when correctness bugs are already known and later phases will
otherwise build on top of shaky assumptions.
**Recommended here:** add focused tests around nested managed assets before
touching broader watch runtime behavior in Phase 8.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Nested watch correctness | A watch-only alternate file discovery path | One canonical managed-asset inventory in `src/skills/` | Avoids further parser/watch/serve drift |
| Polling parity | A shallow hash patch over `scripts/` and `references/` top-level entries | Recursive asset enumeration from the canonical model | The issue is semantic drift, not just a missing hash call |
| Phase proof | Manual spot checks | Focused parity tests on nested asset changes | Later phases need deterministic evidence, not memory |
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Fixing polling without fixing the asset model
**What goes wrong:** nested asset changes pass one specific test but parser and
served-file semantics still diverge.
**How to avoid:** make Phase 7's first deliverable the canonical managed asset
inventory, not a localized watch patch.

### Pitfall 2: Treating recursive access as implementation detail
**What goes wrong:** the server continues serving files the parser and watch
logic do not understand as managed assets.
**How to avoid:** treat recursive file reachability as part of the managed
surface that later install/scan/serve policy will formalize.

### Pitfall 3: Bundling runtime performance work into the parity phase
**What goes wrong:** correctness fixes get mixed with webhook timeouts, async
isolation, and command extraction, making regressions harder to classify.
**How to avoid:** keep this phase on inventory modeling, fingerprint parity,
and proof fixtures only.
</common_pitfalls>

## Validation Architecture

Use a two-part validation model for this phase:

1. **Canonical managed asset proof**
   - parser- or model-level tests proving nested `scripts/` and `references/`
     files become part of the managed inventory
   - compatibility assertions that existing `scripts`/`references` views still
     expose the expected direct entries

2. **Watch parity proof**
   - polling fingerprint tests that change nested managed assets and observe a
     fingerprint delta
   - watch-facing tests or focused helpers showing the same nested changes are
     treated as managed assets by both watch backends or trigger explicit
     degraded-state handling

Recommended evidence targets for this phase:
- canonical asset inventory types and parser coverage
- regression tests for nested managed-asset fingerprint changes
- no user-facing CLI or JSON drift while internal modeling changes

<open_questions>
## Open Questions

1. **Should `SKILL.md` be represented inside the canonical asset inventory or
remain a separate invariant?**
   - What we know: it is always part of the managed skill state and affects
     watch reload behavior.
   - Recommendation: include it in the inventory as a distinguished asset kind
     or ensure the canonical fingerprinting path handles it through the same
     abstraction.

2. **Should Phase 7 add `walkdir` now?**
   - What we know: the milestone research treats it as optional, not required.
   - Recommendation: only add it if the implementation would otherwise duplicate
     recursive traversal in more than one place during this phase.
</open_questions>

<sources>
## Sources

### Primary (HIGH confidence)
- `.planning/phases/07-canonical-asset-inventory-watch-parity/07-CONTEXT.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/research/SUMMARY.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/PITFALLS.md`
- `.planning/codebase/CONCERNS.md`
- `src/skills/models.rs`
- `src/skills/parser.rs`
- `src/server/mod.rs`
- `src/server/handler.rs`

### Secondary (MEDIUM confidence)
- `src/security/skill_scanner.rs` — future alignment constraint for later
  phases, useful to avoid inventing a file model that scanner work cannot reuse
- `notify` crate behavior already captured in milestone research — confirms the
  parity requirement should be semantic rather than backend-specific
</sources>
