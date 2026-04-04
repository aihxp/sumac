# Phase 10: Unified Scan & Serve Enforcement - Research

**Researched:** 2026-04-04
**Domain:** Managed skill policy alignment across scanner and MCP-serving
paths
**Confidence:** HIGH

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- Scanner coverage must use the canonical managed asset inventory instead of
  the legacy compatibility views alone.
- Unreadable and non-UTF-8 managed files must produce explicit findings, not
  silent skips.
- MCP file listing and direct file access must expose only allowlisted managed
  assets.
- Broader `skills` command extraction remains out of scope for this phase.

### the agent's Discretion
- Exact helper layout for managed-asset filtering and finding generation.
- Exact finding messages and whether unreadable vs non-UTF-8 cases share or
  split codes.
- Whether the handler should build a small per-skill allowlist map on demand or
  scan `skill.assets` directly per request.

### Deferred Ideas (OUT OF SCOPE)
- Full `skills` service extraction and CLI contract gates
- Soak-based rollback review
- Larger policy-module reorganization beyond what Phase 10 needs
</user_constraints>

<research_summary>
## Summary

Phase 10 should finish the policy chain that Phases 7 and 9 established:
Phase 7 created the canonical managed asset inventory, and Phase 9 enforced it
for install/update. The remaining gap is that `skill_scanner.rs` still scans
the legacy compatibility views and silently skips unreadable assets, while
`server/handler.rs` still lists and serves any file found under a skill root.

The strongest implementation direction is to keep public MCP behavior stable
while changing the internals so both scanner and handler derive their file
surface from `Skill.assets`. For scanning, the phase should read managed
script/reference assets as bytes, then surface an explicit finding when a file
cannot be read or decoded as UTF-8. For serving, the handler should stop
walking the filesystem recursively and instead expose only allowlisted managed
relative paths from the parsed skill inventory.

**Primary recommendation:** land Phase 10 as two focused steps: first make the
scanner fail closed over canonical managed assets, then tighten the handler's
file resolution and listing helpers to the same managed asset set with
regression coverage.
</research_summary>

<standard_stack>
## Standard Stack

The established tools already present in this repo for this domain:

### Core
| Tool | Purpose | Why Standard Here |
|------|---------|-------------------|
| `src/security/skill_scanner.rs` | Security findings over skill content | Already owns scan reporting and finding generation |
| `Skill.assets` from `src/skills/models.rs` | Canonical managed asset inventory | Already unifies install/watch policy and should become the scan/serve source of truth |
| `std::fs::read` + `String::from_utf8` | Raw-byte asset reads with explicit UTF-8 handling | Matches the repo's current no-new-dependency approach |
| `src/server/handler.rs` | MCP file access and skill metadata reporting | Already owns the file-serving boundary this phase must tighten |

### Supporting
| Tool | Purpose | When to Use |
|------|---------|-------------|
| `.planning/research/ARCHITECTURE.md` | Canonical-asset design guidance | To keep install, scan, and serve inventories aligned |
| `.planning/research/STACK.md` | Raw-byte scanner and allowlist guidance | To avoid silent read/decode skips |
| `.planning/codebase/CONCERNS.md` | Repo-local security gap inventory | To make sure this phase closes the known drift |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Asset-derived scan/serve helpers | Ad hoc recursion in each module | Keeps policy drift alive and repeats the current bug shape |
| Raw-byte reads with explicit findings | `read_to_string` plus silent `continue` | Leaves unreadable or encoded assets unscanned |
| Managed relative-path allowlist | Canonicalized "inside base dir" checks only | Still exposes unmanaged files if they exist on disk |
</standard_stack>

<architecture_patterns>
## Architectural Patterns

### Pattern 1: Canonical Asset Policy Reuse
**What:** derive scanner and server decisions from `Skill.assets`.
**Why here:** Phase 10 is specifically about making installed, scanned, and
served inventories match.
**Recommended here:** yes, as the central policy rule.

### Pattern 2: Fail-Closed Managed Asset Scanning
**What:** read managed file assets as bytes and emit explicit findings for read
or UTF-8 decode failures.
**Why here:** `SEC-01` is about coverage gaps not being silent.
**Recommended here:** yes, for scripts and references that may be executed or
served later.

### Pattern 3: Allowlisted Relative Path Serving
**What:** resolve requested skill files only if their normalized relative path
matches a canonical managed asset entry.
**Why here:** base-dir containment checks alone do not express the product's
  safe serve surface.
**Recommended here:** yes, while preserving existing MCP tool names and return
shapes.
</architecture_patterns>

<dont_hand_roll>
## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Managed file policy | Another recursive file walker just for the handler | Reuse `Skill.assets` as the allowlist | The policy already exists and should not drift again |
| Scan coverage | Silent `read_to_string` fallthrough | Raw-byte reads with explicit findings | The phase requires surfaced failures |
| Serve restrictions | Hidden-file heuristics layered onto recursive listing | Explicit relative-path membership from canonical assets | The product boundary is allowlist-based, not heuristic-based |
</dont_hand_roll>

<common_pitfalls>
## Common Pitfalls

### Pitfall 1: Fixing scan coverage without tightening the server
**What goes wrong:** scans look correct, but unmanaged files remain listable or
readable over MCP.
**How to avoid:** treat scanner and handler as two halves of the same policy
phase and cover both with regressions.

### Pitfall 2: Using `skill.assets` for listing but not for direct reads
**What goes wrong:** `get_skill_details()` looks safe while
`get_skill_related_file()` can still reach unmanaged files by path.
**How to avoid:** make listing and path resolution share the same managed asset
lookup path.

### Pitfall 3: Treating decode failure as a clean scan
**What goes wrong:** binary or invalid UTF-8 managed assets still bypass scan
coverage.
**How to avoid:** emit a real finding for decode failures instead of assuming
text access succeeded.
</common_pitfalls>

## Validation Architecture

Use a two-part validation model for this phase:

1. **Scanner coverage proof**
   - managed script/reference assets are scanned from the canonical asset
     inventory
   - unreadable managed assets create explicit findings
   - invalid UTF-8 managed assets create explicit findings

2. **Serve enforcement proof**
   - skill file listings show only managed assets
   - direct file reads reject unmanaged relative paths even if the file exists
   - MCP resources and executable paths still work for allowlisted assets

Recommended evidence targets for this phase:
- scanner unit tests for unreadable and non-UTF-8 managed assets
- handler tests for managed-only listings and unmanaged-file denial
- full `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`,
  and `git diff --check`

<sources>
## Sources

### Primary (HIGH confidence)
- `.planning/phases/10-unified-scan-serve-enforcement/10-CONTEXT.md`
- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/STACK.md`
- `.planning/codebase/CONCERNS.md`
- `src/security/skill_scanner.rs`
- `src/server/handler.rs`
- `src/skills/models.rs`

### Secondary (MEDIUM confidence)
- `src/skills/install.rs` — source of the managed install boundary created in
  Phase 9
- `src/skills/parser.rs` — source of the canonical managed asset inventory
</sources>
