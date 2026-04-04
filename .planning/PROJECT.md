# Sumac

## What This Is

Sumac (`sxmc`) is a Rust CLI that makes AI assistants stop guessing how local
tools, APIs, MCP servers, skills, and project surfaces work. It inspects real
interfaces, turns them into structured profiles and artifacts, and uses those
to generate host-facing docs, client config, MCP wrappers, and discovery
outputs from one installable binary.

This planning cycle is for a brownfield product-preserving rewrite. The product
surface stays broad and stable, but the internals are being rebuilt so Sumac
feels like a cleaner native system rather than an accumulated set of layers and
special cases.

## Current State

- milestone `v1.0` is complete
- the maintained golden path (`setup`, `add`, `status`, `sync`) now runs
  through dedicated service modules under `src/app/`
- `src/app/golden_path.rs` is now a thin dispatch seam
- the rollback seam remains intentionally available through
  `SXMC_GOLDEN_PATH_ROUTE=legacy` until the documented release-soak rule is met

See:
- [`.planning/v1.0-MILESTONE-AUDIT.md`](/Users/hprincivil/Projects/sxmc/.planning/v1.0-MILESTONE-AUDIT.md)
- [`.planning/milestones/v1.0-ROADMAP.md`](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.0-ROADMAP.md)
- [`.planning/milestones/v1.0-REQUIREMENTS.md`](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.0-REQUIREMENTS.md)

## Next Milestone Goals

- let one stable release cycle ship with the rollback seam intact
- decide whether to retire `SXMC_GOLDEN_PATH_ROUTE=legacy` after that soak
- choose the next subsystem family to migrate onto the same core/app pattern

## Core Value

Sumac must let AI systems understand and use real existing tools and interfaces
without bespoke glue, while staying fast, local-first, and reliable.

## Requirements

### Validated

- ✓ AI host onboarding and reconciliation exist as a shipped workflow through
  `setup -> add -> status -> sync` — existing
- ✓ CLI inspection produces structured profiles and generated startup artifacts
  for multiple AI hosts — existing
- ✓ MCP serving, wrapping, and inspection are core shipped product paths —
  existing
- ✓ Skills can be discovered, served, installed, updated, and executed —
  existing
- ✓ API, GraphQL, database, codebase, and traffic discovery are shipped product
  surfaces — existing
- ✓ Distribution and maintenance flows exist across crates.io, GitHub Releases,
  npm wrapper, Homebrew, and CI/release automation — existing

### Active

- [ ] Select the next rewrite milestone after the golden-path release soak
- [ ] Decide whether the rollback seam can be retired after one stable release
- [ ] Extend the proven migration pattern to the next subsystem family

### Out of Scope

- Greenfielding the product into a narrower or different product identity —
  this milestone preserves Sumac’s existing value proposition
- Breaking `1.x` CLI or JSON contracts during the rewrite — external stability
  matters more than internal convenience
- Pausing releases until the rewrite is complete — migration must coexist with
  active shipping
- Rebuilding every subsystem at once — the rewrite should progress by stable
  slices, not a parallel universe that never lands

## Context

Sumac is already a shipped `1.x` product with a wide surface area: CLI
inspection, AI-host materialization, MCP wrapping and serving, discovery
surfaces, skills lifecycle, bundle/registry flows, and cross-platform release
automation. The codebase map confirms that the core pain point is concentrated
in orchestration and boundary quality rather than product viability: `src/main.rs`
mixes command dispatch, watch logic, network/file helpers, and multiple
subcommand implementations in one hotspot, while adjacent subsystems like skill
install and watch/reload have grown harder to test and reason about cleanly.

The rewrite is therefore not about re-deciding what Sumac is. It is about
making the internals feel like a native product that had a coherent core from
day one: stable outside, greenfield inside, migrated slice by slice. The first
rewrite slice will start with the stable onboarding/reconciliation loop because
it is the maintained user path and touches the exact orchestration seams that
currently need the most cleanup.

## Constraints

- **Compatibility**: Public CLI behavior, JSON outputs, generated files, and
  release semantics must remain stable throughout the migration — Sumac is
  already a stable `1.x` product
- **Migration shape**: The rewrite should happen inside the existing repo as a
  cleaner internal core, not as a second product or a long-lived parallel code
  tree — slice-by-slice cutover is the strategy
- **Release cadence**: Releases continue during the rewrite — migration cannot
  freeze shipping
- **Product scope**: Sumac keeps the same broad product surface during this
  cycle — the rewrite is internal architecture work, not a product narrowing
- **Testing**: Each migrated slice must prove parity with the existing product
  path before old logic is retired

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Preserve the existing Sumac product while rewriting internals | The product already had validated value; the problem was internal coherence, not product-market direction | Complete in v1.0 |
| Start with `src/main.rs` and the golden onboarding path | This was the clearest orchestration hotspot and the maintained user workflow | Complete in v1.0 |
| Build a cleaner internal core/app layer inside the current repo | Enabled greenfield internals without a product reset or long-lived fork | Complete in v1.0 |
| Keep releases and `1.x` contracts stable during migration | Prevented the rewrite from becoming a trust-breaking freeze or compatibility reset | Complete in v1.0 |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `$gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `$gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-04-05 after v1.0 completion*
