# MCP → CLI verification notes

This document records **manual verification** (2026-03) that **sxmc** can act as an MCP **client** and expose remote or stdio MCP servers as **CLI** commands: **`sxmc stdio`** and **`sxmc http`**.

## Summary

| Transport | CLI | Status | Evidence |
|-----------|-----|--------|----------|
| **stdio** | `sxmc stdio "<spawn cmd>" …` | **Working** | Live runs + `tests/cli_integration.rs` (`test_stdio_*`) |
| **HTTP** (streamable MCP) | `sxmc http <url>/mcp …` | **Working** | Live run against local `sxmc serve --transport http` + `test_http_*` in same file |

## Implementation (source)

- **stdio client:** `src/client/mcp_stdio.rs` — `StdioClient`, rmcp child-process transport.
- **HTTP client:** `src/client/mcp_http.rs` — streamable HTTP MCP.
- **CLI wiring:** `src/main.rs` — subcommands `stdio` and `http`.

## Manual checks performed

### 1. `sxmc stdio` (nested skills server)

```bash
sxmc stdio "sxmc serve" --list
```

Expected: hybrid tools (`get_available_skills`, `get_skill_details`, `get_skill_related_file`) plus per-skill script tools when skills are discovered.

```bash
sxmc stdio "sxmc serve --paths /path/to/sxmc/tests/fixtures" get_available_skills --pretty
```

Expected: JSON listing fixture skills.

### 2. `sxmc http` (local HTTP MCP server)

In one shell:

```bash
sxmc serve --transport http --host 127.0.0.1 --port 8765 --paths /path/to/sxmc/tests/fixtures
```

In another:

```bash
sxmc http http://127.0.0.1:8765/mcp --list
```

Expected: same tool list as stdio serve for the same `--paths` (prompts/resources included).

## Caveats

- The bridge only works as well as the **upstream MCP server** (auth, crashes, non-compliant responses).
- **Tool names and arguments** must match the server’s schema (same as any MCP client).

## Automated regression tests

From the repo root:

```bash
cargo test --test cli_integration
```

Covers stdio bridging, HTTP MCP with auth headers/bearer tokens, hybrid skill tools, and related flows.

## Related docs

- [SMOKE_TESTS.md](SMOKE_TESTS.md) — scripted smoke including stdio ↔ serve loop
- [CLIENTS.md](CLIENTS.md) — example `sxmc http` with `--auth-header`
- [VALUE_AND_BENCHMARK_FINDINGS.md](VALUE_AND_BENCHMARK_FINDINGS.md) — why MCP → CLI is useful
