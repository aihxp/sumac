# Testing Patterns

**Analysis Date:** 2026-04-04

## Test Framework

**Runner:**
- Rust tests run through `cargo test`; there is no custom `nextest`, `llvm-cov`, or alternate harness config file.
- Config lives in `Cargo.toml` through `[dev-dependencies]` and the default Cargo test runner.
- CI executes `cargo test --locked --all-targets` in `.github/workflows/ci.yml`.

**Assertion Library:**
- Standard Rust assertions: `assert!`, `assert_eq!`, and `assert_ne!`.
- CLI assertions: `assert_cmd` plus `predicates`, wired through `Cargo.toml` and used heavily in `tests/cli_integration.rs`.

**Run Commands:**
```bash
cargo test                              # Default Rust suite
cargo test --lib                        # Unit tests in `src/**`
cargo test --test cli_integration       # Large CLI integration suite
cargo test test_scan_malicious_skill    # Single test by name
cargo test --locked --all-targets       # CI lane in `.github/workflows/ci.yml`
bash scripts/smoke_portable_core.sh target/debug/sxmc .
bash scripts/smoke_portable_fixtures.sh target/debug/sxmc tests/fixtures
SXMC=target/debug/sxmc bash scripts/test-sxmc.sh --json /tmp/sxmc-test-results.json
```

## Test File Organization

**Location:**
- Keep unit tests colocated with the code under `#[cfg(test)]`, for example in `src/auth/secrets.rs`, `src/skills/parser.rs`, `src/server/mod.rs`, `src/output/mod.rs`, and `src/security/patterns.rs`.
- Put command-level and end-to-end Rust integration coverage in `tests/cli_integration.rs`.
- Store portable fixtures in `tests/fixtures/`, including skill directories like `tests/fixtures/simple-skill/` and the synthetic server `tests/fixtures/stateful_mcp_server.py`.
- Keep shell-based smoke and release validation in `scripts/`, especially `scripts/smoke_portable_core.sh`, `scripts/smoke_portable_fixtures.sh`, `scripts/smoke_test_clients.sh`, and `scripts/test-sxmc.sh`.

**Naming:**
- Rust test functions use `test_<behavior>` naming across both unit and integration tests.
- The main integration suite is a single file, `tests/cli_integration.rs`, instead of many smaller files.
- Smoke scripts use `smoke_*` prefixes; the broad shell suite is `scripts/test-sxmc.sh`.

**Structure:**
```text
src/<module>.rs            # unit tests in #[cfg(test)] mod tests
tests/cli_integration.rs   # binary-level integration coverage
tests/fixtures/            # deterministic local fixtures
scripts/smoke_*.sh         # portable smoke paths
scripts/test-sxmc.sh       # expansive regression + benchmark suite
```

## Test Structure

**Suite Organization:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_argument_hint_angle_brackets() {
        let args = parse_argument_hint("<repo> [--dry-run]");
        assert_eq!(args[0].name, "repo");
        assert!(args[0].required);
    }
}
```

**Patterns:**
- Put shared helpers at the top of `tests/cli_integration.rs`, for example `sxmc()`, `sxmc_bin_string()`, `spawn_http_server()`, `wait_for_http_server()`, and `command_json_with_config_home()`.
- Use `tempfile::tempdir()` or `TempDir::new()` for isolated filesystem state in both unit and integration tests, as seen in `src/skills/parser.rs`, `src/server/handler.rs`, `src/cache.rs`, and throughout `tests/cli_integration.rs`.
- For CLI behavior, prefer fluent `assert_cmd` chains for exit status and stdout/stderr matching:

```rust
sxmc()
    .arg("--help")
    .assert()
    .success()
    .stdout(predicate::str::contains("Sumac"))
    .stdout(predicate::str::contains("serve"));
```

- For structured output, capture stdout and deserialize JSON before making field-level assertions, as in `tests/cli_integration.rs` and `src/output/mod.rs` tests.
- When background processes are spawned, tests explicitly clean them up with `child.kill()` plus `child.wait()`, as in the HTTP server tests in `tests/cli_integration.rs`.

## Mocking

**Framework:** No dedicated mocking framework

**Patterns:**
```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
let addr = listener.local_addr().unwrap();
let handle = tokio::spawn(async move {
    let app = Router::new().route("/openapi.json", get(move || async move { Json(spec) }));
    let _ = axum::serve(listener, app).await;
});
```

**What to Mock:**
- Replace remote HTTP dependencies with in-process `axum` servers, as in `tests/cli_integration.rs` and `src/server/mod.rs`.
- Replace third-party CLIs with temporary wrapper scripts or fake executables, as in helper functions near the top of `tests/cli_integration.rs`.
- Use local fixtures for skills and MCP servers from `tests/fixtures/` instead of reaching across the network.

**What NOT to Mock:**
- Do not mock the `sxmc` binary interface when testing command behavior; integration tests invoke the compiled binary with `assert_cmd` or `std::process::Command`.
- Do not stub away stdio/HTTP flows that are central to the product contract; the current suite exercises real subprocesses, listeners, and JSON payloads.

## Fixtures and Factories

**Test Data:**
```rust
let temp = tempfile::tempdir().unwrap();
fs::write(temp.path().join("AGENTS.md"), "# Existing\n").unwrap();

let value = command_json_with_config_home(
    temp.path(),
    &["doctor", "--root", temp.path().to_str().unwrap()],
);
assert_eq!(value["startup_files"]["portable_agent_doc"]["present"], true);
```

**Location:**
- Static fixtures live in `tests/fixtures/`; see `tests/fixtures/README.md` for their purpose.
- Dynamic fixtures are created inline with `tempfile`, `fs::write`, and helper constructors in `tests/cli_integration.rs`.
- Cross-platform smoke scripts also create their own temporary workspaces with `mktemp -d`, as in `scripts/smoke_portable_core.sh` and `scripts/smoke_portable_fixtures.sh`.

## Coverage

**Requirements:** No percentage threshold is enforced.

**View Coverage:**
```bash
cargo test
bash scripts/certify_release.sh target/debug/sxmc tests/fixtures
```

- The repo treats coverage as layered validation rather than a numeric coverage report.
- The enforced gates are formatting, Clippy, Cargo tests, smoke scripts, packaging checks, and the large shell suite. See `.github/workflows/ci.yml`, `scripts/certify_release.sh`, and `docs/VALIDATION.md`.
- No `cargo llvm-cov`, `tarpaulin`, `grcov`, or similar tool is configured.

## Test Types

**Unit Tests:**
- Small logic tests sit next to the implementation in `src/**`.
- These cover parsers, scanners, serializers, caches, path handling, and schema formatting, for example `src/skills/parser.rs`, `src/security/patterns.rs`, `src/client/openapi.rs`, `src/client/graphql.rs`, and `src/cache.rs`.

**Integration Tests:**
- `tests/cli_integration.rs` is the primary Rust integration surface.
- It covers CLI UX, structured JSON output, HTTP and stdio transports, bake flows, doctor/status behavior, wrapping, API mode, and cross-feature regressions.
- This file is intentionally broad and uses helper functions plus fixtures to keep repeated setup centralized.

**E2E Tests:**
- `scripts/smoke_portable_core.sh` validates the stable cross-platform basics.
- `scripts/smoke_portable_fixtures.sh` validates local skill serving plus stdio/HTTP/baked MCP flows.
- `scripts/test-sxmc.sh` is the expansive regression and benchmark suite documented in `docs/TEST_SCRIPT_GUIDE.md`.
- `scripts/certify_release.sh` is the release gate that chains Rust checks, smoke scripts, packaging checks, and the full shell suite.

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn test_http_health_endpoint_reports_auth_mode() {
    let (mut child, port) = spawn_http_server(&[
        "--bearer-token",
        "health-token",
        "--paths",
        "tests/fixtures",
    ]);

    let response: serde_json::Value = reqwest::get(format!("http://127.0.0.1:{port}/healthz"))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(response["status"], "ok");
    let _ = child.kill();
    let _ = child.wait();
}
```

**Error Testing:**
```rust
let output = ProcessCommand::new(sxmc_bin_string())
    .args(["stdio", &spec, "hello", "name=Sam", "--pretty"])
    .output()
    .unwrap();
assert!(!output.status.success());
assert!(String::from_utf8_lossy(&output.stderr).contains("Unknown argument 'name'"));
```

## Practical Rules To Follow

- Add unit tests next to new library code first; only extend `tests/cli_integration.rs` when the change affects binary behavior or end-to-end flows.
- Reuse `tests/fixtures/` for stable local scenarios before inventing new network-dependent cases.
- Prefer real subprocesses, temp directories, and local HTTP listeners over mocks.
- When a feature touches the stable workflow, add or update shell validation in `scripts/smoke_portable_core.sh`, `scripts/smoke_portable_fixtures.sh`, or `scripts/test-sxmc.sh`.
- Preserve cross-platform assumptions: many tests and smoke scripts are written to work on Linux, macOS, and Windows CI.

---

*Testing analysis: 2026-04-04*
