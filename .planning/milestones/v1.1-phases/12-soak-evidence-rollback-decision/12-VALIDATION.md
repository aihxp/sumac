# Phase 12: Soak Evidence & Rollback Decision - Validation Strategy

**Created:** 2026-04-04
**Phase:** 12-soak-evidence-rollback-decision

## Purpose

Define how Phase 12 proves that the rollback decision is based on explicit
evidence and is documented clearly enough for future maintainers to follow.

## Evidence Required

### 1. Soak Evidence Record

The phase must prove that:
- maintainers can point to a concrete evidence record covering the migrated
  `watch`, `skills`, and `serve` work
- the record cites specific verification artifacts and contract tests
- the record distinguishes local validation from true post-release soak

### 2. Explicit Rollback Decision

The phase must prove that:
- the status of the older rollback seam is explicitly retained or retired
- the rationale is documented and discoverable
- `watch` and `skills` are clearly described as sole-route migrated command
  families with evidence behind that decision

## Required Checks

- direct review of the soak-evidence report and decision notes
- confirmation that `ROL-06` is marked complete
- `git diff --check`

## Pass Conditions

- maintainers can point to an explicit soak-evidence artifact
- the rollback seam status is documented with rationale rather than inertia
- project and milestone state reflect the decision accurately

## Risks To Watch

- overstating the available soak evidence
- leaving the rollback seam status implicit
- documenting the decision only in a transient phase artifact and nowhere that
  maintainers will look later
