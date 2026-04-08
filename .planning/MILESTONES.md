# Project Milestones: Sumac

## v1.1 Platform Hardening and Core Expansion (Shipped: 2026-04-08)

**Delivered:** Product-preserving hardening for `watch`, `skills`, and
CLI-facing `serve`, backed by a canonical managed asset policy and an explicit
rollback governance decision.

**Phases completed:** 7-12 (12 plans total, 21 tasks)

**Key accomplishments:**
- made managed skill assets the canonical surface for watch, install, scan, and
  serve behavior
- moved `sxmc watch` behind a dedicated runtime seam and bounded notification
  side effects
- staged local and git-backed skill installs atomically with explicit payload
  allowlisting
- pinned the migrated `skills` and `serve` paths with real-fixture contract
  tests
- documented the release-soak-based rollback decision instead of leaving seam
  retention implicit

**Stats:**
- 71 files modified
- 8,039 insertions and 1,306 deletions
- 6 phases, 12 plans, 21 tasks
- 4 days from milestone kickoff to archive

**Git range:** `f1f6878` → `482bf0f`

**What's next:** Define the next milestone around diagnostics, provenance,
trust policy, and the next orchestration seams.

---

## v1.0 Product-Preserving Greenfield Core Rewrite (Shipped: 2026-04-05)

**Delivered:** The maintained onboarding and reconciliation path moved onto a
cleaner internal core/app seam without changing the shipped product surface.

**Phases completed:** 1-6 (10 plans total)

**Key accomplishments:**
- published the golden-path contract and parity baseline
- introduced `src/app/` as the new internal seam for migrated command families
- migrated `status`, `sync`, `add`, and `setup` onto typed app services
- reduced golden-path orchestration to a thin dispatcher backed by shared
  onboarding logic

**What's next:** Expand the same rewrite pattern to `watch`, `skills`, serve
policy, and rollout governance in v1.1.

---
