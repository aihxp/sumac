# Phase 7: Canonical Asset Inventory & Watch Parity - Validation Strategy

**Created:** 2026-04-04
**Phase:** 07-canonical-asset-inventory-watch-parity

## Purpose

Define how Phase 7 proves that Sumac uses one canonical managed skill asset
inventory and that nested asset changes are handled consistently across native
and polling watch behavior.

## Evidence Required

### 1. Canonical Asset Inventory

The phase must establish one managed asset definition that covers:
- `SKILL.md`
- recursively discovered managed files under `scripts/`
- recursively discovered managed files under `references/`

That model should be authoritative enough that later install, serve, and scan
hardening can build on it instead of re-defining the file surface again.

### 2. Polling Parity Checks

The phase must prove that changing a nested managed asset changes the polling
watch view of the world. At minimum, regression tests should cover:
- nested file changes under `scripts/**`
- nested file changes under `references/**`
- preservation of existing direct script/reference behavior where expected

### 3. No User-Facing Contract Drift

Phase 7 should stay internal. Validation should confirm that no CLI contract,
JSON surface, or served-skill behavior regresses while the model changes.

## Required Checks

- `cargo test --quiet skills::parser server::mod`
- targeted nested-asset regression tests that exercise the canonical asset
  inventory and polling fingerprint behavior
- any touched existing watch or skill parser tests remain green

## Pass Conditions

- canonical asset inventory exists in the skill model or parser layer
- nested managed assets are covered by tests, not just top-level files
- polling fingerprint behavior changes when nested managed assets change
- no current user-facing watch or skill parsing behavior regresses outside the
  intended parity fix

## Risks To Watch

- canonical inventory is introduced but polling still uses a different traversal
  rule
- nested assets are modeled correctly but existing direct script/reference
  callers break
- Phase 7 accidentally absorbs Phase 8 runtime hardening or later install/serve
  policy work
