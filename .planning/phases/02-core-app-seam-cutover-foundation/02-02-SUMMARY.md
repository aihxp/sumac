# Summary 02-02: Define Typed Contracts and Explicit Adapters

## Outcome

The new seam now uses typed request/result structs and explicit adapter objects
for the golden path instead of keeping orchestration as ad hoc local match-arm
logic.

## Typed Contracts Added

- `AddRequest`
- `SetupRequest`
- `StatusRequest`
- `SyncRequest`
- `CommandOutcome`

## Explicit Adapter Boundary

`GoldenPathAdapters` now centralizes the legacy infrastructure hooks used by
the new seam:

- host auto-detection
- tool auto-detection
- CLI inspection and readiness gating
- artifact resolution and materialization
- status collection
- sync collection

This gives later phases one place to replace legacy behavior slice by slice
without changing the stable command-entry seam.
