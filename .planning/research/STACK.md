# Technology Stack

**Project:** Sumac (`sxmc`)
**Research scope:** Brownfield internal core/app rewrite for a mature Rust CLI/MCP product
**Researched:** 2026-04-04
**Overall confidence:** HIGH

## Recommendation

Sumac should keep its current Rust transport stack and introduce only narrow additions. The rewrite target is not a new platform; it is a cleaner internal boundary inside the existing product. Keep `clap`, `tokio`, `serde`, `reqwest`, `axum`, and `rmcp` as the external-facing stack. Add only the tooling needed to make the migration safe: structured tracing, golden contract tests, and a pinned Rust toolchain.

The first rewrite slice should stay in a single crate. Introduce logical layers in code before introducing crate boundaries. Sumac already has a stable `1.x` CLI and JSON surface, and the main architectural risk is boundary churn, not missing framework capability. `src/main.rs` is over 11k lines and the codebase relies heavily on `serde_json::Value`; that argues for contract isolation and orchestration cleanup, not a workspace split.

## Recommended Stack

### Keep As-Is

| Technology | Version line | Purpose | Recommendation | Confidence |
|------------|--------------|---------|----------------|------------|
| Rust + Cargo | Stable toolchain, pinned in repo | Core language/runtime | Keep Rust as the only implementation language for the rewrite. Add `rust-toolchain.toml` to pin the toolchain for migration stability. Keep the initial migration compatible with the current edition; treat any edition bump as separate work. | HIGH |
| `clap` | 4.x | Public CLI parsing and help surface | Keep. The rewrite must preserve CLI contracts, so changing the parser stack now would create needless help/UX drift. | HIGH |
| `tokio` | 1.x | Async runtime | Keep. It already matches `reqwest`, `axum`, and `rmcp`, and there is no benefit in changing runtimes during an internal rewrite. | HIGH |
| `serde` / `serde_json` / `serde_yaml` / `toml` | 1.x / current major lines | Config and contract serialization | Keep, but move stable external DTOs into explicit contract modules instead of using raw `Value` as the application lingua franca. | HIGH |
| `reqwest` + `rustls` | 0.13.x | Outbound HTTP | Keep. Reuse pooled clients in adapters; do not let the new core/app layer construct HTTP clients directly. | HIGH |
| `axum` + `tower` | 0.8.x / 0.5.x | Hosted HTTP/MCP serving | Keep. It is already aligned with Hyper 1-era Rust web stack and does not need replacement for this migration. | HIGH |
| `rmcp` | 1.x | MCP client/server transport and protocol | Keep. Upgrade within the same major line when convenient, but do not make MCP SDK churn part of the boundary rewrite. | HIGH |
| `thiserror` | 2.x | Typed internal errors | Keep and standardize on it for core/app/adapters. | HIGH |

### Add Narrowly

| Addition | Version line | Purpose | Why it fits this rewrite | Confidence |
|----------|--------------|---------|--------------------------|------------|
| `tracing` | 0.1.x | Structured internal diagnostics | Add instrumentation at command-family, adapter, and rewrite seam boundaries. This helps compare legacy and rewritten flows without changing user-visible output. | HIGH |
| `tracing-subscriber` | 0.3.x | Trace/log initialization and filtering | Add one subscriber setup in the binary boundary only. This replaces ad hoc internal diagnostics without forcing a product-wide UX change. | HIGH |
| `insta` | 1.47.x | Golden snapshots for JSON outputs and generated files | Add for parity testing on `setup`, `add`, `status`, `sync`, and materialized artifacts. This is the most direct guardrail for a stable-contract rewrite. | HIGH |
| `trycmd` | 1.2.x | End-to-end CLI contract tests | Add for help text, stdout/stderr, and exit-code cases that should remain stable through the migration. It complements existing `assert_cmd` coverage. | HIGH |
| `rust-toolchain.toml` | current stable pin | Reproducible builds and CI/dev parity | Add at repo root. The rewrite should not depend on each contributor’s local default toolchain. | HIGH |

## Internal Stack Boundaries

The rewrite should introduce boundaries in this order, without changing the public surface:

| Layer | Responsibility | Allowed dependencies | Must not depend on |
|------|----------------|----------------------|--------------------|
| CLI/transport shell | `clap` parsing, TTY decisions, stdout/stderr rendering, process exit codes, trace init | `clap`, output renderers, `tracing-subscriber` | Business rules, filesystem orchestration, direct HTTP/MCP logic |
| App layer | Command-family orchestration for `setup`, `add`, `status`, `sync`; transaction-style use cases; parity cutover seam | `tokio`, typed request/response structs, `thiserror`, narrow internal ports | `clap`, `axum`, `rmcp`, `reqwest`, direct env/path lookups |
| Core layer | Pure policies and models: host selection, reconciliation rules, write plans, diff logic, contract-neutral state transitions | std, small pure helpers, typed domain models | `tokio`, networking, filesystem, subprocesses, raw JSON transport values |
| Adapters/infra | Filesystem, process, HTTP, MCP, cache, secrets, time, path resolution | `reqwest`, `axum`, `rmcp`, `tokio`, existing path/auth modules, `tracing` | `clap` and presentation formatting |
| Contract modules | Stable JSON/file DTOs for CLI output and generated artifacts | `serde`, schema helpers if needed | Core policy logic and transport-specific code |

## Prescriptive Implementation Choices

### 1. Keep one crate first

Do not split Sumac into a multi-crate workspace in the first migration phases. Use internal modules such as `src/app/`, `src/core/`, `src/adapters/`, and `src/contracts/` first. A workspace split can come later if boundaries prove stable. For this rewrite, extra crates would mostly add movement cost.

### 2. Standardize error boundaries

Use `thiserror` for typed errors below the CLI shell. Keep error-to-exit-code and error-to-rendered-output mapping in the binary/app boundary. `anyhow` is currently declared but not used in the repo; do not expand its use during the rewrite.

### 3. Stop spreading `serde_json::Value`

The current codebase uses `Value` pervasively. The rewrite should not continue that pattern into the new app/core seam. Define typed request/response structs for the first migrated command family, then serialize at the contract boundary. Keep `Value` only where Sumac is genuinely proxying dynamic external payloads.

### 4. Add tracing only at seams

Instrument:
- command entry/exit
- adapter calls to filesystem, subprocess, HTTP, and MCP
- legacy-vs-new cutover decisions
- parity test diagnostics

Do not replace user-facing `println!`/`eprintln!` output with trace output. Tracing is for internal observability, not a CLI UX rewrite.

### 5. Add parity-first tests before broad extraction

Use:
- `trycmd` for CLI behavior and help snapshots
- `insta` for JSON outputs and generated files
- existing `assert_cmd` integration tests where custom setup is already working

This combination fits a brownfield rewrite better than inventing a new test harness.

## What Not To Add

| Do not add | Why not |
|------------|---------|
| Multi-crate workspace split as phase 1 | Too much movement for too little safety; it couples architecture cleanup to packaging churn. |
| New async/runtime stack | `tokio` already matches the rest of the ecosystem; changing runtimes adds risk without payoff. |
| DI container or service locator framework | The new app layer should use explicit constructors and narrow ports. Rust does not need a container here. |
| Repo-wide `async-trait` abstraction layer | Modern Rust supports `async fn` in traits, but official guidance still warns about public trait limitations and object safety. Sumac’s rewrite should prefer concrete types or private/narrow traits instead of an abstraction spree. |
| ORM/config framework restack | This rewrite is not schema-heavy application development; existing lightweight serde/path handling is sufficient. |
| Public-contract changes hidden inside the rewrite | Stable JSON, files, and help text are the constraint. Treat contract changes as explicit product work, not refactor fallout. |

## Suggested Version Posture

| Area | Recommendation |
|------|----------------|
| Rust edition | Keep the first rewrite slice compatible with the repo’s current edition posture. Consider Edition 2024 only after the parity harness is in place and the first core/app slice lands cleanly. |
| Dependency upgrades | Allow patch/minor updates inside existing major lines. Prefer `rmcp` 1.3.x when touched, but do not make version chasing a prerequisite for the rewrite. |
| CI toolchain | Pin stable Rust in `rust-toolchain.toml` and use the same toolchain in CI. |

## Final Recommendation

For this milestone, Sumac should remain a single Rust CLI binary with the same transport stack and only four stack additions:

1. `rust-toolchain.toml`
2. `tracing`
3. `tracing-subscriber`
4. contract-focused test tooling: `insta` and `trycmd`

Everything else should be boundary work, not stack churn. The rewrite should create a clean `core -> app -> adapters -> CLI/contracts` shape inside the existing repo, migrate the golden onboarding path first, and defer any edition flip, workspace split, or SDK restack until after parity is proven.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Keep current Rust transport stack | HIGH | Strong fit with current codebase and mature 2026 ecosystem. |
| Add tracing + tracing-subscriber | HIGH | Standard incremental observability addition with low migration risk. |
| Add insta + trycmd for parity | HIGH | Strong match for stable CLI/JSON/file contract preservation. |
| Delay workspace split / edition bump | HIGH | Best fit for brownfield rewrite risk profile; based on codebase shape and migration goal. |

## Sources

- Rust 2024 Edition Guide: https://doc.rust-lang.org/stable/edition-guide/rust-2024/index.html
- Rust Blog, `async fn` and RPIT in traits guidance: https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits/
- `rmcp` docs.rs, latest crate line and transport guidance: https://docs.rs/rmcp/latest/rmcp/
- `reqwest` docs.rs, current crate line and client guidance: https://docs.rs/reqwest/latest/reqwest/
- `insta` docs.rs, snapshot testing and JSON/YAML snapshot support: https://docs.rs/insta/latest/insta/
- `trycmd` docs.rs, CLI snapshot testing harness: https://docs.rs/crate/trycmd/latest

## Evidence From Current Repo

- `src/main.rs` is 11,209 lines on 2026-04-04, which makes boundary cleanup more urgent than stack replacement.
- `anyhow` is declared in `Cargo.toml` but not used in `src/`, `tests/`, `packaging/`, or `.github/`.
- `serde_json::Value` is pervasive across `src/main.rs`, `src/client/`, `src/server/`, and `src/output/`; the rewrite should explicitly contain that pattern rather than spread it into the new core.
