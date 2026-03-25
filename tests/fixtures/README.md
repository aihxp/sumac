# Test Fixtures

These fixtures are the portable local validation surfaces used by Sumac’s smoke
and release-certification flows.

Current fixture set:

- `simple-skill`
  - minimal skill for listing, info, prompt rendering, and interpolation
- `malicious-skill`
  - intentionally unsafe sample for scanner validation
- `skill-with-references`
  - skill with `references/` content for MCP resource coverage
- `skill-with-scripts`
  - skill with executable script tools for `skills run` and MCP tool execution
- `stateful_mcp_server.py`
  - synthetic MCP fixture used by bake/session coverage

These fixtures are intentionally:

- local-first
- deterministic
- cross-platform friendly
- small enough for CI smoke and release certification

For an inventory view, run:

```bash
python3 scripts/fixture_inventory.py tests/fixtures
```
