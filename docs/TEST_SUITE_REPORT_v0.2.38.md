# Sumac (`sxmc`) v0.2.38 Test Suite Report

**Version:** 0.2.38  
**Platform:** macOS Darwin arm64  
**Date:** 2026-03-24  
**Test script:** `scripts/test-sxmc.sh`

---

## Results

| Metric | Value |
|---|---|
| Total tests | 250 |
| Passed | 250 |
| Failed | 0 |
| Skipped | 0 |
| CLI tools parsed | 94 |
| CLI tools failed to parse | 0 |
| Bad summaries | 0 |
| Benchmark iterations | 5 per measurement |

**ALL 250 TESTS PASSED — ZERO FAILURES, ZERO SKIPS**

---

## Scope

This pass covers the complete shipped surface through `v0.2.38`, including:

- CLI inspection, compact mode, caching, diffing, drift, and watch
- scaffold generation and AI host initialization across 10 clients
- skill discovery, info, execution, and MCP serving
- MCP bake flows, stdio/http bridges, wrap, and wrapped execution telemetry
- OpenAPI API mode, publish/pull, bundle export/import/verify/signing
- corpus export/query/stats, registry flows, trust policy, and known-good selection
- doctor, status, health gates, and host comparison
- side-by-side workflow comparisons and benchmark runs

---

## Coverage Highlights

### Part A — Existing Product Surface

- `94` installed CLI tools parsed successfully
- `0` parse failures
- `0` bad summaries
- previously broken tools remain fixed
- compact mode, caching, scaffold, init-ai, scan, MCP, bake, API, doctor, serve, and wrap all passed

### Part B — Expansion Features

- bundle export/import/verify/signing passed
- publish/pull round-trips passed
- corpus stats/query passed with metadata assertions
- registry init/add/list/pull/sync/serve/push passed, including remote end-to-end workflow
- trust-report and trust-policy passed with enforcement assertions
- known-good selection passed with candidate ranking checks

### Part C — 10x10x10 Matrix

- 10 CLIs: passed
- 10 skills: passed
- 10 MCPs: passed

### Part D — Benchmarks

Median benchmark snapshots from the run:

- CLI inspection warm cache: `4ms`
- average CLI inspection speedup: about `37x`
- wrap `git -> stdio --list`: `7ms`
- bundle export (5 profiles): `5ms`
- full `inspect -> scaffold -> init-ai` pipeline for 5 CLIs: `67ms`

---

## Notes

- The committed test script now prefers the repo-local build (`target/release` or `target/debug`) before falling back to an installed `sxmc` on `PATH`, preventing stale-binary false failures.
- This report supersedes earlier “latest validation” references that pointed at the older v0.2.37 snapshot.
