# Technology Stack

**Analysis Date:** 2026-04-04

## Languages

**Primary:**
- Rust 2021 edition - application code and CLI entrypoints in `Cargo.toml`, `src/main.rs`, and `src/lib.rs`

**Secondary:**
- JavaScript (Node.js ESM) - npm install wrapper and launcher metadata in `packaging/npm/package.json` and `packaging/npm/scripts/install.mjs`
- Shell + PowerShell - release, smoke, and CI automation in `.github/workflows/ci.yml`, `.github/workflows/release.yml`, and `scripts/`
- Python - helper scripts such as `scripts/fixture_inventory.py` and `scripts/benchmark_startup.py`

## Runtime

**Environment:**
- Native Rust CLI binary `sxmc` built from `src/main.rs`
- Async runtime: Tokio 1 with `full` features in `Cargo.toml`
- HTTP serving/client runtime: Axum 0.8 + Reqwest 0.13 in `src/server/mod.rs`, `src/server/wrap.rs`, `src/client/api.rs`, `src/client/openapi.rs`, and `src/client/graphql.rs`

**Package Manager:**
- Cargo via the stable Rust toolchain used in `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- Lockfile: present in `Cargo.lock`
- Secondary package manager: npm for distribution wrapper only in `packaging/npm/package.json`

## Frameworks

**Core:**
- `rmcp` 1.2 - MCP client/server transport layer for stdio and streamable HTTP in `src/server/mod.rs`, `src/server/wrap.rs`, `src/client/mcp_stdio.rs`, and `src/client/mcp_http.rs`
- `clap` 4 - CLI argument parsing in `src/cli_args.rs`
- `serde` / `serde_json` / `serde_yaml` / `toml` - structured config, spec, and artifact parsing in `src/client/`, `src/bake/config.rs`, and `src/cli_surfaces/`

**Testing:**
- Rust built-in test harness via `cargo test` with integration coverage in `tests/cli_integration.rs`
- `assert_cmd` 2 and `predicates` 3 - CLI assertions in `Cargo.toml` and `tests/cli_integration.rs`
- Shell smoke suites in `scripts/test-sxmc.sh`, `scripts/smoke_portable_core.sh`, and `scripts/smoke_test_clients.sh`

**Build/Dev:**
- GitHub Actions - CI and release packaging in `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- `clap_complete` 4 - shell completion generation wired through `src/cli_args.rs`
- `notify` 8 - file watch/reload support in `src/server/mod.rs`

## Key Dependencies

**Critical:**
- `rmcp` 1.2 - the MCP protocol backbone for Sumac’s core product surface in `Cargo.toml`
- `reqwest` 0.13 with `rustls` - all outbound HTTP for MCP, OpenAPI, GraphQL, bundle publish/pull, registry push/pull, npm install downloads, and webhooks in `src/client/`, `src/main.rs`, and `packaging/npm/scripts/install.mjs`
- `axum` 0.8 - inbound HTTP endpoints for hosted MCP and bundle registry servers in `src/server/mod.rs`, `src/server/wrap.rs`, and `src/main.rs`
- `clap` 4 - the large multi-command CLI surface in `src/cli_args.rs`
- `tokio` 1 - async execution, child processes, and HTTP serving in `src/main.rs`, `src/server/`, and `src/client/`

**Infrastructure:**
- `rusqlite` 0.32 with `bundled` - SQLite schema inspection for discovery in `src/client/database.rs`
- `postgres` 0.19 - PostgreSQL schema inspection for discovery in `src/client/database.rs`
- `hmac` 0.12 + `sha2` 0.10 + `ed25519-dalek` 2 - bundle signing and verification in `src/main.rs`
- `dirs` 6 + `tempfile` 3 - local config/cache placement and atomic writes in `src/paths.rs`, `src/bake/config.rs`, and `src/cache.rs`

## Configuration

**Environment:**
- Local/global state roots are derived from `SXMC_CONFIG_HOME`, `SXMC_CACHE_HOME`, `XDG_CONFIG_HOME`, and `XDG_CACHE_HOME` in `src/paths.rs`
- HTTP auth and signing secrets are supplied at runtime, typically via `env:` or `file:` indirection handled in `src/auth/secrets.rs`
- NPM wrapper install behavior is controlled by `SXMC_NPM_SKIP_DOWNLOAD` and `SXMC_NPM_DOWNLOAD_BASE` in `packaging/npm/scripts/install.mjs`

**Build:**
- Rust manifest and dependency graph: `Cargo.toml` and `Cargo.lock`
- NPM wrapper metadata: `packaging/npm/package.json`
- Homebrew packaging: `packaging/homebrew/sxmc.rb`
- CI/release automation: `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- No pinned `rust-toolchain` file, `.nvmrc`, Dockerfile, or compose file detected at repo root

## Platform Requirements

**Development:**
- Rust stable toolchain with Cargo is required for the canonical install/build path in `README.md` and `.github/workflows/ci.yml`
- Node.js `>=18` is required only for the npm wrapper in `packaging/npm/package.json`
- Unix and Windows shell coverage are both part of CI in `.github/workflows/ci.yml`

**Production:**
- Primary deployment target is a standalone CLI distributed via crates.io, GitHub Releases, npm wrapper metadata, and Homebrew as documented in `README.md`, `.github/workflows/release.yml`, `packaging/npm/scripts/install.mjs`, and `packaging/homebrew/sxmc.rb`
- Prebuilt release assets are published for Linux x64, macOS x64, macOS arm64, and Windows x64 in `.github/workflows/release.yml` and consumed by `packaging/npm/scripts/install.mjs`
- Hosted runtime mode is optional and uses Axum HTTP servers for MCP and registry endpoints in `src/server/mod.rs`, `src/server/wrap.rs`, and `src/main.rs`

---

*Stack analysis: 2026-04-04*
