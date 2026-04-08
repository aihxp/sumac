# Phase 9: Secure Skill Materialization & Atomic Activation - Validation Strategy

**Created:** 2026-04-04
**Phase:** 09-secure-skill-materialization-atomic-activation

## Purpose

Define how Phase 9 proves that skill installs and updates materialize safely,
activate atomically, and reject unsafe payload contents before the skill
becomes live.

## Evidence Required

### 1. Correct Source Materialization

The phase must prove that:
- local path installs still work
- git repo root installs resolve from the actual clone root
- git repo subpath installs resolve the intended nested skill directory
- transient clone directories are scoped and not leaked into the final install

### 2. Managed Allowlist Enforcement

The phase must prove that only Sumac-managed skill contents are staged for
activation, and that unsafe content is rejected explicitly:
- symlinks
- hidden files and directories
- VCS directories like `.git`
- obvious build-artifact directories or files

### 3. Atomic Activation

Validation must confirm that a failed install or update attempt never leaves a
partially activated skill in place and does not destroy the previously active
skill.

## Required Checks

- focused install lifecycle tests covering local, git root, and git subpath
  sources
- rejection tests for unsafe payload trees
- an update failure regression proving the prior installed skill remains intact
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`

## Pass Conditions

- git and local source materialization resolve to the correct skill directory
- only the managed allowlist is staged and activated
- unsafe payload contents are rejected before activation
- failed updates preserve the previous installed skill contents

## Risks To Watch

- fixing git resolution without fixing allowlist copying
- staging the right payload but still deleting the live target too early
- relying on parser-level tests without end-to-end install/update coverage
