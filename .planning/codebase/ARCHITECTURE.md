# Architecture

**Analysis Date:** 2026-04-04

## Pattern Overview

**Overall:** Modular Rust library behind a single CLI binary

**Key Characteristics:**
- `src/main.rs` is the operational entrypoint and contains the top-level `clap` dispatch for the full `sxmc` command surface.
- Reusable behavior is pushed into library modules under `src/`, so the binary mostly orchestrates parsing, path resolution, output selection, and module calls.
- The product is organized around surface translation: skills to MCP in `src/skills/` + `src/server/`, MCP or APIs to CLI in `src/client/`, and CLI to AI host artifacts in `src/cli_surfaces/`.

## Layers

**CLI Surface And Dispatch:**
- Purpose: Parse arguments, choose the subcommand path, and glue together modules for runtime execution.
- Location: `src/main.rs`, `src/cli_args.rs`, `src/command_handlers.rs`
- Contains: `clap` enum definitions, shared option structs, top-level `match cli.command`, and reusable handlers for skills and API commands.
- Depends on: Nearly every exported module from `src/lib.rs`, especially `src/client/`, `src/server/`, `src/skills/`, `src/cli_surfaces/`, `src/paths.rs`, and `src/output/mod.rs`.
- Used by: The `sxmc` binary declared in `Cargo.toml`.

**Skill Discovery And Modeling:**
- Purpose: Discover skill folders, parse `SKILL.md`, enumerate `scripts/` and `references/`, and build in-memory skill models.
- Location: `src/skills/discovery.rs`, `src/skills/parser.rs`, `src/skills/models.rs`, `src/skills/install.rs`, `src/skills/generator.rs`
- Contains: Filesystem scans, YAML frontmatter parsing, argument-hint parsing, install metadata, and OpenAPI-to-skill generation.
- Depends on: `src/error.rs`, `src/paths.rs`, and the local filesystem.
- Used by: `src/server/mod.rs`, `src/server/handler.rs`, `src/command_handlers.rs`, and subcommands in `src/main.rs`.

**MCP Server Construction:**
- Purpose: Turn parsed skills or wrapped CLI profiles into MCP servers over stdio or streamable HTTP.
- Location: `src/server/mod.rs`, `src/server/handler.rs`, `src/server/wrap.rs`
- Contains: `SkillsServer`, HTTP auth and limit handling, watch/reload support, discovery snapshot mounting, and wrapped-CLI tool derivation.
- Depends on: `src/skills/`, `src/discovery_snapshots.rs`, `src/executor.rs`, `src/cli_surfaces/`, and `rmcp`/`axum`.
- Used by: `Commands::Serve` and `Commands::Wrap` in `src/main.rs`.

**Client Adapters:**
- Purpose: Normalize external sources into a common CLI interaction model.
- Location: `src/client/api.rs`, `src/client/openapi.rs`, `src/client/graphql.rs`, `src/client/mcp_stdio.rs`, `src/client/mcp_http.rs`, `src/client/database.rs`, `src/client/codebase.rs`, `src/client/traffic.rs`, `src/client/commands.rs`
- Contains: API autodetection, MCP transport clients, schema discovery, codebase inspection, HAR inspection, and shared command definitions.
- Depends on: `src/error.rs`, `src/output/mod.rs`, `src/projection.rs`, HTTP/MCP crates, and local files for schema/codebase discovery.
- Used by: `src/main.rs` and `src/command_handlers.rs`.

**CLI Inspection And Artifact Generation:**
- Purpose: Inspect a real CLI into a normalized profile, then generate AI-facing docs, configs, wrappers, and supporting artifacts.
- Location: `src/cli_surfaces/inspect.rs`, `src/cli_surfaces/model.rs`, `src/cli_surfaces/render.rs`, `src/cli_surfaces/materialize.rs`
- Contains: command-spec parsing, profile caching, profile models, host registry, artifact rendering, and file materialization policies.
- Depends on: `src/cache.rs`, `src/paths.rs`, `src/error.rs`, and the filesystem.
- Used by: `inspect`, `init`, `scaffold`, `doctor`, `setup`, `add`, `status`, `sync`, and `wrap` flows in `src/main.rs`.

**State, Paths, And Shared Infrastructure:**
- Purpose: Provide durable local/global state, formatting, execution, and lightweight utilities.
- Location: `src/paths.rs`, `src/cache.rs`, `src/bake/config.rs`, `src/auth/secrets.rs`, `src/output/mod.rs`, `src/executor.rs`, `src/projection.rs`, `src/error.rs`
- Contains: local vs global install paths, cache TTL storage, baked connection persistence, secret/header resolution, output formatting, subprocess execution, JSON field projection, and the crate-wide error type.
- Depends on: filesystem, environment variables, and serde-based serialization.
- Used by: most modules in the crate.

**Security Scanning:**
- Purpose: Scan skills and MCP surfaces for risky content and report findings with severity.
- Location: `src/security/mod.rs`, `src/security/skill_scanner.rs`, `src/security/mcp_scanner.rs`, `src/security/patterns.rs`
- Contains: finding models, pattern heuristics, and report formatting.
- Depends on: `src/error.rs` and parsed skill or MCP metadata.
- Used by: `scan`-related flows in `src/main.rs`.

## Data Flow

**Skills To MCP:**

1. `src/main.rs` resolves skill paths through `resolve_paths()` and `src/skills/discovery::default_paths()`.
2. `src/server/mod.rs` calls `build_server()` to discover and parse skills from `src/skills/discovery.rs` and `src/skills/parser.rs`.
3. `src/server/handler.rs` converts parsed `Skill` values into MCP prompts, tools, and resources inside `SkillsServer`.
4. `src/server/mod.rs` serves that handler over stdio or HTTP, with optional auth, request limits, and watch-based reload behavior.

**Wrapped CLI To MCP:**

1. `Commands::Wrap` in `src/main.rs` parses runtime options and inspects the target CLI profile.
2. `src/server/wrap.rs` converts `CliSurfaceProfile` subcommands and parameters into narrowed MCP tools.
3. Wrapped tool calls execute subprocesses through `tokio::process::Command`, while execution history is retained as in-memory MCP-readable resources.

**API Or MCP To CLI:**

1. `src/main.rs` or `src/command_handlers.rs` parses user intent from `stdio`, `http`, `mcp`, `api`, `spec`, `graphql`, `discover`, or related commands.
2. `src/client/api.rs` auto-detects OpenAPI vs GraphQL and delegates to `src/client/openapi.rs` or `src/client/graphql.rs`.
3. Raw MCP access goes through `src/client/mcp_stdio.rs` or `src/client/mcp_http.rs`.
4. Results are shaped and rendered through `src/output/mod.rs` and `src/projection.rs`.

**CLI To AI Artifacts:**

1. `src/cli_surfaces/inspect.rs` parses a command spec, executes the real CLI help surface, and builds a `CliSurfaceProfile`.
2. Profiles are cached via `src/cache.rs` and represented by models in `src/cli_surfaces/model.rs`.
3. `src/cli_surfaces/render.rs` produces host-specific docs and config content.
4. `src/cli_surfaces/materialize.rs` chooses target paths and write strategies using `src/paths.rs` and `InstallPaths`.

**State Management:**
- User-global state lives under `src/paths.rs` paths such as `~/.config/sxmc` and `~/.cache/sxmc`.
- Repo-local generated AI state lives under `.sxmc/` when `InstallScope::Local` is used via `src/paths.rs`.
- Baked connection definitions are stored through `BakeStore` in `src/bake/config.rs`.
- CLI inspection cache entries are file-based and TTL-driven in `src/cache.rs`.

## Key Abstractions

**`Skill`:**
- Purpose: In-memory representation of one skill directory.
- Examples: `src/skills/models.rs`, constructed in `src/skills/parser.rs`
- Pattern: Parse once from filesystem, then reuse for listing, serving, running, and security scanning.

**`SkillsServer`:**
- Purpose: MCP handler for skills, discovery resources, and hybrid helper tools.
- Examples: `src/server/handler.rs`
- Pattern: Precompute indexes for skills, tools, and resources, then answer MCP list/read/call requests from those indexes.

**`CliSurfaceProfile`:**
- Purpose: Canonical JSON-friendly description of a CLI command surface.
- Examples: `src/cli_surfaces/model.rs`, produced by `src/cli_surfaces/inspect.rs`
- Pattern: Treat the normalized profile as source of truth, then derive docs, configs, wrappers, and drift workflows from it.

**`BakeConfig` / `BakeStore`:**
- Purpose: Persist reconnectable definitions for MCP and API sources.
- Examples: `src/bake/config.rs`
- Pattern: Load once from config storage, mutate in memory, and persist atomically through a temp file.

**`CommandDef`:**
- Purpose: Shared operation model for API-backed CLI execution.
- Examples: `src/client/commands.rs`, consumed by `src/client/api.rs`
- Pattern: Different backends expose one list-and-execute shape to the CLI surface.

## Entry Points

**Binary CLI:**
- Location: `src/main.rs`
- Triggers: Running `sxmc ...`
- Responsibilities: Parse CLI arguments, branch on subcommands, resolve state roots, call library modules, and emit output.

**Library API:**
- Location: `src/lib.rs`
- Triggers: Embedding `sxmc` as a Rust crate
- Responsibilities: Re-export stable internal modules such as `server`, `skills`, `client`, `cli_surfaces`, `security`, and `bake`.

**Integration Test Harness:**
- Location: `tests/cli_integration.rs`
- Triggers: `cargo test`
- Responsibilities: Exercise the compiled binary, spawn temporary HTTP/MCP fixtures, and validate end-to-end command flows.

## Error Handling

**Strategy:** One crate-level `Result<T>` and `SxmcError` type for application logic, with protocol-specific errors translated at boundaries.

**Patterns:**
- Use `crate::error::Result<T>` and `SxmcError` from `src/error.rs` across non-protocol modules.
- Convert HTTP/MCP boundary failures into protocol-native errors inside `src/server/handler.rs` and `src/server/mod.rs`.
- Emit non-fatal warnings to stderr for partial failures during listing or optional behavior, especially in `src/command_handlers.rs` and `src/main.rs`.

## Cross-Cutting Concerns

**Logging:** Minimal explicit logging; user-facing status and warnings are printed directly from `src/main.rs`, `src/command_handlers.rs`, and selected modules.

**Validation:** Input validation is largely local to each module: `clap` in `src/cli_args.rs`, command-spec parsing in `src/cli_surfaces/inspect.rs`, JSON/schema checks in `src/discovery_snapshots.rs`, and path/secret validation in `src/auth/secrets.rs`.

**Authentication:** Remote serving auth is implemented in `src/server/mod.rs` through required headers or bearer tokens; secret indirection is resolved in `src/auth/secrets.rs`.

---

*Architecture analysis: 2026-04-04*
