# Golden Path Contract

This document is the rewrite-focused source of truth for the maintained
`setup -> add -> status -> sync` lifecycle.

Use it when changing or migrating the internal implementation of the golden
path. It is narrower than the full product contract on purpose: the goal is to
capture the stable baseline that later rewrite phases must preserve while
moving orchestration out of `src/main.rs`.

Read it together with:

- [PRODUCT_CONTRACT.md](PRODUCT_CONTRACT.md) for the wider shipped support boundary
- [STABILITY.md](STABILITY.md) for the `1.x` compatibility rules
- [VALIDATION.md](VALIDATION.md) for the rewrite parity and release checks

## Scope

The stable rewrite target in this document is:

```text
setup -> add -> status -> sync
```

Across these commands, Phase 1 treats the following as contract-level behavior:

- command names and primary aliases stay stable
- `--root`, `--global`, and `--local` preserve install-scope semantics
- stable machine-readable top-level fields stay additive within `1.x`
- generated host artifacts and `.sxmc` state/profile behavior stay intact
- stdout/stderr and exit behavior do not silently drift during migration

## Shared Rules

### Install Scope

- project-local remains the default behavior
- `--global` writes host-facing artifacts into user-level locations and uses
  Sumac-managed global state
- `--local` is the explicit project-local override
- `--root` chooses the project root for local installs and conflicts with
  `--global`

### Host Selection

- `--host ...` is the primary host-selection flag
- `--client ...` remains a visible alias where supported
- auto-detection may use existing managed host files or installed host runtimes

### Machine-Readable Stability

For the structured outputs covered here, top-level shapes are stable in `1.x`
and should evolve additively.

### Generated Artifacts

The golden path may create or refresh:

- saved CLI profiles under `.sxmc/ai/profiles/` for local installs or the
  global Sumac config directory for global installs
- startup-facing markdown such as `CLAUDE.md`, `AGENTS.md`, and host-native
  equivalents
- mergeable client config fragments and sidecars for supported AI hosts
- `.sxmc/state.json` for sync state in the selected install scope

## setup

### Purpose

Onboard a set of tools in one pass and prepare host-facing docs/config for the
selected install scope.

### Stable Command Surface

- command: `sxmc setup`
- primary tool selector: `--tool ...`
- stable scope controls:
  - `--root`
  - `--global`
  - `--local`
- stable host selector:
  - `--host ...`
  - alias: `--client ...`
- stable behavior modifiers:
  - `--limit`
  - `--depth`
  - `--skills-path`
  - `--preview`
  - `--allow-low-confidence`
  - `--allow-self`
  - `--pretty`
  - `--format ...`

### Stable Structured Output

Structured `setup` output keeps these top-level fields stable:

- `command`
- `tools`
- `root`
- `install_scope`
- `effective_mode`
- `preview_requested`
- `auto_previewed_due_to_missing_hosts`
- `auto_detected_tools`
- `auto_detected_hosts`
- `hosts`
- `results`
- `outcome_summary`
- `recommended_command`

### Stable Behavioral Notes

- when no hosts are detected, `setup` may fall back to preview instead of
  writing host artifacts
- `setup --global` targets user-level host locations and global Sumac state
- generated outputs remain the same artifacts that `add` and `sync` later
  reconcile

## add

### Purpose

Onboard one additional CLI into saved profiles and host-facing docs/config.

### Stable Command Surface

- command: `sxmc add <tool>`
- stable scope controls:
  - `--root`
  - `--global`
  - `--local`
- stable host selector:
  - `--host ...`
  - alias: `--client ...`
- stable behavior modifiers:
  - `--depth`
  - `--skills-path`
  - `--preview`
  - `--allow-low-confidence`
  - `--allow-self`
  - `--pretty`
  - `--format ...`

### Stable Structured Output

Structured `add` output keeps these top-level fields stable:

- `command`
- `tool`
- `root`
- `install_scope`
- `effective_mode`
- `preview_requested`
- `auto_previewed_due_to_missing_hosts`
- `auto_detected_hosts`
- `hosts`
- `profile`
- `outcomes`
- `outcome_summary`
- `recommended_command`

### Stable Behavioral Notes

- `add` writes or refreshes the saved profile for the target CLI
- `add` updates managed host artifacts for the selected or detected hosts
- when no hosts are detected, `add` may preview instead of applying writes
- `add --global` writes user-level host artifacts and global saved profiles

## status

### Purpose

Expose the current onboarding and reconciliation state as the maintained
machine-readable source of truth.

### Stable Command Surface

- command: `sxmc status`
- stable scope controls:
  - `--root`
  - `--global`
  - `--local`
- stable host selector:
  - `--only ...`
  - alias: `--host ...`
- stable comparison/health flags:
  - `--compare-hosts ...`
  - `--health`
  - `--exit-code` (requires `--health`)
  - `--human`
  - `--pretty`
  - `--format ...`

### Stable Structured Output

Structured `status` output is additive, but these top-level sections are part
of the maintained baseline:

- `install_scope`
- `startup_files`
- `cache`
- `saved_profiles`
- `sync_state`
- `ai_knowledge`
- `recovery_plan`

Additional stable conditional sections:

- `host_capabilities`
- `host_capability_diff` when `--compare-hosts` is used
- `baked_health` when `--health` is used

### Stable Behavioral Notes

- `status` extends the startup-file/health view with saved-profile drift and
  reconciliation status
- `ai_knowledge` and `recovery_plan` are maintained machine-readable recovery
  surfaces, not best-effort decorations
- `status --global` reports user-level target paths and global state locations
- `status --health --exit-code` is a CI-friendly gate and should not silently
  change its exit semantics inside `1.x`

## sync

### Purpose

Reconcile saved CLI profiles and derived host artifacts against the current
installed tool surfaces.

### Stable Command Surface

- command: `sxmc sync`
- stable scope controls:
  - `--root`
  - `--global`
  - `--local`
- stable host selector:
  - `--only ...`
  - alias: `--host ...`
- stable behavior modifiers:
  - `--skills-path`
  - `--apply`
  - `--check`
  - `--allow-low-confidence`
  - `--pretty`
  - `--format ...`

### Stable Structured Output

Structured `sync` output keeps these top-level fields stable:

- `command`
- `root`
- `install_scope`
- `mode`
- `state_path`
- `profile_dir`
- `host_ids`
- `profile_count`
- `changed_count`
- `unchanged_count`
- `blocked_count`
- `error_count`
- `profile_outcomes`
- `profile_outcome_summary`
- `artifact_outcomes`
- `artifact_outcome_summary`
- `entries`
- `sync_state`
- `recommended_command`

### Stable Behavioral Notes

- preview mode reports drift without mutating profiles or host artifacts
- `--apply` writes refreshed profiles, rewrites eligible host artifacts, and
  persists `.sxmc/state.json` for the selected scope
- `--check` is a drift gate and should continue to report non-zero when drift or
  blocking sync errors remain
- low-confidence profiles may refresh saved profiles without updating host
  artifacts unless `--allow-low-confidence` is used

## Phase 1 Usage

Later rewrite phases should treat this document as the contract baseline when
migrating `setup`, `add`, `status`, and `sync` onto the new core/app layer.

If code, tests, and this document disagree, fix the disagreement explicitly;
do not let the rewrite proceed on assumption alone.
