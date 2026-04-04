# Phase 4: Sync Migration - Research

## Why `sync` Is the Right Next Slice

After `status`, `sync` is the cleanest next proof point because it is:

- still part of the golden path
- mostly deterministic
- already shaped as structured output
- the first slice that proves state mutation and artifact refresh through the
  new seam

## Best Migration Shape

Mirror the Phase 3 pattern:

- add `src/app/sync.rs`
- move `SyncRequest` and sync execution there
- keep the current sync value builder as an adapter initially
- add direct parity proof between default/core and explicit legacy routing

This keeps the rewrite incremental while proving the seam can handle write-side
flows, not just read-only ones.
