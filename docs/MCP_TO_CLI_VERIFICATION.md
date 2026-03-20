# MCP → CLI verification notes

This document records **manual verification** (2026-03) that **sxmc** can act
as an MCP **client** and expose remote or stdio MCP servers through a
tool-focused CLI bridge: **`sxmc stdio`** and **`sxmc http`**.

## What “MCP → CLI” means in sxmc today

`sxmc stdio` and `sxmc http` currently provide:

- listing of **tools**, **prompts**, and **resources**
- invocation of **tools**
- pretty-printing and shell-friendly inspection of tool results

They do **not** yet provide first-class CLI commands for:

- `get_prompt`
- `read_resource`

So the precise contract is:

- **MCP discovery surface:** tools, prompts, resources
- **MCP invocation surface:** tools

## Summary

| Transport | Discovery | Invocation | Status | Evidence |
|-----------|-----------|------------|--------|----------|
| **stdio** | tools, prompts, resources | tools | **Working** | Live runs + `tests/cli_integration.rs` (`test_stdio_*`) |
| **HTTP** (streamable MCP) | tools, prompts, resources | tools | **Working** | Live run against local `sxmc serve --transport http` + `test_http_*` in same file |

## Best for

- shell automation against MCP tools
- CI checks and scripted workflows
- debugging MCP servers outside a full agent/IDE
- inspecting the available tool/prompt/resource surface quickly with `--list`

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

```bash
sxmc stdio "sxmc serve --paths /path/to/sxmc/tests/fixtures" skill_with_scripts__hello args=test
```

Expected: tool execution succeeds and returns script output.

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

```bash
sxmc http http://127.0.0.1:8765/mcp \
  --auth-header "Authorization: Bearer $SXMC_MCP_TOKEN" \
  --list
```

Expected: hosted/secured streamable HTTP MCP endpoints can be inspected the same
way once auth headers are supplied.

## Caveats

- The bridge only works as well as the **upstream MCP server** (auth, crashes, non-compliant responses).
- **Tool names and arguments** must match the server’s schema (same as any MCP client).
- `sxmc stdio` currently spawns from a **single command string** using simple
  whitespace splitting, so complex quoting is better handled by a wrapper
  script.
- Prompts/resources are visible in `--list`, but the bridge is primarily for
  **tool invocation** today.

## Common failure modes

- wrong or missing auth headers when using `sxmc http`
- upstream MCP server exits early or returns a non-compliant response
- tool name typo or argument mismatch
- over-complicated shell quoting inside the `sxmc stdio "<command>"` string

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
