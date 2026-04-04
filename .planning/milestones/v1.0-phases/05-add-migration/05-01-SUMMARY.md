# Summary 05-01: Migrate `add` onto the Shared Core/App Seam and Prove Parity

## Outcome

`add` is now a dedicated migrated onboarding slice under `src/app/add.rs`,
while the reusable host-selection and inspect/materialize behavior lives in
`src/app/onboarding.rs` instead of staying embedded in
`src/app/golden_path.rs`.

## What Changed

- added `src/app/add.rs`
- added `src/app/onboarding.rs`
- moved `AddRequest` and add execution into `AddService`
- promoted shared onboarding selection and materialization into
  `OnboardingService`
- updated `GoldenPathApp` to delegate `add` directly to the dedicated service
- rewired `setup` core-path onboarding to consume the shared onboarding
  service, preparing the final consolidation phase

## Parity Proof Added

- direct integration coverage for legacy-route `add`
- direct core-vs-legacy parity coverage comparing the shipped structured
  contract fields for `sxmc add`
- full shell-suite proof that the add pipeline still preserves configured-host
  detection, preview fallback, structured output, and global install-scope
  behavior

This makes `add` the second onboarding-oriented slice to move behind the new
core/app seam and establishes the reusable service that Phase 6 can use to
finish the golden path.
