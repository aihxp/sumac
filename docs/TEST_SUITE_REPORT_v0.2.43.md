# Sumac (`sxmc`) v0.2.43 Test Suite Report

**Version:** 0.2.43  
**Platform:** macOS Darwin arm64  
**Date:** 2026-03-25  
**Test script:** `scripts/test-sxmc.sh`

---

## Results

| Metric | Value |
|---|---|
| Total tests | 293 |
| Passed | 293 |
| Failed | 0 |
| Skipped | 0 |
| CLI tools parsed | 94 |
| CLI tools failed to parse | 0 |
| Bad summaries | 0 |
| Benchmark iterations | 5 per measurement |

**ALL 293 TESTS PASSED — ZERO FAILURES, ZERO SKIPS**

---

## Scope

This pass covers the full shipped surface through `v0.2.43`, including:

- CLI inspection, compact mode, caching, diffing, drift, and watch
- scaffold generation and AI host initialization across 10 clients
- skill discovery, info, execution, and MCP serving
- MCP bake flows, stdio/http bridges, wrap, wrapped execution telemetry, and interactive/TUI filtering
- OpenAPI API mode, GraphQL discovery, GraphQL schema snapshots, and GraphQL diffing
- codebase discovery, snapshotting, and diffing
- database discovery for SQLite/PostgreSQL, including snapshot output
- traffic discovery from HAR and saved `curl` history, plus traffic snapshots and diffing
- publish/pull, bundle export/import/verify/signing
- corpus export/query/stats, registry flows, trust policy, and known-good selection
- doctor, status, health gates, host comparison, onboarding recovery, and one-step onboarding flows
- side-by-side workflow comparisons and benchmark runs

---

## Highlights

- `94` installed CLI tools parsed successfully
- `0` parse failures
- `0` bad summaries
- `293` total tests passed
- `sxmc add` and `sxmc setup` now expose explicit structured-output contracts via `--pretty` / `--format ...`
- `sxmc add --client ...`, `sxmc setup --client ...`, `sxmc doctor --host ...`, and `sxmc status --host ...` now validate the tightened host-selection UX
- onboarding and health/status contract coverage now sits in the same release certification run as the legacy CLI/MCP/API surfaces

---

## Benchmark Snapshot

Median snapshots from the run:

- warm CLI inspection: `6–8ms`
- `wrap git -> stdio --list`: `27ms`
- bundle export (5 profiles): `15ms`
- bundle export + HMAC sign: `20ms`
- full `inspect -> scaffold -> init-ai` pipeline for 5 CLIs: `101ms`

---

## Notes

- This report supersedes the previous `v0.2.41` “latest validation” references.
- The remaining `1.0.0` work is now mostly release-contract and platform validation polish rather than missing workflow coverage.
