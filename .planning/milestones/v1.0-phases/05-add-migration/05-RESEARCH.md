# Phase 5: Add Migration - Research

## Why `add` Comes Before `setup`

`add` is the cleaner onboarding slice because it handles one tool at a time.
That makes it the right place to promote shared onboarding logic into a real
service before `setup` starts orchestrating multiple tools over the same
service.

## Best Migration Shape

Phase 5 should extract two things:

1. a reusable onboarding service
2. an `add`-specific service that depends on it

Recommended modules:

- `src/app/onboarding.rs`
- `src/app/add.rs`

This lets Phase 6 rebuild `setup` on the exact same onboarding primitive
instead of keeping parallel orchestration.
