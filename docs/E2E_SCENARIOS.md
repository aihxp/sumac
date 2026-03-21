# E2E Scenarios

This document tracks 25 high-value end-to-end scenarios for `sxmc`.

Use it together with:

- [PRODUCT_CONTRACT.md](PRODUCT_CONTRACT.md) for what we actually claim
- [SMOKE_TESTS.md](SMOKE_TESTS.md) for repeatable smoke procedures
- [COMPATIBILITY_MATRIX.md](COMPATIBILITY_MATRIX.md) for dated client/server validation

Coverage status meanings:

- `Automated`: covered by `cargo test` / `tests/cli_integration.rs`
- `Certified`: exercised by `scripts/certify_release.sh`
- `Manual`: still worth spot-checking by hand for a release or client-specific flow

| # | Scenario | Product area | Coverage |
|---|----------|--------------|----------|
| 1 | `skills list` against fixture skills | Skills | Automated, Certified |
| 2 | `skills info` for an existing skill | Skills | Automated, Certified |
| 3 | `skills info` for a missing skill | Skills | Automated |
| 4 | `skills run` for a prompt-only skill | Skills | Automated, Certified |
| 5 | `skills create` from a local OpenAPI spec | Skills / API | Automated |
| 6 | `scan` on a clean skill | Security | Automated, Certified |
| 7 | `scan` on a malicious skill | Security | Automated, Certified |
| 8 | `scan --json` output shape | Security | Automated |
| 9 | `scan --severity critical` filtering and exit behavior | Security | Automated |
| 10 | `serve` over stdio with fixture skills | Skills -> MCP | Certified |
| 11 | `stdio --list` against `sxmc serve` | MCP -> CLI | Automated, Certified |
| 12 | `stdio --list-prompts` | MCP -> CLI | Automated |
| 13 | `stdio --resource <uri>` | MCP -> CLI | Automated |
| 14 | `stdio --prompt <name>` | MCP -> CLI | Automated |
| 15 | `stdio` tool call for a script-backed skill | MCP -> CLI | Automated, Certified |
| 16 | `stdio` hybrid retrieval tool call | MCP -> CLI | Automated |
| 17 | `serve --transport http` and `http --list` | Hosted MCP | Automated, Certified |
| 18 | `http --list-resources` | Hosted MCP | Automated |
| 19 | `http --prompt <name>` | Hosted MCP | Automated |
| 20 | `http --resource <uri>` | Hosted MCP | Automated |
| 21 | `http` with required-header auth | Hosted MCP / Auth | Automated, Certified |
| 22 | `http` with bearer-token auth | Hosted MCP / Auth | Automated, Certified |
| 23 | `/healthz` inventory and auth reporting | Hosted MCP / Ops | Automated, Certified |
| 24 | `serve --watch` reload after editing `SKILL.md` | Hosted MCP / Watch | Automated |
| 25 | Local GraphQL list and call, plus local OpenAPI autodetect list and call | API -> CLI | Automated |

## Notes

- Scenario 10 is release-certified through the smoke/certification scripts even
  though the exact `serve` startup path is not represented by one standalone
  integration assertion.
- Scenario 16 covers the hybrid retrieval tools like `get_skill_details` and
  `get_skill_related_file`.
- Scenario 25 intentionally groups the two local API-server paths that had
  previously been called out as edge-expansion gaps:
  - `sxmc api` autodetecting and calling a local OpenAPI endpoint
  - `sxmc graphql` listing and calling a local GraphQL endpoint

## Release Use

Before tagging a release, the goal should be:

1. all `Automated` scenarios green in CI
2. all `Certified` scenarios green in `scripts/certify_release.sh`
3. any remaining `Manual` scenarios justified as client-specific rather than core product gaps

If a future release adds a new claimed product path, add it here instead of
letting the checklist drift into ad hoc notes.
