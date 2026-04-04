# Phase 10: Unified Scan & Serve Enforcement - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Autonomous discuss (`--auto`)

<domain>
## Phase Boundary

Make Sumac's skill scanner and MCP-serving paths use one enforced managed asset
policy so installed, scanned, and served skill inventories match. This phase is
about closing the trust-boundary drift left after Phase 9: scanner coverage
must consume the canonical managed asset inventory, and serve-time file access
must stop exposing arbitrary files that happen to live under a skill root. It
does not yet do the broader `skills` command extraction or the final rollback
decision work.

</domain>

<decisions>
## Implementation Decisions

### Scanner coverage and fail-closed behavior
- **D-01:** `scan_skill()` must walk the canonical managed asset inventory from
  `Skill.assets`, not just the legacy top-level `scripts` and `references`
  compatibility views.
- **D-02:** Unreadable and non-UTF-8 managed script/reference files must
  produce explicit findings instead of being silently skipped.
- **D-03:** The prompt body and frontmatter checks remain in scope, but Phase
  10's new guarantees apply specifically to managed file assets that may later
  be executed or served.

### Serve-time enforcement
- **D-04:** `get_skill_related_file()` and skill file listings must expose only
  allowlisted managed assets, not arbitrary files under `skill.base_dir`.
- **D-05:** Serve-time resolution should reuse the canonical managed asset
  inventory rather than recursing the filesystem independently.
- **D-06:** References exposed through MCP resources should continue to work,
  but the server's ad hoc file APIs must be narrowed to the same managed file
  set used by install and scan.

### Phase boundaries
- **D-07:** Phase 10 should tighten policy and consistency, not refactor the
  whole `skills` command family. Broader service extraction belongs to Phase
  11.
- **D-08:** This phase should preserve the current outward MCP tool/resource
  names and general success/error shapes while tightening what they can reach.

### the agent's Discretion
- Whether to implement the shared allowlist policy through small helper
  functions in `server/handler.rs` and `skill_scanner.rs` now or introduce a
  dedicated shared policy module later.
- Exact finding codes/messages for unreadable vs non-UTF-8 managed assets.
- Whether SKILL.md should participate in the managed file listing and direct
  file access helpers, as long as the installed, scanned, and served
  inventories stay consistent.

</decisions>

<specifics>
## Specific Ideas

- Replace recursive filesystem listing in the MCP handler with an asset-derived
  inventory from `skill.assets`.
- Add explicit scanner regressions for unreadable assets and invalid UTF-8 so
  the phase proves fail-closed coverage instead of assuming it.
- Keep resource exposure reference-specific, but make `get_skill_details`
  report only the same managed files the server can actually serve.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone and phase contract
- `.planning/ROADMAP.md` — Phase 10 goal, dependency order, and success
  criteria.
- `.planning/REQUIREMENTS.md` — `SEC-01` and `SEC-02` define the required
  outcomes for this phase.
- `.planning/PROJECT.md` — milestone constraints and the product-preserving
  rewrite strategy.
- `.planning/STATE.md` — current milestone position after Phase 9.

### Milestone research
- `.planning/research/ARCHITECTURE.md` — canonical asset inventory and
  scan/serve alignment guidance.
- `.planning/research/STACK.md` — raw-byte scan guidance and allowlist
  enforcement expectations.
- `.planning/research/FEATURES.md` — product expectations for secure skill
  serving.
- `.planning/codebase/CONCERNS.md` — repo-local gaps around unmanaged file
  serving and silent scan skips.

### Current implementation touchpoints
- `src/security/skill_scanner.rs` — current scan coverage over body,
  frontmatter, and compatibility views that still skip unreadable files.
- `src/server/handler.rs` — current file listing, relative file access, and
  reference resource reads for the skill MCP server.
- `src/skills/models.rs` — canonical managed asset model introduced in Phase 7.
- `src/skills/install.rs` — Phase 9's staged allowlist boundary that Phase 10
  should now match at scan and serve time.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Skill.assets` already describes the canonical managed asset inventory with
  relative paths and asset kinds.
- `skill.scripts` and `skill.references` still exist as compatibility views,
  which Phase 10 can preserve for outward behavior while stopping them from
  being the primary policy source.
- The handler already has focused helpers like `resolve_skill_file_path()` and
  `list_skill_files()`, so serve enforcement can land without changing the
  handler's public API shape.

### Established Patterns
- Phase 9 hardened install by reusing the canonical managed asset model rather
  than inventing a second allowlist; Phase 10 should extend that same policy to
  scan and serve.
- The milestone favors targeted regressions over broad rewrites, so explicit
  unreadable/non-UTF-8 scanner tests and unmanaged-file serve denials are the
  proof points to add.

### Integration Points
- `scan_skill()` currently scans body and frontmatter, then iterates only
  `skill.scripts` and `skill.references`, silently continuing on read failure.
- `get_skill_details()` currently reports `list_skill_files(&skill.base_dir)?`,
  which recursively lists arbitrary files under the skill root.
- `get_skill_related_file()` currently canonicalizes any relative file inside
  the skill directory, which means unmanaged files remain reachable if they are
  present on disk.

</code_context>

<deferred>
## Deferred Ideas

- Broader `skills` command-family extraction and contract gating — Phase 11.
- Rollback seam retirement and soak evidence review — Phase 12.
- Additional policy centralization into a shared `skills::policy` module if the
  current helper-level changes prove insufficient.

</deferred>

---

*Phase: 10-unified-scan-serve-enforcement*
*Context gathered: 2026-04-04*
