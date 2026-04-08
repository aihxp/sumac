# Phase 9: Secure Skill Materialization & Atomic Activation - Context

**Gathered:** 2026-04-04
**Status:** Ready for planning
**Mode:** Autonomous discuss (`--auto`)

<domain>
## Phase Boundary

Make `sxmc skills install` and `sxmc skills update` materialize local and
git-backed skills into ephemeral staging, validate only the managed allowlist,
and activate atomically so failures never leave a partial live install. This
phase is about lifecycle correctness and trust boundary narrowing inside the
install pipeline. It does not yet enforce served-file policy or broader
`skills` command extraction.

</domain>

<decisions>
## Implementation Decisions

### Materialization and staging
- **D-01:** Git-backed installs must resolve from the actual clone directory,
  including repo-root and repo-subpath forms, without leaking transient clone
  directories.
- **D-02:** Install and update should stage candidate contents in a temporary
  directory under the destination root, then activate with atomic rename or
  equivalent all-or-nothing replacement.
- **D-03:** A failed validation or copy step must leave the previous installed
  skill untouched.

### Allowlist and trust boundary
- **D-04:** The installed payload should be product-defined, not source-defined:
  `SKILL.md`, managed `scripts/**`, managed `references/**`, and explicit Sumac
  metadata only.
- **D-05:** Symlinks plus hidden, VCS, and build-artifact content should be
  rejected before activation instead of silently copied or ignored.
- **D-06:** The allowlist should align with the canonical managed asset model
  introduced in Phase 7 so later scan and serve work can reuse the same policy.

### Phase boundaries
- **D-07:** Keep this phase focused on install and update correctness. Serve
  enforcement and scanner fail-closed behavior belong to Phase 10.
- **D-08:** Broader `skills` command-family extraction belongs to Phase 11, so
  this phase should preserve stable entrypoints in `src/skills/install.rs`
  even if internal helpers are split.

### the agent's Discretion
- Whether to introduce new internal helper modules now or keep the phase inside
  `src/skills/install.rs`, as long as the staging, allowlist, and atomicity
  behavior become explicit and testable.
- Exact allowlist helper and rejection-report shapes.
- Test fixture strategy for git-backed installs and unsafe tree rejection.

</decisions>

<specifics>
## Specific Ideas

- Split the current pipeline into source materialization, staged payload
  construction, and activation rather than patching `copy_dir_recursive()` in
  place.
- Add real git-backed integration coverage so repo-root vs subpath resolution
  and cleanup are proven, not assumed.
- Preserve `.sxmc-source.json` as managed metadata written by Sumac rather than
  allowing arbitrary source metadata files through the install boundary.

</specifics>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone and phase contract
- `.planning/ROADMAP.md` — Phase 9 goal, dependency order, and success
  criteria.
- `.planning/REQUIREMENTS.md` — `SKILL-01`, `SKILL-02`, and `SKILL-03` define
  the required outcomes for this phase.
- `.planning/PROJECT.md` — milestone constraints and the product-preserving
  rewrite strategy.
- `.planning/STATE.md` — current milestone position after Phases 7 and 8.

### Milestone research
- `.planning/research/ARCHITECTURE.md` — recommended source/policy/staging
  split for the skills lifecycle pipeline.
- `.planning/research/STACK.md` — TempDir, system `git`, path validation, and
  staged activation guidance.
- `.planning/research/FEATURES.md` — maturity expectations for allowlisted
  install payloads and staged validation.
- `.planning/codebase/CONCERNS.md` — repo-local install bugs, trust-boundary
  gaps, and missing test coverage.

### Current implementation touchpoints
- `src/skills/install.rs` — current source resolution, clone materialization,
  recursive copy, metadata write, and activation logic.
- `src/server/handler.rs` — current served-file exposure that Phase 10 must
  later align to the same managed asset policy.
- `src/security/skill_scanner.rs` — later scan coverage that should ultimately
  consume the same allowlisted asset model.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `Skill.assets` from Phase 7 already defines the canonical managed asset
  inventory that this phase can reuse for payload selection.
- `tempfile::TempDir` is already in use in the install pipeline, so atomic
  staging does not require a new dependency or lifecycle model.
- Installed skill metadata already exists via `.sxmc-source.json`, which can
  remain the managed provenance artifact written during staging.

### Established Patterns
- The milestone prefers small internal seams over rewrites, so Phase 9 can keep
  `install_skill()` and `update_skills()` stable while splitting their helpers.
- Phase 8 established the pattern of fixing behavior with targeted regressions
  first, so install hardening should add end-to-end lifecycle tests rather than
  only unit checks.

### Integration Points
- `materialize_source_dir()` currently clones into `temp.path()/repo` but then
  resolves from `temp.keep()`, which breaks repo-root and repo-subpath
  resolution and leaks temp dirs.
- `copy_dir_recursive()` currently copies the full source tree without a
  managed-file allowlist or symlink rejection.
- `install_skill_into_target()` currently stages under the destination parent,
  but it still depends on the unchecked full-tree copy and removes the old
  target before final rename.

</code_context>

<deferred>
## Deferred Ideas

- Serve-time file restriction and scanner fail-closed enforcement — Phase 10.
- Broader `skills` command extraction and parity gates — Phase 11.
- Provenance previews and broader trust policy — later milestone work.

</deferred>

---

*Phase: 09-secure-skill-materialization-atomic-activation*
*Context gathered: 2026-04-04*
