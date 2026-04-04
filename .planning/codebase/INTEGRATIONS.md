# External Integrations

**Analysis Date:** 2026-04-04

## APIs & External Services

**MCP protocol:**
- MCP over stdio - Sumac spawns local MCP servers as subprocesses in `src/client/mcp_stdio.rs`
  - SDK/Client: `rmcp`
  - Auth: per-process env injection from baked configs in `src/bake/config.rs`
- MCP over streamable HTTP - Sumac connects to and serves remote MCP endpoints in `src/client/mcp_http.rs`, `src/server/mod.rs`, and `src/server/wrap.rs`
  - SDK/Client: `rmcp` + `reqwest` + `axum`
  - Auth: `--auth-header`, `--require-header`, and `--bearer-token` parsed in `src/cli_args.rs` and resolved in `src/auth/secrets.rs`

**HTTP APIs:**
- OpenAPI documents from local files or HTTP(S) URLs - auto-detected and executed in `src/client/api.rs` and `src/client/openapi.rs`
  - SDK/Client: `reqwest`
  - Auth: arbitrary headers passed through `--auth-header`
- GraphQL endpoints - schema introspection and operation execution in `src/client/graphql.rs`
  - SDK/Client: `reqwest`
  - Auth: arbitrary headers passed through `--auth-header`

**Distribution services:**
- GitHub Releases - release archives and checksums are built in `.github/workflows/release.yml` and downloaded by the npm wrapper in `packaging/npm/scripts/install.mjs`
  - SDK/Client: browser/HTTP download via Node `fetch`
  - Auth: public by default; custom mirror base can be supplied with `SXMC_NPM_DOWNLOAD_BASE`

**Notification endpoints:**
- Generic outbound webhooks - watch events are POSTed in `src/main.rs`
  - SDK/Client: `reqwest`
  - Auth: extra headers from `--notify-header`
- Slack-compatible webhooks - watch events can be reshaped to Slack payloads in `src/main.rs` and `docs/USAGE.md`
  - SDK/Client: `reqwest`
  - Auth: webhook URL plus optional headers

**AI host clients:**
- Generated startup docs and MCP config target multiple local AI tools in `src/cli_surfaces/model.rs`, `src/paths.rs`, and `src/cli_surfaces/render.rs`
  - Supported targets include Claude Code, Cursor, Gemini CLI, GitHub Copilot docs, Continue, OpenCode, JetBrains AI Assistant, Junie, Windsurf, OpenAI/Codex, and generic MCP profiles
  - Auth: host-specific files only; no repo-managed identity provider detected

## Data Storage

**Databases:**
- Application persistence: no app-owned database detected; Sumac stores its own state as JSON files in config/cache directories via `src/bake/config.rs`, `src/cache.rs`, and `src/paths.rs`
  - Connection: local filesystem
  - Client: Rust stdlib + `serde_json`
- Discovery support: SQLite and PostgreSQL schema inspection are first-class external targets in `src/client/database.rs`
  - Connection: SQLite path or PostgreSQL DSN passed to the CLI
  - Client: `rusqlite` and `postgres`

**File Storage:**
- Local filesystem only for skills, saved CLI profiles, generated host artifacts, caches, and profile bundles in `src/paths.rs`, `src/cache.rs`, `src/bake/config.rs`, and `src/main.rs`
- Optional file-based bundle publishing/pulling supports plain paths and `file://` URIs in `src/main.rs`

**Caching:**
- Local file cache under the resolved cache root from `src/paths.rs`, implemented in `src/cache.rs`
- No Redis, Memcached, or remote cache service detected

## Authentication & Identity

**Auth Provider:**
- Custom auth handling; no OAuth, SSO, or external identity provider detected
  - Implementation: header- and bearer-token-based auth for hosted MCP/registry HTTP endpoints in `src/server/mod.rs`, `src/server/wrap.rs`, and `src/main.rs`

**Secret Resolution:**
- Runtime secrets can be passed literally or resolved from `env:` / `file:` references in `src/auth/secrets.rs`
- Bundle signing supports HMAC secrets and Ed25519 key files in `src/main.rs`

## Monitoring & Observability

**Error Tracking:**
- None detected

**Logs:**
- CLI-oriented stdout/stderr output plus `eprintln!` server lifecycle logs in `src/server/mod.rs`, `src/server/wrap.rs`, and `src/main.rs`
- Health endpoints are exposed for hosted services at `/healthz` in `src/server/mod.rs` and `src/main.rs`

## CI/CD & Deployment

**Hosting:**
- Primary runtime is a local CLI binary built from `src/main.rs`
- Optional hosted services:
  - MCP server at `/mcp` with `/healthz` in `src/server/mod.rs`
  - Wrapped CLI MCP server with HTTP mode in `src/server/wrap.rs`
  - Bundle registry server with `/index.json`, `/bundles`, `/bundles/{name}`, and `/healthz` in `src/main.rs`

**CI Pipeline:**
- GitHub Actions in `.github/workflows/ci.yml` and `.github/workflows/release.yml`
- Distribution channels: crates.io, GitHub Releases, npm wrapper in `packaging/npm`, and Homebrew formula in `packaging/homebrew/sxmc.rb`

## Environment Configuration

**Required env vars:**
- No single required env var for the core CLI was detected
- Important optional vars:
  - `SXMC_CONFIG_HOME`, `SXMC_CACHE_HOME`, `XDG_CONFIG_HOME`, `XDG_CACHE_HOME` in `src/paths.rs`
  - `SXMC_MCP_TOKEN` for hosted MCP examples in `docs/OPERATIONS.md` and `docs/USAGE.md`
  - `SXMC_BUNDLE_SECRET` for bundle signing/verification examples in `docs/USAGE.md`
  - `SXMC_NPM_SKIP_DOWNLOAD` and `SXMC_NPM_DOWNLOAD_BASE` in `packaging/npm/scripts/install.mjs`

**Secrets location:**
- Not stored in-repo by default
- Expected at runtime via environment variables or external files referenced through `env:` / `file:` prefixes in `src/auth/secrets.rs`

## Webhooks & Callbacks

**Incoming:**
- MCP HTTP server:
  - `/mcp` and `/healthz` in `src/server/mod.rs`
- Wrapped CLI HTTP MCP server:
  - `/mcp` and `/healthz` in `src/server/wrap.rs`
- Bundle registry server:
  - `/index.json`, `PUT /bundles`, `GET /bundles/{name}`, and `/healthz` in `src/main.rs`

**Outgoing:**
- Watch notifications to arbitrary webhook URLs and Slack-compatible webhooks in `src/main.rs`
- Bundle publish to arbitrary HTTP(S) targets with HTTP `PUT` in `src/main.rs`
- Bundle pull and registry sync from arbitrary HTTP(S) JSON endpoints with HTTP `GET` in `src/main.rs`
- OpenAPI, GraphQL, and remote MCP calls over HTTP(S) in `src/client/openapi.rs`, `src/client/graphql.rs`, and `src/client/mcp_http.rs`

---

*Integration audit: 2026-04-04*
