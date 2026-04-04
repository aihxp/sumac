# Phase 9: Secure Skill Materialization & Atomic Activation - Research

**Researched:** 2026-04-04
**Domain:** Safe skill install/update lifecycle hardening for a shipped Rust
CLI
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Git-backed and local installs must materialize into ephemeral staging and
  activate atomically.
- The install payload must be allowlisted to Sumac-managed skill contents,
  with symlinks plus hidden, VCS, and build-artifact content rejected before
  activation.
- This phase should preserve stable install/update entrypoints while fixing
  lifecycle behavior internally.
- Serve-time enforcement and broader command-family extraction are out of scope
  for this phase.

### the agent's Discretion
- Exact helper structure for source materialization, payload selection, and
- activation.
- How aggressively to factor the current `install.rs` file now versus later.
- Exact rejection messages and test-fixture structure.

### Deferred Ideas (OUT OF SCOPE)
- Serve-only allowlist enforcement in `server/handler.rs`
- Scanner fail-closed enforcement for unreadable or non-UTF-8 files
- Broad `skills` family service extraction
</user_constraints>

<research_summary>
## Summary

Phase 9 should turn the current install pipeline into three explicit steps:
materialize source, build a validated staged payload, then activate atomically.
The main correctness bugs are already known in the repo: git installs resolve
from the wrong directory because `materialize_source_dir()` uses `temp.keep()`
instead of the clone root, the pipeline copies whole trees rather than a
managed allowlist, and failures can remove or partially replace the active
target because activation still depends on an unchecked recursive copy.

The strongest implementation direction is to keep `install_skill()` and
`update_skills()` as stable entrypoints while introducing focused helpers for
source materialization, staged payload construction, and atomic replacement.
The canonical managed asset inventory from Phase 7 provides the right payload
source of truth. This phase does not need new infrastructure: the repo already
has `tempfile`, system `git`, path validation primitives, and enough test
tooling to build temp git repositories and unsafe tree fixtures.

**Primary recommendation:** fix git/local source materialization and atomic
activation first, then enforce the managed-file allowlist plus rejection rules
in the staged copy path with end-to-end tests.
</research_summary>

<standard_stack>
## Standard Stack

The established tools already present in this repo for this domain:

### Core
| Tool | Purpose | Why Standard Here |
|------|---------|-------------------|
| `src/skills/install.rs` | Stable lifecycle entrypoints | Natural place to keep install/update API stable while extracting helpers |
| `Skill.assets` from `src/skills/models.rs` | Canonical managed asset inventory | Already represents the managed payload Phase 9 should install |
| `tempfile::TempDir` | Ephemeral staging and clone cleanup | Already in the repo and directly suited to scoped materialization |
| `std::process::Command` with system `git` | Git clone materialization | Repo research already prefers keeping system `git` for v1.1 |
| `assert_cmd` + `tempfile` + real git fixtures | Install lifecycle proof | Existing test stack can cover root/subpath resolution and unsafe payload rejection |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `.planning/research/ARCHITECTURE.md` | Lifecycle split guidance | To keep source/policy/staging ownership clear |
| `.planning/research/STACK.md` | TempDir and allowlist guidance | To anchor activation and cleanup behavior |
| `.planning/codebase/CONCERNS.md` | Repo-local install bug map | To make sure the phase closes the known lifecycle gaps |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Scoped TempDir + system `git` | `git2` and custom clone APIs | Adds complexity without solving the known lifecycle bugs |
| Managed allowlist from canonical assets | Recursive tree copy plus ignore rules | Keeps the trust boundary source-defined and unsafe |
| Atomic stage-then-rename activation | In-place overwrite and cleanup | Makes partial installs much more likely on failure |
</standard_stack>

<architecture_patterns>
## Architectural Patterns

### Pattern 1: Source → Policy → Staging Pipeline
**What:** Separate source materialization, payload selection, and activation
into explicit helpers.
**Why here:** The current pipeline couples clone resolution, recursive copy,
and activation so tightly that bugs survive without focused tests.
**Recommended here:** yes, even if helpers stay in one file for now.

### Pattern 2: Canonical Managed Payload
**What:** Install only what Sumac already defines as the managed skill asset
surface, plus explicit managed metadata.
**Why here:** Phase 7 already created the canonical asset inventory; Phase 9 is
the first place that inventory should become a trust boundary.
**Recommended here:** yes, as the payload selection rule.

### Pattern 3: Atomic Replacement with Validation Before Activation
**What:** Build the next install completely in staging, validate it, then swap
it into place in one activation step.
**Why here:** `SKILL-03` is specifically about failed installs never leaving a
partial active skill behind.
**Recommended here:** yes, with end-to-end tests that prove old installs
survive failed updates.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Remote skill acquisition | A new git abstraction or custom downloader | System `git` in a scoped `TempDir` | The bug is lifecycle handling, not missing clone capability |
| Install trust boundary | Recursive copy plus ad hoc exclusions | Explicit allowlist from canonical managed assets | Security policy should be product-defined |
| Failure handling | In-place overwrite and best-effort cleanup | Validate in staging, then atomically activate | Prevents partial installs and preserves old targets on failure |
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Fixing git resolution without narrowing the payload
**What goes wrong:** git-backed installs start working but still copy `.git`,
build output, or hidden files into active skills.
**How to avoid:** make payload selection and rejection rules first-class, not
an afterthought.

### Pitfall 2: Treating hidden/VCS/build files as ignorable instead of invalid
**What goes wrong:** unsafe content disappears silently from one code path but
is still present or reachable in others.
**How to avoid:** reject unsafe payload contents explicitly during staging and
surface a real install failure.

### Pitfall 3: Deleting the live install too early
**What goes wrong:** a copy or metadata failure leaves no working skill at all.
**How to avoid:** build and validate the full replacement in staging before the
existing target is touched.
</common_pitfalls>

## Validation Architecture

Use a two-part validation model for this phase:

1. **Materialization and activation proof**
   - local path, git repo root, and git repo subpath installs resolve to the
     intended skill directory
   - transient clone materialization is scoped and not persisted by default
   - failed updates leave the previous installed skill intact

2. **Allowlist and rejection proof**
   - only managed assets plus managed metadata are copied into the staged skill
   - symlinks, hidden files, VCS directories, and build artifacts are rejected
     before activation
   - update/install entrypoints keep their existing outward behavior

Recommended evidence targets for this phase:
- end-to-end install/update tests using temp git repos
- explicit rejection tests for unsafe skill trees
- atomic activation proof showing old targets survive failures

<sources>
## Sources

### Primary (HIGH confidence)
- `.planning/phases/09-secure-skill-materialization-atomic-activation/09-CONTEXT.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/STACK.md`
- `.planning/research/FEATURES.md`
- `.planning/codebase/CONCERNS.md`
- `src/skills/install.rs`
- `src/skills/models.rs`
- `src/skills/parser.rs`

### Secondary (MEDIUM confidence)
- `src/server/handler.rs` — later serve-time consumer of the same managed
  payload policy
- `src/security/skill_scanner.rs` — later validation consumer of the same
  managed payload policy
</sources>
