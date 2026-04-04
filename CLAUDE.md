<!-- sxmc:begin cli-ai:claude-code -->
## Sumac CLI Surface: `sxmc`

Use `sxmc` (Sumac) as a first-class terminal workflow in this repo for Claude Code.

Summary: Sumac — bring out what your tools can do (Skills × MCP × CLI)

Preferred flow:
1. `sxmc --help`
2. `sxmc serve` --help

High-confidence subcommands:
- `serve`: Start the MCP server (serves skills over MCP)
- `skills`: Manage skills
- `stdio`: Connect to an MCP server via stdio (MCP Server → CLI)
- `http`: Connect to an MCP server via HTTP (MCP Server → CLI)
- `mcp`: Use baked MCP connections in a token-efficient, mcp-cli-style workflow

Guidance:
- When the exact CLI surface is unclear, start with `sxmc inspect cli <tool> --depth 1 --format json-pretty` instead of pasting raw help output into chat.
- When the MCP surface is unknown, start with `sxmc stdio "<cmd>" --list` or `sxmc mcp grep <pattern>` before guessing tool calls.
- When the API surface is unknown, start with `sxmc api <url-or-spec> --list` before constructing requests by hand.
- Keep bulky output in files or pipes when possible.
- Prefer machine-friendly flags like `--json` when the CLI supports them.
- Re-check `--help` before using low-confidence flows.
- Startup file convention last verified against official docs on 2026-03-21.
- Reference: https://docs.anthropic.com/en/docs/claude-code/memory
<!-- sxmc:end cli-ai:claude-code -->

<!-- GSD:project-start source:PROJECT.md -->
## Project

**Sumac**

Sumac (`sxmc`) is a Rust CLI that makes AI assistants stop guessing how local
tools, APIs, MCP servers, skills, and project surfaces work. It inspects real
interfaces, turns them into structured profiles and artifacts, and uses those
to generate host-facing docs, client config, MCP wrappers, and discovery
outputs from one installable binary.

This planning cycle is for a brownfield product-preserving rewrite. The product
surface stays broad and stable, but the internals are being rebuilt so Sumac
feels like a cleaner native system rather than an accumulated set of layers and
special cases.

**Core Value:** Sumac must let AI systems understand and use real existing tools and interfaces
without bespoke glue, while staying fast, local-first, and reliable.

### Constraints

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
<!-- GSD:project-end -->

<!-- GSD:stack-start source:codebase/STACK.md -->
## Technology Stack

## Languages
- Rust 2021 edition - application code and CLI entrypoints in `Cargo.toml`, `src/main.rs`, and `src/lib.rs`
- JavaScript (Node.js ESM) - npm install wrapper and launcher metadata in `packaging/npm/package.json` and `packaging/npm/scripts/install.mjs`
- Shell + PowerShell - release, smoke, and CI automation in `.github/workflows/ci.yml`, `.github/workflows/release.yml`, and `scripts/`
- Python - helper scripts such as `scripts/fixture_inventory.py` and `scripts/benchmark_startup.py`
## Runtime
- Native Rust CLI binary `sxmc` built from `src/main.rs`
- Async runtime: Tokio 1 with `full` features in `Cargo.toml`
- HTTP serving/client runtime: Axum 0.8 + Reqwest 0.13 in `src/server/mod.rs`, `src/server/wrap.rs`, `src/client/api.rs`, `src/client/openapi.rs`, and `src/client/graphql.rs`
- Cargo via the stable Rust toolchain used in `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- Lockfile: present in `Cargo.lock`
- Secondary package manager: npm for distribution wrapper only in `packaging/npm/package.json`
## Frameworks
- `rmcp` 1.2 - MCP client/server transport layer for stdio and streamable HTTP in `src/server/mod.rs`, `src/server/wrap.rs`, `src/client/mcp_stdio.rs`, and `src/client/mcp_http.rs`
- `clap` 4 - CLI argument parsing in `src/cli_args.rs`
- `serde` / `serde_json` / `serde_yaml` / `toml` - structured config, spec, and artifact parsing in `src/client/`, `src/bake/config.rs`, and `src/cli_surfaces/`
- Rust built-in test harness via `cargo test` with integration coverage in `tests/cli_integration.rs`
- `assert_cmd` 2 and `predicates` 3 - CLI assertions in `Cargo.toml` and `tests/cli_integration.rs`
- Shell smoke suites in `scripts/test-sxmc.sh`, `scripts/smoke_portable_core.sh`, and `scripts/smoke_test_clients.sh`
- GitHub Actions - CI and release packaging in `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- `clap_complete` 4 - shell completion generation wired through `src/cli_args.rs`
- `notify` 8 - file watch/reload support in `src/server/mod.rs`
## Key Dependencies
- `rmcp` 1.2 - the MCP protocol backbone for Sumac’s core product surface in `Cargo.toml`
- `reqwest` 0.13 with `rustls` - all outbound HTTP for MCP, OpenAPI, GraphQL, bundle publish/pull, registry push/pull, npm install downloads, and webhooks in `src/client/`, `src/main.rs`, and `packaging/npm/scripts/install.mjs`
- `axum` 0.8 - inbound HTTP endpoints for hosted MCP and bundle registry servers in `src/server/mod.rs`, `src/server/wrap.rs`, and `src/main.rs`
- `clap` 4 - the large multi-command CLI surface in `src/cli_args.rs`
- `tokio` 1 - async execution, child processes, and HTTP serving in `src/main.rs`, `src/server/`, and `src/client/`
- `rusqlite` 0.32 with `bundled` - SQLite schema inspection for discovery in `src/client/database.rs`
- `postgres` 0.19 - PostgreSQL schema inspection for discovery in `src/client/database.rs`
- `hmac` 0.12 + `sha2` 0.10 + `ed25519-dalek` 2 - bundle signing and verification in `src/main.rs`
- `dirs` 6 + `tempfile` 3 - local config/cache placement and atomic writes in `src/paths.rs`, `src/bake/config.rs`, and `src/cache.rs`
## Configuration
- Local/global state roots are derived from `SXMC_CONFIG_HOME`, `SXMC_CACHE_HOME`, `XDG_CONFIG_HOME`, and `XDG_CACHE_HOME` in `src/paths.rs`
- HTTP auth and signing secrets are supplied at runtime, typically via `env:` or `file:` indirection handled in `src/auth/secrets.rs`
- NPM wrapper install behavior is controlled by `SXMC_NPM_SKIP_DOWNLOAD` and `SXMC_NPM_DOWNLOAD_BASE` in `packaging/npm/scripts/install.mjs`
- Rust manifest and dependency graph: `Cargo.toml` and `Cargo.lock`
- NPM wrapper metadata: `packaging/npm/package.json`
- Homebrew packaging: `packaging/homebrew/sxmc.rb`
- CI/release automation: `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- No pinned `rust-toolchain` file, `.nvmrc`, Dockerfile, or compose file detected at repo root
## Platform Requirements
- Rust stable toolchain with Cargo is required for the canonical install/build path in `README.md` and `.github/workflows/ci.yml`
- Node.js `>=18` is required only for the npm wrapper in `packaging/npm/package.json`
- Unix and Windows shell coverage are both part of CI in `.github/workflows/ci.yml`
- Primary deployment target is a standalone CLI distributed via crates.io, GitHub Releases, npm wrapper metadata, and Homebrew as documented in `README.md`, `.github/workflows/release.yml`, `packaging/npm/scripts/install.mjs`, and `packaging/homebrew/sxmc.rb`
- Prebuilt release assets are published for Linux x64, macOS x64, macOS arm64, and Windows x64 in `.github/workflows/release.yml` and consumed by `packaging/npm/scripts/install.mjs`
- Hosted runtime mode is optional and uses Axum HTTP servers for MCP and registry endpoints in `src/server/mod.rs`, `src/server/wrap.rs`, and `src/main.rs`
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

## Naming Patterns
- Use `snake_case` for Rust modules and standalone files such as `src/cli_args.rs`, `src/command_handlers.rs`, and `src/security/skill_scanner.rs`.
- Use `mod.rs` as the barrel for grouped modules such as `src/server/mod.rs`, `src/client/mod.rs`, and `src/output/mod.rs`.
- Keep helper scripts descriptive and lowercase with underscores in `scripts/`, for example `scripts/smoke_portable_core.sh` and `scripts/benchmark_startup.py`.
- Use kebab-case for skill fixture directories and package metadata where the external format expects it, for example `tests/fixtures/simple-skill/` and `packaging/npm/package.json`.
- Use `snake_case` for functions and methods throughout Rust code, for example `resolve_secret` in `src/auth/secrets.rs`, `inspect_codebase` in `src/client/codebase.rs`, and `format_tool_result` in `src/output/mod.rs`.
- Name tests as `test_<behavior>` or `test_<condition>_<expected_result>`, as in `test_split_frontmatter` in `src/skills/parser.rs` and `test_api_list_json_suppresses_detection_banner_on_stderr` in `tests/cli_integration.rs`.
- Prefer imperative helper names for setup and orchestration code, such as `spawn_http_server`, `wait_for_http_server`, and `write_discovery_snapshot` in `tests/cli_integration.rs`.
- Use `snake_case` for locals and parameters, for example `working_dir`, `skill_name`, `return_type`, and `recommended_commands` across `src/executor.rs`, `src/server/handler.rs`, and `src/client/codebase.rs`.
- Use short temporary names only inside tight scopes; otherwise favor descriptive names like `discovery_tool_manifests` in `src/cli_args.rs` and `stderr_capture` in `src/server/wrap.rs`.
- Use `PascalCase` for structs and enums such as `SxmcError` in `src/error.rs`, `SkillFrontmatter` in `src/skills/models.rs`, and `StructuredOutputFormat` in `src/output/mod.rs`.
- Use `SCREAMING_SNAKE_CASE` for cross-function constants, for example `PROFILE_BUNDLE_SCHEMA` in `src/main.rs` and `TOOL_GET_AVAILABLE_SKILLS` in `src/server/handler.rs`.
- When external file formats use kebab-case keys, encode that with serde attributes rather than Rust field naming changes, as in `#[serde(rename_all = "kebab-case")]` in `src/skills/models.rs`.
## Code Style
- Format Rust with `cargo fmt`. CI enforces `cargo fmt --all --check` in `.github/workflows/ci.yml`, and release certification repeats that in `scripts/certify_release.sh`.
- No `rustfmt.toml`, `.editorconfig`, or repository-level formatter config is present. Use default Rustfmt behavior.
- Shell scripts that act as CI or smoke entrypoints generally use `set -euo pipefail`, as in `scripts/certify_release.sh`, `scripts/smoke_portable_core.sh`, and `scripts/smoke_portable_fixtures.sh`.
- Python helper scripts are small, typed, and `main()`-driven, as in `scripts/benchmark_startup.py`.
- Node packaging scripts stay minimal and runtime-focused; there is no repo-level ESLint or Prettier config. Validation is syntax-only through `node --check` in `.github/workflows/ci.yml` and `scripts/certify_release.sh`.
- Treat Clippy warnings as errors. CI runs `cargo clippy --all-targets --all-features -- -D warnings` in `.github/workflows/ci.yml`.
- Keep non-Rust packaging files valid through lightweight tool-native checks instead of a separate lint stack: `node --check packaging/npm/bin/sxmc.js`, `node packaging/npm/scripts/install.mjs`, and `ruby -c packaging/homebrew/sxmc.rb` in `scripts/certify_release.sh`.
## Import Organization
- No alias system is used.
- Inside library code, import sibling modules with `crate::...`, as in `src/server/handler.rs`.
- Inside `src/main.rs`, import public crate APIs through `sxmc::...` and local private modules through bare `mod` plus `use`, as in `src/main.rs`.
## Error Handling
- Return the shared `crate::error::Result<T>` alias from library functions, as in `src/auth/secrets.rs`, `src/executor.rs`, `src/client/api.rs`, and `src/skills/parser.rs`.
- Convert lower-level failures into `SxmcError` with contextual strings via `map_err`, as in `src/client/codebase.rs`, `src/client/openapi.rs`, and `src/client/graphql.rs`.
- Use `ok_or_else` for missing required values and argument validation, as in `src/server/handler.rs` and `src/client/graphql.rs`.
- Use `unwrap()` freely in tests and small setup helpers, but keep production paths on explicit `Result` returns.
- Prefer structured error messages that include the failing path, command, or operation name. This pattern shows up across `src/auth/secrets.rs`, `src/executor.rs`, and `src/client/*`.
## Logging
- Write user-facing data and machine-readable payloads to stdout with `println!`, especially in `src/main.rs` and `src/command_handlers.rs`.
- Write warnings, progress notes, and transport diagnostics to stderr with `eprintln!`, often prefixed with `[sxmc]`, as in `src/server/mod.rs`, `src/server/wrap.rs`, and `src/cli_surfaces/inspect.rs`.
- Preserve stdout as the machine-readable channel when a command supports structured output. `tests/cli_integration.rs` explicitly checks this behavior in `test_api_list_json_suppresses_detection_banner_on_stderr`.
- No `tracing`, `log`, or `env_logger` integration is present in the repo.
## Comments
- Use doc comments on public modules and externally useful functions, as in `src/lib.rs`, `src/auth/secrets.rs`, and `src/output/mod.rs`.
- Add short inline comments only when explaining compatibility behavior, parsing edge cases, or interop constraints, for example the zero-argument MCP note in `src/client/mod.rs` and the JSON parsing fallback note in `src/main.rs`.
- In long pattern or shell files, use banner comments to divide sections, as in `src/security/patterns.rs` and `scripts/test-sxmc.sh`.
- Not used.
- JavaScript files in `packaging/npm/bin/sxmc.js` and `packaging/npm/scripts/install.mjs` rely on clear names and small function scope instead of docblock comments.
## Function Design
- Prefer focused helpers in library modules, such as `resolve_header` in `src/auth/secrets.rs`, `build_call_tool_params` in `src/client/mod.rs`, and `parse_argument_hint` in `src/skills/parser.rs`.
- Accept that orchestration-heavy files grow large when they centralize CLI routing. `src/main.rs`, `src/cli_args.rs`, and `src/cli_surfaces/inspect.rs` are current exceptions and should be extended with new helpers before adding more inline branching.
- Prefer borrowed inputs like `&Path`, `&str`, `&[String]`, and `&[PathBuf]`, as seen throughout `src/client/`, `src/auth/`, and `src/skills/`.
- Model command surfaces with enums and typed structs rather than raw strings when possible, for example `Commands` and subcommand enums in `src/cli_args.rs`.
- Use dedicated option structs when a helper needs several related knobs, as in `ApiCommandOptions` and `SkillListOptions` in `src/command_handlers.rs`.
- Return `serde_json::Value` for machine-readable command payloads and summaries, as in `src/client/codebase.rs`, `src/client/graphql.rs`, and many helpers in `src/main.rs`.
- Return typed structs or enums where the code needs richer internal modeling, such as `Skill`, `ExecResult`, `DiscoveryResource`, and `OpenApiOperation`.
## Module Design
- Declare the public surface centrally in `src/lib.rs` with one `pub mod` per subsystem and a short doc comment for each module.
- Use `mod.rs` to assemble grouped subsystems such as `src/client/mod.rs` and `src/server/mod.rs`.
- Keep binary-only helpers in `src/main.rs`, `src/cli_args.rs`, and `src/command_handlers.rs` rather than exporting them from the library.
- Use Rust `mod.rs` files as the only barrel pattern.
- Do not introduce extra re-export-only files outside the normal module system unless the API surface genuinely needs it.
## Practical Rules To Follow
- Add new Rust code under an existing subsystem and match that module’s import grouping, error style, and naming.
- Use `crate::error::Result<T>` and map errors with context instead of propagating raw library errors into user-facing code.
- Keep structured stdout clean; put diagnostics on stderr.
- Add doc comments only where they help readers understand a public API or a non-obvious edge case.
- For scripts under `scripts/`, prefer portable bash or small Python utilities over heavyweight tooling, and preserve the current `set -euo pipefail` / `main()` style.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

## Pattern Overview
- `src/main.rs` is the operational entrypoint and contains the top-level `clap` dispatch for the full `sxmc` command surface.
- Reusable behavior is pushed into library modules under `src/`, so the binary mostly orchestrates parsing, path resolution, output selection, and module calls.
- The product is organized around surface translation: skills to MCP in `src/skills/` + `src/server/`, MCP or APIs to CLI in `src/client/`, and CLI to AI host artifacts in `src/cli_surfaces/`.
## Layers
- Purpose: Parse arguments, choose the subcommand path, and glue together modules for runtime execution.
- Location: `src/main.rs`, `src/cli_args.rs`, `src/command_handlers.rs`
- Contains: `clap` enum definitions, shared option structs, top-level `match cli.command`, and reusable handlers for skills and API commands.
- Depends on: Nearly every exported module from `src/lib.rs`, especially `src/client/`, `src/server/`, `src/skills/`, `src/cli_surfaces/`, `src/paths.rs`, and `src/output/mod.rs`.
- Used by: The `sxmc` binary declared in `Cargo.toml`.
- Purpose: Discover skill folders, parse `SKILL.md`, enumerate `scripts/` and `references/`, and build in-memory skill models.
- Location: `src/skills/discovery.rs`, `src/skills/parser.rs`, `src/skills/models.rs`, `src/skills/install.rs`, `src/skills/generator.rs`
- Contains: Filesystem scans, YAML frontmatter parsing, argument-hint parsing, install metadata, and OpenAPI-to-skill generation.
- Depends on: `src/error.rs`, `src/paths.rs`, and the local filesystem.
- Used by: `src/server/mod.rs`, `src/server/handler.rs`, `src/command_handlers.rs`, and subcommands in `src/main.rs`.
- Purpose: Turn parsed skills or wrapped CLI profiles into MCP servers over stdio or streamable HTTP.
- Location: `src/server/mod.rs`, `src/server/handler.rs`, `src/server/wrap.rs`
- Contains: `SkillsServer`, HTTP auth and limit handling, watch/reload support, discovery snapshot mounting, and wrapped-CLI tool derivation.
- Depends on: `src/skills/`, `src/discovery_snapshots.rs`, `src/executor.rs`, `src/cli_surfaces/`, and `rmcp`/`axum`.
- Used by: `Commands::Serve` and `Commands::Wrap` in `src/main.rs`.
- Purpose: Normalize external sources into a common CLI interaction model.
- Location: `src/client/api.rs`, `src/client/openapi.rs`, `src/client/graphql.rs`, `src/client/mcp_stdio.rs`, `src/client/mcp_http.rs`, `src/client/database.rs`, `src/client/codebase.rs`, `src/client/traffic.rs`, `src/client/commands.rs`
- Contains: API autodetection, MCP transport clients, schema discovery, codebase inspection, HAR inspection, and shared command definitions.
- Depends on: `src/error.rs`, `src/output/mod.rs`, `src/projection.rs`, HTTP/MCP crates, and local files for schema/codebase discovery.
- Used by: `src/main.rs` and `src/command_handlers.rs`.
- Purpose: Inspect a real CLI into a normalized profile, then generate AI-facing docs, configs, wrappers, and supporting artifacts.
- Location: `src/cli_surfaces/inspect.rs`, `src/cli_surfaces/model.rs`, `src/cli_surfaces/render.rs`, `src/cli_surfaces/materialize.rs`
- Contains: command-spec parsing, profile caching, profile models, host registry, artifact rendering, and file materialization policies.
- Depends on: `src/cache.rs`, `src/paths.rs`, `src/error.rs`, and the filesystem.
- Used by: `inspect`, `init`, `scaffold`, `doctor`, `setup`, `add`, `status`, `sync`, and `wrap` flows in `src/main.rs`.
- Purpose: Provide durable local/global state, formatting, execution, and lightweight utilities.
- Location: `src/paths.rs`, `src/cache.rs`, `src/bake/config.rs`, `src/auth/secrets.rs`, `src/output/mod.rs`, `src/executor.rs`, `src/projection.rs`, `src/error.rs`
- Contains: local vs global install paths, cache TTL storage, baked connection persistence, secret/header resolution, output formatting, subprocess execution, JSON field projection, and the crate-wide error type.
- Depends on: filesystem, environment variables, and serde-based serialization.
- Used by: most modules in the crate.
- Purpose: Scan skills and MCP surfaces for risky content and report findings with severity.
- Location: `src/security/mod.rs`, `src/security/skill_scanner.rs`, `src/security/mcp_scanner.rs`, `src/security/patterns.rs`
- Contains: finding models, pattern heuristics, and report formatting.
- Depends on: `src/error.rs` and parsed skill or MCP metadata.
- Used by: `scan`-related flows in `src/main.rs`.
## Data Flow
- User-global state lives under `src/paths.rs` paths such as `~/.config/sxmc` and `~/.cache/sxmc`.
- Repo-local generated AI state lives under `.sxmc/` when `InstallScope::Local` is used via `src/paths.rs`.
- Baked connection definitions are stored through `BakeStore` in `src/bake/config.rs`.
- CLI inspection cache entries are file-based and TTL-driven in `src/cache.rs`.
## Key Abstractions
- Purpose: In-memory representation of one skill directory.
- Examples: `src/skills/models.rs`, constructed in `src/skills/parser.rs`
- Pattern: Parse once from filesystem, then reuse for listing, serving, running, and security scanning.
- Purpose: MCP handler for skills, discovery resources, and hybrid helper tools.
- Examples: `src/server/handler.rs`
- Pattern: Precompute indexes for skills, tools, and resources, then answer MCP list/read/call requests from those indexes.
- Purpose: Canonical JSON-friendly description of a CLI command surface.
- Examples: `src/cli_surfaces/model.rs`, produced by `src/cli_surfaces/inspect.rs`
- Pattern: Treat the normalized profile as source of truth, then derive docs, configs, wrappers, and drift workflows from it.
- Purpose: Persist reconnectable definitions for MCP and API sources.
- Examples: `src/bake/config.rs`
- Pattern: Load once from config storage, mutate in memory, and persist atomically through a temp file.
- Purpose: Shared operation model for API-backed CLI execution.
- Examples: `src/client/commands.rs`, consumed by `src/client/api.rs`
- Pattern: Different backends expose one list-and-execute shape to the CLI surface.
## Entry Points
- Location: `src/main.rs`
- Triggers: Running `sxmc ...`
- Responsibilities: Parse CLI arguments, branch on subcommands, resolve state roots, call library modules, and emit output.
- Location: `src/lib.rs`
- Triggers: Embedding `sxmc` as a Rust crate
- Responsibilities: Re-export stable internal modules such as `server`, `skills`, `client`, `cli_surfaces`, `security`, and `bake`.
- Location: `tests/cli_integration.rs`
- Triggers: `cargo test`
- Responsibilities: Exercise the compiled binary, spawn temporary HTTP/MCP fixtures, and validate end-to-end command flows.
## Error Handling
- Use `crate::error::Result<T>` and `SxmcError` from `src/error.rs` across non-protocol modules.
- Convert HTTP/MCP boundary failures into protocol-native errors inside `src/server/handler.rs` and `src/server/mod.rs`.
- Emit non-fatal warnings to stderr for partial failures during listing or optional behavior, especially in `src/command_handlers.rs` and `src/main.rs`.
## Cross-Cutting Concerns
<!-- GSD:architecture-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd:quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd:debug` for investigation and bug fixing
- `/gsd:execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->

<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd:profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
