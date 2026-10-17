---
id: TC-007
title: "Verify cross-backend semantic parity"
type: TC
relationships:
  - target: ix://agent-ix/quire-contract-codegen/FR-005
    type: verifies
---
# TC-007: Verify cross-backend semantic parity

## Description

Verify executable, proptest, Kani, and coverage lowerings agree on the shared bounded corpus and every
permitted differential fixture has an attributed disposition.

## Test Procedure

Execute the same canonical cases through each backend and compare normalized clause outcomes,
diagnostics, and dependencies with the expected outcomes and permitted attributed fixtures. For
the Kani slice, compare the exact embedded executable-oracle predicates and enumerate the bounded
Boolean/integer corpus, including each inclusive model endpoint and immediately outside values.

## Differential Disposition Register

The register is this table. Each case on which two backends' normalized outcomes differ has exactly
one row: the case identifier, the backend pair, and the disposition, which is either a regression
fixture that exists in the test tree or a documented profile/backend distinction. FR-005-AC-3 fails
when a differing case has no row, when a row names a fixture that does not exist, or when a row names
neither disposition. No row is recorded at this revision: TC-007-tagged tests run corpus slices and assert
agreement, but no full differential run produces a list of differing cases and no check joins that
list against this register; the first such run's differences are the first rows.

| Case | Backends | Disposition | Fixture or distinction |
|------|----------|-------------|------------------------|

## Expected Results

Shared semantics agree exactly; each difference is retained as a regression fixture or documented
profile/backend distinction rather than suppressed. Kani quantifies only over IR-domain values;
outside-domain executable-oracle results remain observable comparison data and are not silently
reclassified as proof inputs.
