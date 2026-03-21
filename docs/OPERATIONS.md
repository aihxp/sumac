# Operations Guide

This guide consolidates deployment, release, and distribution notes.

## Hosted MCP

Serve a remote MCP endpoint:

```bash
sxmc serve --transport http --host 0.0.0.0 --port 8000 \
  --bearer-token env:SXMC_MCP_TOKEN \
  --paths /absolute/path/to/skills
```

Or require an exact header:

```bash
sxmc serve --transport http --host 0.0.0.0 --port 8000 \
  --require-header "X-Internal-Token: secret-value" \
  --paths /absolute/path/to/skills
```

Key operational endpoints:

- MCP endpoint: `/mcp`
- health check: `/healthz`

Recommended default for hosted deployments:

- prefer `--bearer-token env:...` for a single shared token
- use `--require-header` for stricter internal or proxy-based setups
- keep the service behind a reverse proxy for TLS and access control

## Release Process

Before a release:

1. run the validation commands in [`VALIDATION.md`](VALIDATION.md)
2. update `CHANGELOG.md`
3. confirm `Cargo.toml` and packaging metadata are aligned
4. confirm `README.md` matches the current public surface

Release steps:

```bash
git tag vX.Y.Z
git push origin master --tags
cargo publish
```

GitHub Actions will build the release archives and checksums from the pushed
tag.

## Distribution

Current distribution channels:

- crates.io
- GitHub Releases
- repo-local npm wrapper metadata in `packaging/npm`
- repo-local Homebrew formula in `packaging/homebrew/sxmc.rb`

The canonical install path remains:

```bash
cargo install sxmc
```

## Maintenance Notes

- keep `master` as the canonical branch
- prefer `sxmc mcp` as the primary daily MCP client UX
- keep `sxmc stdio` and `sxmc http` as the lower-level raw bridge/debug layer
- keep docs focused on stable product paths rather than release-by-release notes
