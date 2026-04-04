# Phase 10: Unified Scan & Serve Enforcement - Validation Strategy

**Created:** 2026-04-04
**Phase:** 10-unified-scan-serve-enforcement

## Purpose

Define how Phase 10 proves that Sumac scans every managed file it may later
serve or execute, and exposes only that managed skill asset set through the MCP
server.

## Evidence Required

### 1. Managed Scan Coverage

The phase must prove that:
- managed script and reference assets are scanned from the canonical
  `Skill.assets` inventory
- unreadable managed files create explicit findings
- invalid UTF-8 managed files create explicit findings instead of being treated
  as clean or silently skipped

### 2. Managed Serve Enforcement

The phase must prove that only allowlisted managed assets are exposed through
skill file listing and direct file reads:
- `get_skill_details` reports only managed files
- `get_skill_related_file` rejects unmanaged files even when they exist inside
  the skill directory
- allowlisted managed files remain reachable through the expected MCP paths

## Required Checks

- focused scanner tests covering unreadable and non-UTF-8 managed assets
- focused handler tests covering managed-only listings and unmanaged-file
  denial
- `cargo test --quiet`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `git diff --check`

## Pass Conditions

- the installed, scanned, and served managed file inventories match for an
  active skill
- scanner coverage no longer silently skips unreadable or invalid UTF-8 managed
  assets
- MCP file listing and direct file access expose only allowlisted managed
  assets

## Risks To Watch

- updating only the scanner while leaving unmanaged files reachable through
  direct file reads
- allowing the handler to recurse the filesystem instead of trusting
  `Skill.assets`
- creating findings for read failures but still missing UTF-8 decode failures
