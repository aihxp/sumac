# Coding Conventions

**Analysis Date:** 2026-04-04

## Naming Patterns

**Files:**
- Use `snake_case` for Rust modules and standalone files such as `src/cli_args.rs`, `src/command_handlers.rs`, and `src/security/skill_scanner.rs`.
- Use `mod.rs` as the barrel for grouped modules such as `src/server/mod.rs`, `src/client/mod.rs`, and `src/output/mod.rs`.
- Keep helper scripts descriptive and lowercase with underscores in `scripts/`, for example `scripts/smoke_portable_core.sh` and `scripts/benchmark_startup.py`.
- Use kebab-case for skill fixture directories and package metadata where the external format expects it, for example `tests/fixtures/simple-skill/` and `packaging/npm/package.json`.

**Functions:**
- Use `snake_case` for functions and methods throughout Rust code, for example `resolve_secret` in `src/auth/secrets.rs`, `inspect_codebase` in `src/client/codebase.rs`, and `format_tool_result` in `src/output/mod.rs`.
- Name tests as `test_<behavior>` or `test_<condition>_<expected_result>`, as in `test_split_frontmatter` in `src/skills/parser.rs` and `test_api_list_json_suppresses_detection_banner_on_stderr` in `tests/cli_integration.rs`.
- Prefer imperative helper names for setup and orchestration code, such as `spawn_http_server`, `wait_for_http_server`, and `write_discovery_snapshot` in `tests/cli_integration.rs`.

**Variables:**
- Use `snake_case` for locals and parameters, for example `working_dir`, `skill_name`, `return_type`, and `recommended_commands` across `src/executor.rs`, `src/server/handler.rs`, and `src/client/codebase.rs`.
- Use short temporary names only inside tight scopes; otherwise favor descriptive names like `discovery_tool_manifests` in `src/cli_args.rs` and `stderr_capture` in `src/server/wrap.rs`.

**Types:**
- Use `PascalCase` for structs and enums such as `SxmcError` in `src/error.rs`, `SkillFrontmatter` in `src/skills/models.rs`, and `StructuredOutputFormat` in `src/output/mod.rs`.
- Use `SCREAMING_SNAKE_CASE` for cross-function constants, for example `PROFILE_BUNDLE_SCHEMA` in `src/main.rs` and `TOOL_GET_AVAILABLE_SKILLS` in `src/server/handler.rs`.
- When external file formats use kebab-case keys, encode that with serde attributes rather than Rust field naming changes, as in `#[serde(rename_all = "kebab-case")]` in `src/skills/models.rs`.

## Code Style

**Formatting:**
- Format Rust with `cargo fmt`. CI enforces `cargo fmt --all --check` in `.github/workflows/ci.yml`, and release certification repeats that in `scripts/certify_release.sh`.
- No `rustfmt.toml`, `.editorconfig`, or repository-level formatter config is present. Use default Rustfmt behavior.
- Shell scripts that act as CI or smoke entrypoints generally use `set -euo pipefail`, as in `scripts/certify_release.sh`, `scripts/smoke_portable_core.sh`, and `scripts/smoke_portable_fixtures.sh`.
- Python helper scripts are small, typed, and `main()`-driven, as in `scripts/benchmark_startup.py`.
- Node packaging scripts stay minimal and runtime-focused; there is no repo-level ESLint or Prettier config. Validation is syntax-only through `node --check` in `.github/workflows/ci.yml` and `scripts/certify_release.sh`.

**Linting:**
- Treat Clippy warnings as errors. CI runs `cargo clippy --all-targets --all-features -- -D warnings` in `.github/workflows/ci.yml`.
- Keep non-Rust packaging files valid through lightweight tool-native checks instead of a separate lint stack: `node --check packaging/npm/bin/sxmc.js`, `node packaging/npm/scripts/install.mjs`, and `ruby -c packaging/homebrew/sxmc.rb` in `scripts/certify_release.sh`.

## Import Organization

**Order:**
1. Standard library imports first, for example `use std::path::PathBuf;` in `src/cli_args.rs` and `use std::collections::BTreeSet;` in `src/client/codebase.rs`.
2. Third-party crate imports second, for example `use clap::{Parser, Subcommand, ValueEnum};` in `src/cli_args.rs` and `use serde_json::{json, Value};` in `src/client/codebase.rs`.
3. Internal crate imports last, using `crate::...` inside library modules and `sxmc::...` from the binary, for example `use crate::error::{Result, SxmcError};` in `src/auth/secrets.rs` and `use sxmc::client::{api, codebase, database, graphql, mcp_http, mcp_stdio, openapi, traffic};` in `src/main.rs`.

**Path Aliases:**
- No alias system is used.
- Inside library code, import sibling modules with `crate::...`, as in `src/server/handler.rs`.
- Inside `src/main.rs`, import public crate APIs through `sxmc::...` and local private modules through bare `mod` plus `use`, as in `src/main.rs`.

## Error Handling

**Patterns:**
- Return the shared `crate::error::Result<T>` alias from library functions, as in `src/auth/secrets.rs`, `src/executor.rs`, `src/client/api.rs`, and `src/skills/parser.rs`.
- Convert lower-level failures into `SxmcError` with contextual strings via `map_err`, as in `src/client/codebase.rs`, `src/client/openapi.rs`, and `src/client/graphql.rs`.
- Use `ok_or_else` for missing required values and argument validation, as in `src/server/handler.rs` and `src/client/graphql.rs`.
- Use `unwrap()` freely in tests and small setup helpers, but keep production paths on explicit `Result` returns.
- Prefer structured error messages that include the failing path, command, or operation name. This pattern shows up across `src/auth/secrets.rs`, `src/executor.rs`, and `src/client/*`.

## Logging

**Framework:** `println!` / `eprintln!`

**Patterns:**
- Write user-facing data and machine-readable payloads to stdout with `println!`, especially in `src/main.rs` and `src/command_handlers.rs`.
- Write warnings, progress notes, and transport diagnostics to stderr with `eprintln!`, often prefixed with `[sxmc]`, as in `src/server/mod.rs`, `src/server/wrap.rs`, and `src/cli_surfaces/inspect.rs`.
- Preserve stdout as the machine-readable channel when a command supports structured output. `tests/cli_integration.rs` explicitly checks this behavior in `test_api_list_json_suppresses_detection_banner_on_stderr`.
- No `tracing`, `log`, or `env_logger` integration is present in the repo.

## Comments

**When to Comment:**
- Use doc comments on public modules and externally useful functions, as in `src/lib.rs`, `src/auth/secrets.rs`, and `src/output/mod.rs`.
- Add short inline comments only when explaining compatibility behavior, parsing edge cases, or interop constraints, for example the zero-argument MCP note in `src/client/mod.rs` and the JSON parsing fallback note in `src/main.rs`.
- In long pattern or shell files, use banner comments to divide sections, as in `src/security/patterns.rs` and `scripts/test-sxmc.sh`.

**JSDoc/TSDoc:**
- Not used.
- JavaScript files in `packaging/npm/bin/sxmc.js` and `packaging/npm/scripts/install.mjs` rely on clear names and small function scope instead of docblock comments.

## Function Design

**Size:**
- Prefer focused helpers in library modules, such as `resolve_header` in `src/auth/secrets.rs`, `build_call_tool_params` in `src/client/mod.rs`, and `parse_argument_hint` in `src/skills/parser.rs`.
- Accept that orchestration-heavy files grow large when they centralize CLI routing. `src/main.rs`, `src/cli_args.rs`, and `src/cli_surfaces/inspect.rs` are current exceptions and should be extended with new helpers before adding more inline branching.

**Parameters:**
- Prefer borrowed inputs like `&Path`, `&str`, `&[String]`, and `&[PathBuf]`, as seen throughout `src/client/`, `src/auth/`, and `src/skills/`.
- Model command surfaces with enums and typed structs rather than raw strings when possible, for example `Commands` and subcommand enums in `src/cli_args.rs`.
- Use dedicated option structs when a helper needs several related knobs, as in `ApiCommandOptions` and `SkillListOptions` in `src/command_handlers.rs`.

**Return Values:**
- Return `serde_json::Value` for machine-readable command payloads and summaries, as in `src/client/codebase.rs`, `src/client/graphql.rs`, and many helpers in `src/main.rs`.
- Return typed structs or enums where the code needs richer internal modeling, such as `Skill`, `ExecResult`, `DiscoveryResource`, and `OpenApiOperation`.

## Module Design

**Exports:**
- Declare the public surface centrally in `src/lib.rs` with one `pub mod` per subsystem and a short doc comment for each module.
- Use `mod.rs` to assemble grouped subsystems such as `src/client/mod.rs` and `src/server/mod.rs`.
- Keep binary-only helpers in `src/main.rs`, `src/cli_args.rs`, and `src/command_handlers.rs` rather than exporting them from the library.

**Barrel Files:**
- Use Rust `mod.rs` files as the only barrel pattern.
- Do not introduce extra re-export-only files outside the normal module system unless the API surface genuinely needs it.

## Practical Rules To Follow

- Add new Rust code under an existing subsystem and match that module’s import grouping, error style, and naming.
- Use `crate::error::Result<T>` and map errors with context instead of propagating raw library errors into user-facing code.
- Keep structured stdout clean; put diagnostics on stderr.
- Add doc comments only where they help readers understand a public API or a non-obvious edge case.
- For scripts under `scripts/`, prefer portable bash or small Python utilities over heavyweight tooling, and preserve the current `set -euo pipefail` / `main()` style.

---

*Convention analysis: 2026-04-04*
