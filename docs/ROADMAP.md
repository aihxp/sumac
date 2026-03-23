# Roadmap

This document captures the next major product evolutions for `sxmc` now that
the core CLI/MCP/API/configuration pipeline is stable and well covered by tests.

## Principles

- Preserve the single-binary, low-dependency model.
- Prefer features that compound the value of the existing profile corpus.
- Keep `sxmc` host-neutral so it remains useful across Claude Code, Cursor,
  Copilot, Gemini CLI, Codex, and future AI hosts.
- Treat generated tool knowledge as something that must stay current, not as a
  one-time scaffold.

## Phase 1: Continuous Maintenance

Goal: keep generated knowledge fresh as tools evolve.

Problems solved:

- Tool knowledge drift after CLI upgrades
- Stale AI startup docs and configs
- No easy way to see what changed between saved and live profiles at scale

Candidate deliverables:

- `sxmc status`
  - report profiled tools, stale profiles, baked MCP servers, startup-file health
- `sxmc inspect drift`
  - summarize which cached/saved profiles no longer match the installed tools
- `sxmc inspect diff --exit-code` adoption in CI templates
  - prebuilt examples for blocking changes when tool surfaces drift
- `sxmc watch`
  - monitor profiled binaries and regenerate selected artifacts when they change

Suggested order:

1. `status`
2. drift reports over saved/batch profiles
3. CI recipes and templates
4. long-running watch/daemon mode

## Phase 2: Zero-Config MCP Wrapping

Goal: make existing CLIs MCP-usable with almost no manual glue.

Problems solved:

- MCP adoption friction for teams with existing CLIs
- Repeated hand-written wrappers around command-line tools
- Missing input validation and schema generation for generated wrappers

Candidate deliverables:

- `sxmc wrap <tool>`
  - inspect a CLI and start a generated MCP wrapper immediately
- generated tool schemas from profile options/subcommands
- safer execution envelopes
  - timeout policies
  - stdout/stderr conventions
  - argument allowlists/denylists
- progress/event streaming for long-running wrapped commands

Suggested order:

1. wrapper artifact generation improvements
2. one-command wrapper launch
3. schema tightening and permission boundaries
4. streaming/progress support

## Phase 3: Team Profile Distribution

Goal: make `sxmc` useful for groups, not just one workstation.

Problems solved:

- Repeated local regeneration of the same internal CLI profiles
- No team-blessed source of truth for AI-visible tool knowledge
- No clean way to roll updated profiles out across multiple AI hosts

Candidate deliverables:

- profile bundle export/import
- `sxmc publish` / `sxmc pull`
  - Git/S3/object-store-backed profile distribution
- host-scoped or role-scoped profile sets
  - backend/frontend/platform bundles
- signed or verified profile bundles

Suggested order:

1. bundle format
2. local import/export
3. remote sync
4. role-based bundles and verification

## Phase 4: Unified Capability Status

Goal: show what an AI environment can actually do right now.

Problems solved:

- No single view of which tools/APIs/MCP servers are configured per host
- Hard to answer “is this environment ready for agent work?”
- No observability for broken or stale integrations

Candidate deliverables:

- richer `sxmc doctor` / `sxmc status`
  - per-host capability summaries
  - stale/missing config signals
  - MCP reachability and health
- diffing across hosts
  - “Cursor has this tool surface, Copilot does not”
- API/MCP health checks

Suggested order:

1. richer local status
2. host-to-host capability diff
3. MCP/API health panels

## Phase 5: Shared Profile Intelligence

Goal: turn individual inspections into a durable product moat.

Problems solved:

- Every machine rediscovering the same tool surfaces
- No feedback loop around parser quality or high-value tools
- No way to prioritize parser improvements with real-world evidence

Candidate deliverables:

- opt-in profile telemetry or anonymous profile fingerprints
- verified/community-curated profile sets
- profile quality scoring and freshness metadata
- “known-good profile for tool X version Y” support

Suggested order:

1. local quality/freshness metadata
2. optional export-friendly corpus tooling
3. opt-in aggregation or registry-backed verification

## Near-Term Priorities

If only the next few roadmap items are tackled, the highest-leverage sequence is:

1. `sxmc status` with stale-profile detection
2. CI/drift templates built on the current diff machinery
3. zero-config `sxmc wrap <tool>` MVP
4. profile bundle export/import for teams

## Success Criteria

`sxmc` should eventually make these statements true:

- “Our AI configs stay correct when our tools change.”
- “Any internal CLI can become AI-usable in minutes.”
- “Every engineer on the team gets the same tool knowledge across AI hosts.”
- “We can see what our AI environment can access and whether it is healthy.”
