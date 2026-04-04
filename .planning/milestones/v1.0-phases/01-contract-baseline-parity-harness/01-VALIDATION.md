# Phase 1: Contract Baseline & Parity Harness - Validation Strategy

**Created:** 2026-04-04
**Phase:** 01-contract-baseline-parity-harness

## Purpose

Define how Phase 1 proves the rewrite baseline before any migrated logic
becomes the default code path.

## Evidence Required

### 1. Golden-Path Inventory

The phase must publish a maintained inventory for:
- `sxmc setup`
- `sxmc add`
- `sxmc status`
- `sxmc sync`

The inventory should cover:
- primary commands and aliases
- install-scope behavior (`--root`, `--global`, `--local`)
- stable machine-readable output fields
- generated files and state artifacts
- expected exit and stdout/stderr behavior

### 2. Executable Parity Checks

Parity must be proven with existing validation layers, extended where needed:
- `tests/cli_integration.rs`
- `scripts/test-sxmc.sh`
- existing portable smoke / release certification only where already relevant

### 3. Rewrite Reporting

Validation docs must describe:
- where the golden-path inventory lives
- which tests act as parity proof
- how later migration phases will extend the same reporting model

## Required Checks

- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `bash scripts/test-sxmc.sh --json /tmp/sxmc-phase01-results.json`

## Pass Conditions

- golden-path inventory artifact exists and is linked from maintained docs
- parity checks exist for `setup`, `add`, `status`, and `sync`
- validation docs mention rewrite parity explicitly
- no current stable behavior is loosened or undocumented during the process

## Risks To Watch

- inventory drifts away from actual tests
- parity checks only cover happy-path JSON and miss artifact generation
- rewrite reporting lives only in phase-local docs and never reaches maintained
  validation docs
