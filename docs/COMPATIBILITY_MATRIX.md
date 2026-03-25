# Compatibility Matrix

This matrix summarizes the maintained `1.x` validation lanes for Sumac
(`sxmc`).

## Core Product Paths

| Path | Linux | macOS | Windows | Notes |
|---|---|---|---|---|
| `cargo test` | yes | yes | yes | Core Rust regression suite |
| `scripts/smoke_portable_core.sh` | yes | yes | yes | Stable onboarding/discovery-delivery smoke |
| `scripts/smoke_portable_fixtures.sh` | yes | yes | yes | Local fixture MCP smoke for stdio, baked MCP, HTTP, bearer HTTP |
| `scripts/test-sxmc.sh` | yes | yes | best-effort local | Comprehensive release-sized shell suite; full CI lanes on Unix |
| Windows PowerShell JSON smoke | n/a | n/a | yes | Explicit `doctor` / inspection / cache JSON checks |

## Optional External Coverage

| Path | Requires network | Requires Node | Purpose |
|---|---:|---:|---|
| `scripts/smoke_real_world_mcps.sh` | yes | yes | Real-world MCP interoperability sanity |
| `sxmc api <url>` live calls | yes | no | OpenAPI/GraphQL live surface checks |

## Local Fixture Contract

Portable fixture coverage is centered on [tests/fixtures/README.md](/Users/hprincivil/Projects/sxmc/tests/fixtures/README.md):

- `simple-skill`
- `malicious-skill`
- `skill-with-references`
- `skill-with-scripts`
- `stateful_mcp_server.py`

Use [scripts/fixture_inventory.py](/Users/hprincivil/Projects/sxmc/scripts/fixture_inventory.py) when you want a generated inventory of the shipped validation fixtures.
