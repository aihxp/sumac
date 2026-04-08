# Sumac

## What This Is

Sumac (`sxmc`) is a Rust CLI that makes AI assistants stop guessing how local
tools, APIs, MCP servers, skills, and project surfaces work. It inspects real
interfaces, turns them into structured profiles and artifacts, and uses those
to generate host-facing docs, client config, MCP wrappers, and discovery
outputs from one installable binary.

The current product-preserving rewrite has now shipped two milestones. The
external `1.x` surface stays broad and stable, while the internals keep moving
toward clearer service seams, stronger policy enforcement, and more explicit
operational evidence.

## Current State

- milestones `v1.0` and `v1.1` are complete and archived
- the maintained golden path (`setup`, `add`, `status`, `sync`) runs through
  dedicated service modules under `src/app/`
- `watch`, `skills`, and CLI-facing `serve` now also run through dedicated app
  seams with real-fixture contract coverage
- managed skill assets are now the canonical policy surface across install,
  scan, serve, and watch invalidation flows
- the remaining rollback seam is intentionally limited to the older
  golden-path route via `SXMC_GOLDEN_PATH_ROUTE=legacy` pending a later
  release-soak retirement decision
- the live planning space is reset and ready for the next milestone definition

See:
- [`.planning/milestones/v1.0-ROADMAP.md`](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.0-ROADMAP.md)
- [`.planning/milestones/v1.0-REQUIREMENTS.md`](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.0-REQUIREMENTS.md)
- [`.planning/v1.0-MILESTONE-AUDIT.md`](/Users/hprincivil/Projects/sxmc/.planning/v1.0-MILESTONE-AUDIT.md)
- [`.planning/milestones/v1.1-ROADMAP.md`](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.1-ROADMAP.md)
- [`.planning/milestones/v1.1-REQUIREMENTS.md`](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.1-REQUIREMENTS.md)
- [`.planning/milestones/v1.1-MILESTONE-AUDIT.md`](/Users/hprincivil/Projects/sxmc/.planning/milestones/v1.1-MILESTONE-AUDIT.md)

## Next Milestone Goals

- add operator-facing diagnostics for watch backend health, degraded mode, and
  last successful reload
- expose provenance previews and resulting serve-surface summaries before
  remote skill activation
- enforce origin and pinning policy for remote skill installs
- extend parity-diff and contract tooling beyond `watch` and `skills`
- continue reducing remaining top-level orchestration hotspots without
  breaking the stable `1.x` product surface

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
- ✓ `watch` stays responsive and accurate under polling, notifications, and
  long-running automation use — v1.1
- ✓ git-backed and remote skill installs are reliable, constrained to safe
  file sets, and cleaned up correctly — v1.1
- ✓ served skill contents follow a predictable allowlist and scanner coverage
  no longer silently skips risky files — v1.1
- ✓ the next non-golden-path command families moved onto clearer module and
  service boundaries without breaking `1.x` CLI behavior — v1.1
- ✓ the rollback seam is now governed by explicit soak evidence rather than
  assumption, and is retained intentionally where retirement is not yet earned
  — v1.1

### Active

- [ ] operators can inspect watch backend health, last successful reload, and
  degraded-mode reasons without digging through ad hoc logs
- [ ] remote skill installs can preview provenance and the resulting serve
  surface before activation
- [ ] remote skill installs can enforce origin and pinning policy
- [ ] parity-diff and contract tooling extends beyond `watch` and `skills`
- [ ] remaining orchestration-heavy command families keep moving out of
  top-level dispatch hotspots via the same typed service pattern

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

With v1.1 shipped, that pattern now covers the golden path plus the most
operationally sensitive adjacent surfaces: `watch`, `skills`, and CLI-facing
`serve`. The codebase now has a canonical managed asset model, staged skill
activation, unified scan-and-serve policy, and clearer command-family seams
backing those flows.

The next milestone does not need to re-open the same migration questions. It
can build on the shipped seams by improving operator visibility, tightening
remote provenance policy, and extending parity tooling to the next slice of
orchestration-heavy behavior.

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
- **Security exposure**: Installed and served skills must expose only intended
  contents — remote convenience cannot quietly widen the readable surface

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Preserve the existing Sumac product while rewriting internals | The product already had validated value; the problem was internal coherence, not product-market direction | Complete in v1.0 |
| Start with `src/main.rs` and the golden onboarding path | This was the clearest orchestration hotspot and the maintained user workflow | Complete in v1.0 |
| Build a cleaner internal core/app layer inside the current repo | Enabled greenfield internals without a product reset or long-lived fork | Complete in v1.0 |
| Keep releases and `1.x` contracts stable during migration | Prevented the rewrite from becoming a trust-breaking freeze or compatibility reset | Complete in v1.0 |
| Use v1.1 to harden `watch`, `skills`, and top-level orchestration together | These were the clearest remaining risk surfaces and they overlapped in reliability, security, and dispatch complexity | Complete in v1.1 |
| Use the canonical managed asset inventory as the shared policy contract | Watch invalidation, staged install, scan coverage, and serving all needed one authoritative asset surface | Complete in v1.1 |
| Enforce skill activation through scoped staging and allowlisted payloads | Remote convenience could not justify partial activation or silent policy widening | Complete in v1.1 |
| Keep migrated `watch` / `skills` as the sole route, but retain `SXMC_GOLDEN_PATH_ROUTE=legacy` intentionally until later release soak | Local verification and contract evidence are strong for the migrated route, but same-session validation is not the same as post-release soak for final seam retirement | Complete in v1.1 Phase 12 |

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
*Last updated: 2026-04-08 after v1.1 milestone archive*
