# Codebase Structure

**Analysis Date:** 2026-04-04

## Directory Layout

```text
sxmc/
├── src/                 # Rust crate: binary entrypoint plus reusable library modules
├── tests/               # Integration tests and CLI/MCP fixtures
├── docs/                # Product, usage, architecture, validation, and operations docs
├── examples/            # Sample generated profiles, client configs, and agent-doc snippets
├── packaging/           # Distribution wrappers for npm and Homebrew
├── schemas/             # JSON schema artifacts for generated profile output
├── .github/workflows/   # CI and release automation
├── .planning/codebase/  # Generated codebase mapping documents
├── Cargo.toml           # Crate manifest and binary declaration
└── README.md            # Top-level product and command documentation
```

## Directory Purposes

**`src/`:**
- Purpose: All production Rust code for the `sxmc` crate and binary.
- Contains: top-level modules like `src/main.rs`, `src/lib.rs`, `src/cli_args.rs`, `src/command_handlers.rs`, and feature folders such as `src/client/`, `src/server/`, `src/skills/`, `src/cli_surfaces/`, `src/security/`, `src/auth/`, and `src/bake/`.
- Key files: `src/main.rs`, `src/lib.rs`, `src/paths.rs`, `src/error.rs`

**`src/client/`:**
- Purpose: Incoming-surface adapters that let `sxmc` talk to MCP, APIs, databases, codebases, and traffic captures.
- Contains: transport-specific clients and normalization helpers.
- Key files: `src/client/mcp_stdio.rs`, `src/client/mcp_http.rs`, `src/client/openapi.rs`, `src/client/graphql.rs`, `src/client/api.rs`

**`src/server/`:**
- Purpose: Outgoing MCP serving behavior.
- Contains: skill-backed server logic and wrapped-CLI server logic.
- Key files: `src/server/mod.rs`, `src/server/handler.rs`, `src/server/wrap.rs`

**`src/skills/`:**
- Purpose: Skill discovery, parsing, metadata, installation, and generation.
- Contains: scanners for `SKILL.md`, `scripts/`, and `references/`.
- Key files: `src/skills/discovery.rs`, `src/skills/parser.rs`, `src/skills/models.rs`, `src/skills/install.rs`

**`src/cli_surfaces/`:**
- Purpose: CLI inspection and AI-host artifact generation.
- Contains: profile models, cache-backed inspection, rendering, and file materialization.
- Key files: `src/cli_surfaces/inspect.rs`, `src/cli_surfaces/model.rs`, `src/cli_surfaces/render.rs`, `src/cli_surfaces/materialize.rs`

**`src/security/`:**
- Purpose: Security scanning for skill content and MCP surfaces.
- Contains: finding models, scanners, and pattern libraries.
- Key files: `src/security/mod.rs`, `src/security/skill_scanner.rs`, `src/security/mcp_scanner.rs`, `src/security/patterns.rs`

**`tests/`:**
- Purpose: End-to-end verification against the compiled CLI.
- Contains: one large integration suite plus fixture assets.
- Key files: `tests/cli_integration.rs`, `tests/fixtures/README.md`, `tests/fixtures/stateful_mcp_server.py`

**`docs/`:**
- Purpose: Human-maintained product and operational documentation.
- Contains: usage guides, product contract, architecture notes, validation, and release/ops instructions.
- Key files: `docs/ARCHITECTURE.md`, `docs/USAGE.md`, `docs/OPERATIONS.md`, `docs/VALIDATION.md`

**`examples/`:**
- Purpose: Checked-in samples of generated outputs and starter configs.
- Contains: client config examples, sample profiles, and agent-doc snippets.
- Key files: `examples/clients/cursor-mcp.json`, `examples/clients/codex-mcp.toml`, `examples/profiles/from_cli.json`

**`packaging/`:**
- Purpose: Non-Cargo distribution metadata and wrapper tooling.
- Contains: npm wrapper packaging and a Homebrew formula.
- Key files: `packaging/npm/package.json`, `packaging/npm/bin/sxmc.js`, `packaging/homebrew/sxmc.rb`

## Key File Locations

**Entry Points:**
- `src/main.rs`: Binary entrypoint and top-level command dispatch.
- `src/lib.rs`: Public library surface for embedding `sxmc` as a crate.
- `tests/cli_integration.rs`: Integration harness that exercises the binary as a user would.

**Configuration:**
- `Cargo.toml`: crate metadata, dependencies, and binary declaration.
- `.github/workflows/ci.yml`: CI test and validation pipeline.
- `.github/workflows/release.yml`: release packaging pipeline.
- `packaging/npm/package.json`: npm wrapper metadata.
- `schemas/cli_surface_v1.schema.json`: serialized profile contract.

**Core Logic:**
- `src/cli_args.rs`: `clap` subcommand and flag definitions.
- `src/command_handlers.rs`: extracted reusable handlers for selected command families.
- `src/paths.rs`: local/global state and artifact target resolution.
- `src/discovery_snapshots.rs`: loading and exposing saved discovery artifacts.

**Testing:**
- `tests/cli_integration.rs`: end-to-end command tests.
- `tests/fixtures/simple-skill/SKILL.md`: minimal skill fixture.
- `tests/fixtures/skill-with-scripts/scripts/hello.sh`: script-backed skill fixture.
- `tests/fixtures/malicious-skill/SKILL.md`: security-scanner fixture.

## Naming Conventions

**Files:**
- Rust source files use `snake_case.rs` naming such as `src/cli_args.rs`, `src/discovery_snapshots.rs`, and `src/client/mcp_http.rs`.
- Feature directories use lowercase names that match the exported module name, such as `src/server/` and `src/cli_surfaces/`.

**Directories:**
- Group related modules under a folder with `mod.rs` only when the feature spans multiple files, such as `src/client/mod.rs`, `src/server/mod.rs`, `src/skills/mod.rs`, and `src/cli_surfaces/mod.rs`.
- Keep small cross-cutting helpers as flat files directly under `src/`, such as `src/error.rs`, `src/paths.rs`, `src/cache.rs`, and `src/projection.rs`.

## Where to Add New Code

**New Feature:**
- Primary code: add `clap` flags and subcommands in `src/cli_args.rs`, dispatch in `src/main.rs`, and place reusable logic in the relevant feature module under `src/`.
- Tests: extend `tests/cli_integration.rs` and add new fixture files under `tests/fixtures/` when the feature needs external inputs.

**New Component/Module:**
- Implementation: add a new file under the feature folder that owns the behavior, then re-export it from that folder’s `mod.rs` if the rest of the crate needs access.
- Use `src/client/` for new inbound adapters, `src/server/` for MCP serving behavior, `src/skills/` for skill lifecycle changes, `src/cli_surfaces/` for CLI-inspection or artifact-generation work, and `src/security/` for scanner rules.

**Utilities:**
- Shared helpers: place cross-cutting, low-domain utilities at the `src/` root beside `src/error.rs`, `src/paths.rs`, `src/cache.rs`, `src/output/mod.rs`, and `src/projection.rs`.
- Avoid adding more generic helpers to `src/main.rs`; move reusable logic into library modules first and keep `src/main.rs` as orchestration code.

**Common Extension Paths:**
- New AI host target: update `src/cli_surfaces/model.rs`, `src/cli_surfaces/render.rs`, `src/cli_surfaces/materialize.rs`, and `src/paths.rs`.
- New API backend: start in `src/client/` and wire it through `src/client/api.rs`.
- New MCP server behavior: start in `src/server/mod.rs` or `src/server/wrap.rs`.
- New persisted state file: add path resolution in `src/paths.rs` and storage logic near the owning feature, not in `src/main.rs`.

## Special Directories

**`target/`:**
- Purpose: Rust build outputs, generated docs, and packaged artifacts.
- Generated: Yes
- Committed: No

**`.planning/codebase/`:**
- Purpose: Generated architecture, structure, conventions, testing, and concern maps for GSD workflows.
- Generated: Yes
- Committed: No files are tracked here in the current repo state.

**`examples/`:**
- Purpose: Checked-in examples of generated artifacts and config snippets.
- Generated: No
- Committed: Yes

**`packaging/`:**
- Purpose: Distribution-specific wrappers and formulas outside the Rust crate.
- Generated: No
- Committed: Yes

---

*Structure analysis: 2026-04-04*
