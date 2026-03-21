# Architecture

`sxmc` is organized around one idea: take an existing surface that agents struggle to use directly, normalize it, and expose it as a smaller, safer, more reusable surface.

## Core Pipeline

```text
skills/        -> server/       -> client/        -> cli_surfaces/
Skills         -> MCP server    -> CLI workflows  -> AI startup artifacts
```

There is a parallel API path:

```text
OpenAPI / GraphQL -> client/ -> CLI workflows
```

## Module Map

- `src/skills/`
  Skill discovery, parsing, frontmatter handling, and API-to-skill generation.
- `src/server/`
  MCP server construction plus stdio and streamable HTTP transports.
- `src/client/`
  MCP stdio/HTTP clients, OpenAPI, GraphQL, and shared command definitions.
- `src/bake/`
  Saved connection definitions for MCP and API sources.
- `src/security/`
  Skill and MCP surface scanners.
- `src/output/`
  Human-readable, JSON, and TOON-format rendering.
- `src/cli_surfaces/`
  CLI inspection, host profile registry, startup-doc generation, config scaffolding, and artifact materialization.
- `src/cli_args.rs`
  Clap argument definitions for the binary.
- `src/command_handlers.rs`
  Shared command handlers extracted from the CLI entrypoint.
- `src/main.rs`
  CLI orchestration, MCP session flow, and top-level command dispatch.

## Design Boundaries

### Skills -> MCP

`sxmc serve` exposes:
- skill bodies as prompts
- `scripts/` as tools
- `references/` as resources
- generic retrieval tools for broader client compatibility

That hybrid model is what makes `skills -> MCP -> CLI` viable without forcing every client to understand every MCP surface equally well.

### MCP -> CLI

There are two user-facing layers:
- `sxmc stdio` / `sxmc http`
  Raw bridge and debugging layer
- `sxmc mcp`
  Saved, token-aware, day-to-day workflow

This split keeps transport debugging available without making the low-level path the default UX.

### API -> CLI

The API client layer normalizes both OpenAPI and GraphQL into a shared `CommandDef` shape. That gives the CLI one listing and invocation model even though the backing protocols differ.

### CLI -> AI

The `cli_surfaces` module uses a two-stage model:
1. inspect a CLI into a canonical JSON profile
2. generate host-aware startup artifacts from that profile

The important design choice is that JSON is the source of truth and host files are outputs. That keeps generation testable, versionable, and reviewable.

## Reliability Choices

- Bake writes are atomic.
- HTTP-facing commands support explicit timeouts.
- HTTP MCP serving applies request-body and concurrency guardrails.
- Watch mode prefers filesystem events and falls back to polling if needed.
- Markdown and TOML updates use managed blocks instead of full-file overwrites.
- JSON config scaffolds merge known host config shapes instead of replacing them wholesale.

## Why The Split Matters

Two files were carrying too much responsibility before this hardening pass:
- `src/main.rs`
- `src/cli_surfaces.rs`

They are now split by concern:
- clap surface vs runtime handling
- profile model vs inspection vs rendering vs materialization

That makes it much easier to:
- add a new host profile
- add a new CLI flag
- change artifact write policy
- reason about tests and regressions

## Where To Extend

If you want to add:

- a new MCP transport or client behavior:
  start in `src/client/` or `src/server/`
- a new AI host target:
  start in `src/cli_surfaces/model.rs` and `src/cli_surfaces/render.rs`
- a new API source type:
  start in `src/client/` and wire it through `src/client/api.rs`
- a new security pattern:
  start in `src/security/`

## Release Gate

Architecturally, `sxmc` is healthy when:
- the public command surface matches the docs
- generated artifacts remain review-first
- new hosts are added through the registry rather than ad hoc string matching
- tests cover both raw bridges and baked daily workflows
