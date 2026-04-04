# Phase 6: Setup Consolidation & Golden Path Closeout - Research

## Key Findings

- The Phase 5 onboarding service already holds the reusable host-selection and
  inspect/materialize flow, so `setup` only needs a dedicated orchestration
  service to complete the migration pattern.
- The cleanest phase closeout is to make `GoldenPathApp` dispatch-only across
  `setup`, `add`, `status`, and `sync`; that gives the repo a real new core/app
  layer rather than one final generic exception.
- Direct core-vs-legacy parity for `setup` should follow the `add` proof style:
  compare stable contract fields semantically instead of requiring identical
  temp-root paths.
- The command-family shims can be considered retired once no migrated command
  keeps its implementation in `src/app/golden_path.rs`, even though the
  top-level rollback route stays available until the documented release-soak
  criterion is met.

## Risks To Control

- `setup` touches multiple tools in one run, so parity checks must avoid false
- negatives from temp-root-specific output paths.
- The milestone should not overclaim rollback retirement; Phase 2 explicitly
  required one stable release cycle before removing the legacy route.
