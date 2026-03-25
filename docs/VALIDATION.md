# Validation Guide

This guide consolidates the release checklist, compatibility notes, smoke
tests, and benchmark summary.

The maintained validation record now lives in this guide plus
[`VALIDATION_HISTORY.md`](VALIDATION_HISTORY.md), which keeps the compact
release-by-release milestone table.

## What To Run Before A Release

From the repo root:

```bash
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
bash scripts/certify_release.sh target/debug/sxmc tests/fixtures
SXMC=target/debug/sxmc bash scripts/test-sxmc.sh --json /tmp/sxmc-test-results.json
```

Optional real-world MCP pass when Node and network are available:

```bash
bash scripts/smoke_real_world_mcps.sh target/debug/sxmc
```

For `1.x` readiness, also confirm that:

- [PRODUCT_CONTRACT.md](PRODUCT_CONTRACT.md) still matches the shipped support boundary
- [STABILITY.md](STABILITY.md) still matches the promised stable workflow and JSON rules
- the `setup -> add -> status -> sync` lifecycle still behaves as a
  first-class maintained path, not a best-effort side effect

## Coverage Summary

The maintained product coverage now centers on three layers:

- automated tests in `cargo test`
- release certification via `scripts/certify_release.sh`
- comprehensive CLI/user-path coverage via `scripts/test-sxmc.sh`
- portable cross-platform smoke via `scripts/smoke_portable_core.sh`
- portable fixture-based MCP smoke via `scripts/smoke_portable_fixtures.sh`
- optional real-world MCP smoke via `scripts/smoke_real_world_mcps.sh`
- generated fixture inventory via `scripts/fixture_inventory.py`

High-value scenarios covered in this stack include:

- `skills -> MCP`
- `MCP -> CLI` over stdio and HTTP
- baked `sxmc mcp` workflows
- auth-required hosted MCP
- `/healthz`
- `serve --watch`
- local OpenAPI and GraphQL flows
- `skills create`
- promptless or resource-less third-party MCP servers
- zero-argument tool interoperability
- CLI inspection, startup artifact preview, managed doc apply, and Cursor config merge coverage
- doctor JSON and human-mode coverage
- cache statistics, cache invalidation, and batch CLI inspection coverage
- discovery-tool-manifest serving over stdio and HTTP smoke coverage
- portable discovery-to-delivery smoke for codebase and traffic snapshots
- portable fixture MCP coverage for stdio, baked MCP, hosted HTTP, and
  bearer-protected HTTP flows
- an inspectable inventory of the shipped local fixtures and their roles

## CI Matrix

The repo now validates `sxmc` as a cross-platform product path instead of only a
Rust crate:

- Ubuntu: `cargo test`, startup smoke, `scripts/smoke_portable_core.sh`, and
  `scripts/smoke_portable_fixtures.sh`, and `scripts/test-sxmc.sh`
- macOS: `cargo test`, startup smoke, `scripts/smoke_portable_core.sh`, and
  `scripts/smoke_portable_fixtures.sh`, and `scripts/test-sxmc.sh`
- Windows: `cargo test`, `scripts/smoke_portable_core.sh`,
  `scripts/smoke_portable_fixtures.sh`, plus explicit smoke for `doctor`,
  compact inspection, and cache-stats JSON output

That keeps the larger Unix-oriented validation script in the loop while still
exercising Windows-specific command paths in CI, while also giving every OS the
same smaller discovery-delivery and local fixture MCP smoke paths.

## Compatibility Notes

`sxmc` has been exercised against:

- Codex-style local MCP configuration
- Cursor-style local and remote MCP configuration
- Gemini CLI-style local and remote MCP configuration
- Claude Code-style local and remote MCP configuration
- official external MCP servers such as:
  - `@modelcontextprotocol/server-everything`
  - `@modelcontextprotocol/server-memory`
  - `@modelcontextprotocol/server-filesystem`
  - `@modelcontextprotocol/server-sequential-thinking`
  - `@modelcontextprotocol/server-github`

The practical support boundary is defined in
[`PRODUCT_CONTRACT.md`](PRODUCT_CONTRACT.md).

The intended `1.x` stability rules are summarized in
[`STABILITY.md`](STABILITY.md).

For a compact cross-platform view of what is covered where, see
[`COMPATIBILITY_MATRIX.md`](COMPATIBILITY_MATRIX.md).

## Benchmarks

Local one-shot paths are consistently fast enough that they are not the main
product concern. The more important product value is:

- fewer agent turns
- smaller prompt payloads
- on-demand MCP schema inspection instead of eager schema loading

Benchmarks are useful for regression sanity, not as proof of broad client
compatibility.

The more reliable value signal is workflow compression:

| Task | Without `sxmc` | With `sxmc` | Practical result |
|---|---|---|---|
| List API endpoints | `curl` plus parsing glue | `sxmc api <url> --list` | Replace custom parsing with one command. |
| Call API endpoint | Manual URL, params, and header construction | `sxmc api <url> operation key=value` | Call by operation name instead of reconstructing the request shape. |
| Inspect MCP server | Custom JSON-RPC script or dedicated client | `sxmc stdio "<cmd>" --list` | Shell-level MCP inspection becomes repeatable. |
| Invoke MCP tool | Same plus extra call logic | `sxmc stdio "<cmd>" tool key=value` | One-shot MCP tool access from the terminal. |
| Scan skills | Grep and manual review | `sxmc scan` | Structured, severity-ranked findings with deeper checks. |
| `CLI -> AI` startup setup | Manual doc/config work per host | `sxmc inspect cli ...` + `sxmc init ai ...` | Host-aware startup artifacts generated instead of handwritten. |

Recent parser hardening also improved the real CLI inspection path that powers
`CLI -> AI` scaffolding:

- `gh` now preserves grouped subcommands and top-level flags together
- `rustup` now keeps its global top-level flags
- `python3` no longer turns environment variables into fake subcommands
- `node`, `npm`, and `python3` now produce cleaner summaries for downstream
  agent-doc and skill scaffolds

Validation note:

- `sxmc inspect cli` executes real binaries via subprocess spawn.
- shell aliases or shell functions that only exist in an interactive shell are
  not visible to that subprocess environment.
- treat “works in my shell, not in `sxmc inspect cli`” as an environment/path
  check first, not automatically as a parser regression.

## Real-World Side-by-Side

The current comparison set exercised:

- five real-world skills:
  - `git-commit-review`
  - `docker-debug`
  - `code-review`
  - `pr-summary`
  - `dependency-audit`
- five MCP servers:
  - `server-everything`
  - `server-filesystem`
  - `server-memory`
  - `server-sequential-thinking`
  - `sxmc serve`

### Skills

| Test | Without `sxmc` | With `sxmc` | Practical result |
|---|---|---|---|
| Discover 5 skills | Custom shell/frontmatter parsing | `sxmc skills list` | Cleaner discovery with fewer edge cases and one built-in command. |
| Serve skills as MCP | Handwritten JSON-RPC server code | `sxmc serve --paths <dir>` | Large implementation savings: serving skills over MCP becomes a built-in path. |
| Inspect served MCP surfaces | Custom client script | `sxmc stdio "sxmc serve ..." --list` | Immediate inspection of prompts, tools, and resources. |
| Fetch prompt with args | JSON-RPC client plus prompt payload formatting | `sxmc stdio ... --prompt code-review arguments="src/main.rs"` | Prompt retrieval becomes direct, and argument substitution stays consistent with skill metadata. |
| Scan all 5 skills | Grep/manual review | `sxmc scan` | Broader checks across Unicode, permissions, injection patterns, and rule-coded severity output. |

### MCP Servers

| Test | Without `sxmc` | With `sxmc` | Practical result |
|---|---|---|---|
| List tools from `server-everything` | Custom JSON-RPC script | `sxmc stdio "mcp-server-everything" --list` | Shell-level server inspection with no throwaway client code. |
| Invoke `echo` tool | Same plus call framing | `sxmc stdio ... echo message="Hello"` | One-shot tool invocation from the terminal. |
| List filesystem tools | Custom discovery logic | `sxmc stdio "mcp-server-filesystem /tmp" --list` | Direct tool listing for third-party MCP servers. |
| Read a file via MCP | Custom JSON-RPC client | `sxmc stdio ... read_file path=/path/to/file` | MCP resource/tool access becomes a CLI command instead of bespoke code. |
| Discover memory server | Read source or write a probe client | `sxmc stdio "mcp-server-memory" --list-tools` | Faster schema discovery for servers you did not author. |
| Create entity and search | Custom call framing with nested params | `sxmc stdio ... create_entities ...` then `search_nodes query=sxmc` | Complex tool calls become scriptable shell commands. |
| Inspect tool schema | Read source code or build a schema probe | `sxmc stdio ... --describe-tool sequentialthinking` | Full parameter and type inspection on demand. |
| Cross-server grep | Custom aggregation tooling | `sxmc mcp grep "file"` | Unique cross-server search across baked MCP inventories. |

### Current Read

Across this comparison set, the gains fall into three practical buckets:

- gap-filling:
  - `skills -> MCP`
  - ad hoc `MCP -> CLI`
  - cross-server baked `mcp grep`
  - on-demand `--describe-tool` inspection
- major time savings:
  - serving skills over MCP
  - invoking MCP tools without custom JSON-RPC clients
  - structured security scanning instead of ad hoc grep
- convenience:
  - API listing/calling
  - skill discovery
  - startup scaffolding for AI hosts

## Token Utilization Summary

The stable lesson is narrower and more useful than a single blended headline:
`sxmc` saves the most tokens when it replaces protocol glue, spec-reading, or
multi-step discovery, and it saves much less when the dominant cost is a large
payload that still has to be shown to the model.

Historical and recent measurements both support that:

- older 10-scenario validation runs showed large blended savings when the
  manual path involved repeated JSON-RPC framing, spec reading, and startup
  glue
- a later 8-task Linux comparison found only modest blended savings once a
  single huge direct API payload dominated the total, but still found much
  stronger savings when that outlier was excluded

The practical read:

- strongest token wins:
  - discovery-heavy flows
  - MCP inspection and protocol glue replacement
  - CLI and codebase summarization
- weakest token wins:
  - large direct API responses where the payload itself dominates
- biggest non-token win:
  - fewer turns, fewer retries, and less shell/protocol scaffolding

For token-constrained agent use, prefer these selectors first:

- `sxmc api|spec|graphql --list --compact`
- `sxmc api|spec|graphql --list --names-only --limit N`
- `sxmc api|spec|graphql --list --required-only --fields ...`
- `sxmc api|spec|graphql --list --counts-only --offset N --limit N`
- `sxmc skills list --names-only --limit N`
- `sxmc skills list --counts-only`
- `sxmc skills info --summary-only`
- `sxmc inspect cli --compact`
- `sxmc discover db|codebase|traffic --counts-only`
- `sxmc discover db|codebase|traffic --fields ... --offset N --limit N --format json`

These numbers are best read as workflow-efficiency estimates, not billing-grade
measurements. The stable product lesson is the same: `sxmc` helps most when it
compresses workflows and lets the model ask for smaller views of a surface
instead of dumping the entire surface into context.

## Why Manual JSON-RPC Retries Happen

The retry multiplier in manual MCP flows usually comes from a small set of
failure classes:

- capability assumptions:
  - prompt-less or resource-less servers return `-32601` / "method not found" when a hand-built client assumes every surface exists
- argument-shape mismatches:
  - some servers require `arguments: {}` even for zero-arg tools, or reject calls until the exact schema is followed
- stateful workflow assumptions:
  - repeated fresh stdio invocations do not share MCP session memory
- stdout/stderr mixing:
  - machine parsing fails when informational stderr lines are concatenated with structured stdout
- quoting and command spawning:
  - ad hoc shell-wrapped JSON-RPC scripts are brittle around nested JSON and platform-specific command parsing

In the current comparison model, the most common hidden-cost failures are:

| # | Failure mode | What happens | Typical retry impact |
|---|---|---|---|
| 1 | Skipped or malformed initialization | Some servers reject later calls entirely | `+1-2` turns |
| 2 | Wrong argument shape | `-32602`, `-32603`, or validation failures | `+2-3` turns |
| 3 | Wrong protocol/version assumptions | Server rejects or behaves unexpectedly | `+1-2` turns |
| 4 | Response framing/buffering mistakes | Partial or mixed JSON breaks parsing | `+2-3` turns |
| 5 | `stderr` mixed into `stdout` | JSON parsing fails even when the call succeeded | `+1-2` turns |
| 6 | Missing server startup args | Server crashes, hangs, or starts in the wrong mode | `+1-2` turns |
| 7 | Calling unsupported capabilities | Prompt/resource calls fail on tool-only servers | `+1-2` turns |
| 8 | No cleanup/process hygiene | Zombie processes, port conflicts, flaky follow-up runs | cumulative |
| 9 | Interactive package-manager prompts | `npx`/installer flows hang waiting for confirmation | `+2-3` turns |
| 10 | Wrong API URL/request construction | `404`, `400`, or content-type failures | `+2-4` turns |

The intended `sxmc` recovery path is:

1. `sxmc mcp grep <pattern>` or `sxmc mcp tools <server> --limit 10`
2. `sxmc mcp info <server/tool> --format toon`
3. `sxmc mcp call <server/tool> '<json-object>'`
4. `sxmc mcp session <server>` when continuity matters

Recent CLI behavior also points failed tool calls back toward schema inspection
and session mode so agents can recover in one turn instead of rediscovering the
failure mode from raw JSON-RPC errors.

In practice, `sxmc` reduces or absorbs most of these failure classes because it
handles:

- MCP session initialization and capability discovery
- zero-arg and object-shaped tool arguments
- `stdout`/`stderr` separation in the intended machine-readable path
- process spawning and baked connection reuse
- schema inspection and stateful fallback (`mcp session`) in the recovery path
- API request construction from operation metadata instead of ad hoc URL assembly

That does **not** mean every call succeeds unconditionally on the first try:
broken upstream servers, auth mistakes, bad inputs, and network failures can
still fail. The more accurate product claim is that `sxmc` removes a large
fraction of the *self-inflicted* retry loops that come from hand-built protocol
glue.

## Startup Sanity

Quick startup checks:

```bash
bash scripts/startup_smoke.sh target/debug/sxmc
python3 scripts/benchmark_startup.py /tmp/sxmc-startup-benchmark.md
```

## Current Read

The current validation posture is:

- release certification is scripted
- real-world MCP smoke is scripted
- broad end-to-end paths are covered in tests
- remaining work should come from real user findings, not speculative expansion

## Latest maintainer snapshot

**Stable-major validation snapshot (`1.0.0`)** — **296** tests, **0** failed,
**0** skipped, with a 10x10x10 matrix (10 CLIs, 10 skills, 10 MCPs),
benchmarks, GraphQL/traffic/codebase/db discovery lifecycle coverage, bundle
signing, registry, trust, wrap, onboarding/status contract coverage, local
sync reconciliation, the stability/support sweep, publish/pull, and
side-by-side with/without comparisons.

Repeated standalone **`sxmc stdio …`** invocations do **not** share MCP session memory. For continuity, use **`sxmc mcp session <server>`** (see validation run §9).

Current CLI-to-AI coverage is automated rather than client-UI-driven:

- `inspect cli` self-guard and self-inspection with `--allow-self`
- `init ai` preview mode for Claude-style startup docs
- managed `AGENTS.md` apply without overwriting existing content
- Cursor MCP config merge behavior
- Gemini-native `GEMINI.md` apply behavior
- GitHub Copilot native instructions generation
- Continue / Junie / Windsurf native doc generation
- OpenCode config generation
- JetBrains AI Assistant native rules generation
- optional `llms.txt` export generation

See [COMPATIBILITY_MATRIX.md](COMPATIBILITY_MATRIX.md) for the maintained
cross-platform and host coverage matrices.
